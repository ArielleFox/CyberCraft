use anyhow::{Context, Result};
use tokio::process::Command;
use std::path::{Path};

pub async fn run_file_decryption(dir: &Path, yubi_key_identity: &str) -> Result<()> {
    let identity_path = dirs::home_dir()
        .context("Failed to get home directory")?
        .join(format!(".yubiCrypt/keys/{}", yubi_key_identity));

    // Check if the YubiKey identity file exists
    if !identity_path.is_file() {
        return Err(anyhow::anyhow!("Identity file not found: {}", identity_path.display()).into());
    }

    // Decrypt files in the directory (you can expand this logic to decrypt multiple files)
    let decrypted_file = dir.with_extension(""); // Remove ".age" extension
    let status = Command::new("age")
        .arg("-d")
        .arg("-i")
        .arg(identity_path)
        .arg("-o")
        .arg(&decrypted_file)
        .arg(dir)
        .spawn()?
        .wait()
        .await
        .context("Failed to execute age command")?;

    if status.success() {
        println!("✅ Successfully decrypted: {}", dir.display());
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to decrypt: {}", dir.display()).into())
    }
}
