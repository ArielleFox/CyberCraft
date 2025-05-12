use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::env;

pub async fn list_all_folder() -> io::Result<()> {
    // Get the current working directory
    let current_dir = env::current_dir()?;

    // Get the parent directory of the current working directory
    let parent_dir = current_dir.parent().unwrap_or_else(|| Path::new("."));

    // List contents of the parent directory, including hidden files and directories
    let paths = fs::read_dir(parent_dir)?; // Using ? for error handling

    for entry in paths {
        let folder = entry?; // Handle any errors
        let folder_path = folder.path();
        println!("Name: {}", folder_path.display());

        if folder_path.is_dir() {
            // Here, we can process subdirectories or handle them in a way that's appropriate for your use case
            println!("This is a directory.");
        }
    }
   Ok(())
}

fn find_git_layer_up(start_dir: &Path) -> io::Result<Option<u32>> {
    let mut current_dir = start_dir.to_path_buf();
    let mut layers_up = 0;

    // Loop until we reach the root directory or find `.git`
    loop {
        let git_dir = current_dir.join(".git");

        if git_dir.exists() && git_dir.is_dir() {
            // If `.git/` exists, return the number of layers up
            return Ok(Some(layers_up));
        }

        // Move up one directory
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
            layers_up += 1;
        } else {
            // We have reached the root directory, stop searching
            break;
        }
    }

    // Return None if `.git` was not found
    Ok(None)
}

pub async fn list_folder() -> io::Result<()> {
    // Get the current working directory
    let current_dir = env::current_dir()?;
    
    // Call the find_git_layer_up function to find the layers to `.git`
    match find_git_layer_up(&current_dir)? {
        Some(layers) => println!(".git found at layer level: {}", layers),
        None => println!(".git not found in any parent directories."),
    }

    Ok(())
}

#[tokio::main]
async fn main() {

    let all = list_all_folder().await; 
    let pwd = list_folder().await;  // Await the future

    match all {
        Ok(_) => println!("Directory listing completed successfully."),
        Err(e) => println!("An error occurred: {}", e),
    }

    match pwd {
        Ok(_) => println!("Directory listing completed successfully."),
        Err(e) => println!("An error occurred: {}", e),
    }
}
