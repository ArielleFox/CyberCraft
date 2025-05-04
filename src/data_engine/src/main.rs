use std::env;
use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};

fn is_git_repository() -> bool {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            println!("Failed to get current directory.");
            return false;
        }
    };

    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home"));
    let home_path = Path::new(&home_dir);

    let mut current_path = current_dir;

    loop {
        println!("Checking directory: {}", current_path.display());

        let git_dir = current_path.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            println!("Found .git directory at: {}", current_path.display());
            return true;
        }

        if current_path == home_path {
            println!("Reached home directory, stopping search.");
            return false;
        }

        match current_path.parent() {
            Some(parent) => current_path = parent.to_path_buf(),
            None => {
                println!("No more parent directories to check.");
                return false;
            }
        }
    }
}

fn scan_folders() {
    if is_git_repository() {
        println!("Inside a Git repository. Scanning for folders...");

        let mut dir_entries: Vec<DirEntry> = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .collect();

        // Sort directories by depth in descending order to process deepest first
        dir_entries.sort_by(|a, b| b.depth().cmp(&a.depth()));

        println!("Stash processing order:");
        for entry in &dir_entries {
            println!("Layer: {}", entry.path().display());
        }

        // Process the directories in the determined order
        println!("\nProcessing stash from first to last:");
        for entry in dir_entries {
            let path = entry.path();
            println!("Processing: {}", path.display());
            // Here you would add the actual processing logic for each directory
  }
    } else {
        println!("Not inside a Git repository.");
    }
}

fn main() {
    scan_folders();
}
