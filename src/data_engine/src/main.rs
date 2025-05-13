mod encrypt;
mod decrypt;

use clap::{Arg, Command};
use std::{env::current_dir, path::Path};
use serde::Deserialize;
use tokio::fs;
use anyhow::{Result};
use log::{info, error, warn};

const CY_CONFIG_FILE: &str = ".cy_config.yaml";
const PRE_COMMIT_FILE: &str = ".pre-commit-config.yaml";

#[derive(Debug, Deserialize)]
struct Config {
    methode: Option<String>,
    keyname: Option<String>,
}

// Encrypt function
async fn encrypt() -> Result<()> {
    let current_dir = current_dir()?;
    info!("🔐 Methode: yubikey — Encrypting files...");
    encrypt::run_file_encryption(&current_dir, "first.txt").await?;
    Ok(())
}

// Decrypt function
async fn decrypt(current_dir: &std::path::Path) -> Result<()> {
    info!("🔓 Methode: decrypt — Decrypting files...");
    decrypt::run_file_decryption(current_dir, "first.txt").await?;
    Ok(())
}


// Verify crypt mode
async fn verify_crypt(mode: &str) {
    if mode == "encrypt" {
        if let Err(e) = encrypt().await {
            error!("❌ Error during encryption: {}", e);
        }
    } else if mode == "decrypt" {
        let current_dir = current_dir().unwrap(); // Get current directory to pass to decrypt
        if let Err(e) = decrypt(&current_dir).await {
            error!("❌ Error during decryption: {}", e);
        }
    }
}


// Recursively search upward for config and execute logic
async fn list_all_folder(mode: &String) -> Result<()> {
    let current_dir = std::env::current_dir()?;
   let mut found_config = false;
    let mut found_pre_commit = false;

    let mut dir_opt = Some(current_dir.as_path());

    while let Some(dir) = dir_opt {
        info!("🔍 Searching in: {}", dir.display());

        // Check .cy_config.yaml
        let config_path = dir.join(CY_CONFIG_FILE);
        if config_path.exists() && config_path.is_file() && !found_config {
            info!("✅ Found config: {}", config_path.display());

            let contents = fs::read_to_string(&config_path).await?;
            info!("📄 .cy_config.yaml contents:\n{}", contents);

            match serde_yaml::from_str::<Config>(&contents) {
                Ok(config) => {
                    found_config = true;
                    match config.methode.as_deref() {
                        Some("yubikey") => {
                            verify_crypt(mode).await;
                        }
                        Some("none") => {
                            info!("⚙️ Methode: none — no action taken.");
                        }
                        Some("gpg") => {
                            warn!("🛑 Methode: gpg — not implemented yet.");
                        }
                        Some(other) => {
                            warn!("❓ Unknown methode: {}", other);
                        }
                        None => {
                            warn!("⚠️ No 'methode' field specified.");
                        }
                    }
                }
                Err(e) => {
                    error!("❌ Failed to parse config: {}", e);
                }
            }
        }

        // Check .pre-commit-config.yaml
        let pre_commit_path = dir.join(PRE_COMMIT_FILE);
        if pre_commit_path.exists() && pre_commit_path.is_file() && !found_pre_commit {
            info!("✅ Found pre-commit config: {}", pre_commit_path.display());
            let contents = fs::read_to_string(&pre_commit_path).await?;
            info!("📄 .pre-commit-config.yaml contents:\n{}", contents);
            found_pre_commit = true;
        }

        // Exit early if both found
        if found_config && found_pre_commit {
            break;
        }

        dir_opt = dir.parent();
    }

    if !found_config {
        error!("❌ No .cy_config.yaml file found.");
    }

    if !found_pre_commit {
        error!("❌ No .pre-commit-config.yaml file found.");
    }

    Ok(())
}

// Detects `.git` directory upward and prints level
fn find_git_layer_up(start_dir: &Path) -> std::io::Result<Option<u32>> {
    let mut current_dir = start_dir.to_path_buf();
    let mut layers_up = 0;

    loop {
        let git_dir = current_dir.join(".git");

        if git_dir.exists() && git_dir.is_dir() {
            return Ok(Some(layers_up));
        }

        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
            layers_up += 1;
        } else {
            break;
        }
    }

    Ok(None)
}

// Reports .git location if found
pub async fn list_folder() -> Result<()> {
    let current_dir = std::env::current_dir()?;

    match find_git_layer_up(&current_dir)? {
        Some(layers) => info!("✅ .git found at layer level: {}", layers),
        None => error!("❌ .git not found in any parent directories."),
    }

    Ok(())
}

// Entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Command-line argument parsing
    let matches = Command::new("CyberCraft Rust Rewrite")
    .version("1.0")
    .about("Encrypt or Decrypt files")
    .arg(
        Arg::new("encrypt")
        .long("encrypt")
        .action(clap::ArgAction::SetTrue)
        .help("Encrypt the files in the directory"),
    )
    .arg(
        Arg::new("decrypt")
        .long("decrypt")
        .action(clap::ArgAction::SetTrue)
        .help("Decrypt the files in the directory"),
    )
    .get_matches();

    // Ensure at least one of encrypt or decrypt is present
    if matches.get_one::<bool>("encrypt").is_none() && matches.get_one::<bool>("decrypt").is_none() {
        return Err("You must specify either 'encrypt' or 'decrypt'.".into());
    }
    let mode = if matches.get_one::<bool>("encrypt").is_some() {
        "encrypt".to_string()
    } else {
        "decrypt".to_string()
    };

     // Determine mode

    // Perform config and hook checks
    let all_result = list_all_folder(&mode).await;
    let git_result = list_folder().await;

    match all_result {
        Ok(_) => info!("✅ Config and hook check completed."),
        Err(e) => error!("❌ Error in config/hook check: {}", e),
    }

    match git_result {
        Ok(_) => info!("✅ Git check completed."),
        Err(e) => error!("❌ Error checking .git: {}", e),
    }

    Ok(())

}
