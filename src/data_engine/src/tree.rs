use std::fs;
use std::path::Path;
use std::io;
use std::env;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_program_data_folder() -> String {
    // For Unix-like systems (Linux, macOS)
    "/usr/local/bin".to_string()  // Modify this path as needed
}

#[cfg(target_os = "windows")]
fn get_program_data_folder() -> String {
    // For Windows systems
    let program_data_folder = env::var("ProgramData").expect("Failed to get ProgramData");
    format!("{}/Microsoft/Windows/Start Menu/Programs", program_data_folder)
}

pub async fn list_folder() -> io::Result<()> {
    let program_data_folder = get_program_data_folder();
    let path = Path::new(&program_data_folder);
    let paths = fs::read_dir(path)?; // Using ? for error handling

    for entry in paths {
        let folder = entry?; // Handle any errors
        println!("Name: {}", folder.path().display());

        if folder.path().is_dir() {
            // Here, we can process subdirectories or handle them in a way that's appropriate for your use case
            println!("This is a directory.");
        }
    }
    Ok(())
}

// The correct way to use #[tokio::main] without making main async manually
#[tokio::main]
async fn main() {
    let yeet: u8 = 32;
    let pwd = list_folder().await;  // Await the future

    println!("Number selected: {yeet}");

    match pwd {
        Ok(_) => println!("Directory listing completed successfully."),
        Err(e) => println!("An error occurred: {}", e),
    }
}
