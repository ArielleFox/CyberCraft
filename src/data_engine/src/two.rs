use std::env;
use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};
use std::process::Command;
use std::fs;
use std::io::{self, Write}; // for user input

fn copy_config_file_to(dest_dir: &Path, git_root: &Path) -> io::Result<()> {
    let config_file = git_root.join("cy_config.yaml");
    let dest_file = dest_dir.join(".cy_config.yaml");

    if config_file.exists() {
        std::fs::copy(&config_file, &dest_file)?;
        println!("Copied config to: {}", dest_file.display());
    } else {
        eprintln!("Config file not found at: {}", config_file.display());
    }

    Ok(())
}

fn find_git_root() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home"));
    let home_path = Path::new(&home_dir);
    let mut current_path = current_dir;

    loop {
        let git_dir = current_path.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            return Some(current_path);
        }

        if current_path == home_path {
            return None;
        }

        current_path = current_path.parent()?.to_path_buf();
    }
}


fn is_git_repository() -> bool {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Failed to get current directory.");
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
    let git_root = match find_git_root() {
        Some(path) => path,
        None => {
            println!("Not inside a Git repository.");
            return;
        }
    };

    println!("Git root found at: {}", git_root.display());
    println!("Scanning for folders...");

    let mut dir_entries: Vec<DirEntry> = WalkDir::new(".")
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| e.file_type().is_dir())
    .collect();

    dir_entries.sort_by(|a, b| b.depth().cmp(&a.depth()));

    // Categorize
    let mut with_index_age = Vec::new();
    let mut without_index_age = Vec::new();

    for entry in &dir_entries {
        let path = entry.path();
        let index_age_path = path.join("index.age");
        if index_age_path.exists() {
            with_index_age.push(entry.clone());
        } else {
            without_index_age.push(entry.clone());
        }
    }

    let has_mixed = !with_index_age.is_empty() && !without_index_age.is_empty();

    let mut skip_mismatches = false;

    if has_mixed {
        println!("\n⚠️ Warning: Some folders have 'index.age' and others don't.");
        println!("This means some will be decrypted and others encrypted.");
        print!("Do you want to [c]ontinue, [s]kip mismatches? (c/s): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();
        if choice == "s" {
            skip_mismatches = true;
        }
    }

    // Process folders
    for entry in dir_entries {
        let path = entry.path();
        let has_index_age = path.join("index.age").exists();

        if skip_mismatches {
            if has_index_age && !with_index_age.iter().any(|e| e.path() == entry.path())
            {
                println!("Skipping {} (unexpected index.age)", path.display());
                continue;
            }
            if !has_index_age && !without_index_age.iter().any(|e| e.path() == entry.path())
            {
                println!("Skipping {} (unexpected no index.age)", path.display());
                continue;
            }
        }

        println!("\nProcessing: {}", path.display());

        // List contents
        let ls_output = Command::new("ls")
        .arg(path)
        .output()
        .expect("Failed to execute ls");

        let contents = String::from_utf8_lossy(&ls_output.stdout);
        println!("Contents:\n{}", contents);

        // Copy config
        let config_path = path.join(".cy_config.yaml");
        if let Err(e) = copy_config_file_to(path, &git_root) {
            eprintln!("Failed to copy config: {}", e);
            continue;
        }

        if let Ok(contents) = std::fs::read_to_string(&config_path) {
            println!("Config contents:\n{}", contents);
        }

        // Determine mode
        let mode = if has_index_age { "--decrypt" } else { "--encrypt" };
        println!("Running cybercraft {}", mode);

        let status = Command::new("cybercraft")
        .arg(mode)
        .arg(".") // pass current folder as arg
        .current_dir(path)
        .status(); // ← ADD THIS SEMICOLON

        if !status.expect("Failed to run cybercraft").success() {
            eprintln!("cybercraft {} failed in directory: {}", mode, path.display());
        }

        // Cleanup config
        //match std::fs::remove_file(&config_path) {
          //  Ok(_) => println!("Removed config from: {}", config_path.display()),
            //Err(e) => eprintln!("Failed to remove config file: {}", e),
        //}
    }
}


fn main() {
    scan_folders();
}
