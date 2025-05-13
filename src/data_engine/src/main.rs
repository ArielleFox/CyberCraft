mod command_runner;  // Import the command_runner module

use tokio::fs;
use tokio::io;
use std::path::{Path};
use std::env;
use serde::{Deserialize};

const CY_CONFIG_FILE: &str = ".cy_config.yaml";
const PRE_COMMIT_FILE: &str = ".pre-commit-config.yaml";

#[derive(Debug, Deserialize)]
struct Config {
    methode: Option<String>,
    keyname: Option<String>,
}

/// Checks for .cy_config.yaml and .pre-commit-config.yaml in current and parent dirs
pub async fn list_all_folder() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let mut found_config = false;
    let mut found_pre_commit = false;

    // Traverse upward from the current directory
    let mut dir_opt = Some(current_dir.as_path());

    while let Some(dir) = dir_opt {
        println!("\n🔍 Searching in directory: {}", dir.display());
        match command_runner::run_command("ls", &["", &dir.display().to_string()]) {
            Ok(output) => {
                // Print the command's output (for demonstration)
                if !output.stdout.is_empty() {
                    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(e) => {
                eprintln!("❌ Error running command: {}", e);
            }
        }

        // Check for .cy_config.yaml
        let config_path = dir.join(CY_CONFIG_FILE);
        if config_path.exists() && config_path.is_file() && !found_config {
            println!("✅ Found config file: {}", config_path.display());
            let contents = fs::read_to_string(&config_path).await?;
            println!("📄 .cy_config.yaml contents:\n{}", contents);

            // Parse config
            match serde_yaml::from_str::<Config>(&contents) {
                Ok(config) => {
                    found_config = true;
                    match config.methode.as_deref() {
                        Some("yubikey") => {
                            println!("🔐 Methode is 'yubikey' — perform YubiKey setup.");
                            // Convert dir.display() to a String before passing to run_command
                            match command_runner::run_command("ls", &["", &dir.display().to_string()]) {
                                Ok(output) => {
                                    // Print the command's output (for demonstration)
                                    if !output.stdout.is_empty() {
                                        println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
                                    }
                                }
                                Err(e) => {
                                    eprintln!("❌ Error running command: {}", e);
                                }
                            }
                        }
                        Some("none") => {
                            println!("⚙️ Methode is 'none' — perform fallback logic.");
                        }
                        Some("gpg") => {
                            if let Some(key) = config.keyname {
                                let keyname = key;
                                println!("🔑 Methode is 'gpg' — using GPG keyname: {}", keyname);
                                // You can now use the `keyname` variable as needed
                            } else {
                                println!("⚠️ 'gpg' methode specified, but no keyname found.");
                            }
                        }
                        Some(other) => {
                            println!("❓ Unknown methode: {}", other);
                        }
                        None => {
                            println!("⚠️ No 'methode' field found.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse .cy_config.yaml: {}", e);
                }
            }
        }

        // Check for .pre-commit-config.yaml
        let pre_commit_path = dir.join(PRE_COMMIT_FILE);
        if pre_commit_path.exists() && pre_commit_path.is_file() && !found_pre_commit {
            println!("✅ Found .pre-commit-config.yaml: {}", pre_commit_path.display());
            let contents = fs::read_to_string(&pre_commit_path).await?;
            println!("📄 .pre-commit-config.yaml contents:\n{}", contents);
            found_pre_commit = true;
        }

        // Exit early if both files are found
        if found_config && found_pre_commit {
            break;
        }

        dir_opt = dir.parent();
    }

    if !found_config {
        println!("❌ No .cy_config.yaml file found.");
    }

    if !found_pre_commit {
        println!("❌ No .pre-commit-config.yaml file found.");
    }

    Ok(())
}


/// Finds how many layers up `.git` exists
fn find_git_layer_up(start_dir: &Path) -> io::Result<Option<u32>> {
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

/// Reports location of `.git` directory, if found
pub async fn list_folder() -> io::Result<()> {
    let current_dir = env::current_dir()?;

    match find_git_layer_up(&current_dir)? {
        Some(layers) => println!("✅ .git found at layer level: {}", layers),
        None => println!("❌ .git not found in any parent directories."),
    }

    Ok(())
}

/// Main entry point
#[tokio::main]
async fn main() {
    let all_result = list_all_folder().await;
    let git_result = list_folder().await;

    match all_result {
        Ok(_) => println!("✅ Config and hook check completed."),
        Err(e) => eprintln!("❌ Error during config/hook check: {}", e),
    }

    match git_result {
        Ok(_) => println!("✅ Git check completed."),
        Err(e) => eprintln!("❌ Error checking for .git: {}", e),
    }
}
