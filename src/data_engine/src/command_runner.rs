use tokio::process::Command;
use tokio::task;
use std::path::{PathBuf};
use std::ffi::OsStr;
use std::process::Stdio;
use anyhow::{Context, Result};
use walkdir::{WalkDir, DirEntry};
use futures::future::join_all;

/// Check if a file should be processed (not hidden, is a regular file)
fn is_valid_file(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    if file_name.starts_with('.') {
        return false;
    }
    entry.file_type().is_file()
}

/// Asynchronously run encrypt/decrypt script for a single file
async fn process_file(path: PathBuf) -> Result<()> {
    let script = if path.extension() == Some(OsStr::new("age")) {
        "~/dcde/src/decrypt.py"
    } else {
        "~/dcde/src/encrypt.py"
    };

    let script_path = shellexpand::tilde(script);
    println!("▶️ Processing: {}", path.display());

    let status = Command::new("python3")
    .arg(script_path.to_string())
    .arg(path.to_string_lossy().to_string())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .await
    .with_context(|| format!("Failed to run script on file: {}", path.display()))?;

    if status.success() {
        println!("✅ Done: {}", path.display());
    } else {
        eprintln!("❌ Script failed: {}", path.display());
    }

    Ok(())
}

/// Recursively process all valid files under a directory in parallel
pub async fn run_file_processor(root_dir: &str) -> Result<()> {
    let root = PathBuf::from(root_dir);

    if !root.exists() || !root.is_dir() {
        return Err(anyhow::anyhow!("Not a valid directory: {}", root.display()));
    }

    let tasks: Vec<_> = WalkDir::new(&root)
    .into_iter()
    .filter_map(Result::ok)
    .filter(is_valid_file)
    .map(|entry| {
        let path = entry.into_path();
        task::spawn(async move {
            if let Err(e) = process_file(path.clone()).await {
                eprintln!("❌ Error: {} — {}", path.display(), e);
            }
        })
    })
    .collect();

    let total = tasks.len();

    if total == 0 {
        println!("ℹ️ No visible files found to process in: {}", root.display());
    } else {
        println!("🔄 Processing {} files in parallel...", total);
        join_all(tasks).await;
        println!("🎉 Finished processing all files.");
    }

    Ok(())
}
