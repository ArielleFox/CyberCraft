use anyhow::{Context, Result};
use tokio::process::Command;
use std::path::{Path};

pub async fn run_file_encryption(dir: &Path, yubi_key_identity: &str) -> Result<()> {
    let identity_path = dirs::home_dir()
        .context("Failed to get home directory")?
        .join(format!(".yubiCrypt/keys/{}", yubi_key_identity));

    // Check if the YubiKey identity file exists
    if !identity_path.is_file() {
        return Err(anyhow::anyhow!("Identity file not found: {}", identity_path.display()).into());
    }

    // Encrypt files in the directory (you can expand this logic to encrypt multiple files)
    let encrypted_file = format!("{}.age", dir.display());
    let status = Command::new("age")
        .arg("-r")
        .arg(identity_path)
        .arg("-o")
        .arg(&encrypted_file)
        .arg(dir)
        .spawn()?
        .wait()
        .await
        .context("Failed to execute age command")?;

    if status.success() {
        println!("✅ Successfully encrypted: {}", dir.display());
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to encrypt: {}", dir.display()).into())
    }
}
