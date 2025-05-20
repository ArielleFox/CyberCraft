use std::path::{Path};
use std::process::{Command, Stdio};
use walkdir::WalkDir;
use shellexpand;
use anyhow::{Result, Context};
use log::{info, error};

/// Check if a file should be processed (not hidden, is a regular file)
fn is_valid_target(file_path: &Path) -> bool {
    if let Some(name) = file_path.file_name().and_then(|n| n.to_str()) {
        if name.starts_with('.') || !name.ends_with(".age") {
            return false; // Skip non `.age` files and hidden files
        }
    }

    file_path.is_file() // Only process regular files
}

pub async fn run_file_decryption(folder_path: &Path, yubi_key_identity: &str) -> Result<()> {
    // Expand user path for YubiKey identity
    let identity_raw = format!("~/.yubiCrypt/keys/{}", yubi_key_identity);
    let identity_path = shellexpand::tilde(&identity_raw).to_string();

    info!("🔓 Decrypting files in: {}", folder_path.display());

    for entry in WalkDir::new(folder_path)
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.path().to_path_buf()) {

            if !is_valid_target(&entry) {
                continue; // Skip invalid files
            }

            let input_path = entry.to_str().unwrap();
            let output_path = input_path.trim_end_matches(".age"); // Remove `.age` extension for output file

            info!("🔍 Decrypting: {}", input_path);

            // Decrypt the file using `rage`
            let status = Command::new("rage")
            .arg("-d")
            .arg("-i")
            .arg(&identity_path)  // Use the YubiKey identity path
            .arg("-o")
            .arg(&output_path)  // Output decrypted file
            .arg(&input_path)  // Input encrypted file
            .stdin(Stdio::null()) // Don't pass anything to stdin
            .stdout(Stdio::piped()) // Capture stdout
            .stderr(Stdio::piped()) // Capture stderr
            .spawn()
            .context(format!("Failed to spawn `rage` for file: {}", input_path))?
            .wait()
            .context(format!("Failed to wait for `rage` process to finish for file: {}", input_path))?;

            if status.success() {
                info!("✅ Decrypted: {}", input_path);
                let _ = std::fs::remove_file(input_path); // Remove `.age` file after decryption
            } else {
                error!("❌ Decryption failed: {}", input_path);
            }
        }

        Ok(()) // Return Ok when done
}
