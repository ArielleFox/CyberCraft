use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::socket::SocketAddr;
use std::net::SocketAddr;
use std::error::Error;
use std::collections::HashMap;

fn timer<F>(func: F)
where
F: FnOnce(),
{
    let start = Instant::now();
    println!("Started at: {:?}", start.elapsed());
    func();
    println!("Ended at: {:?}", start.elapsed());
}

fn file_manager<F>(path: &Path, mode: &str, func: F) -> io::Result<()>
where
F: FnOnce(&mut File),
{
    let mut file = match mode {
        "r" => File::open(path)?,
        "w" => File::create(path)?,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode")),
    };

    func(&mut file);

    Ok(())
}

fn encrypt_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let ident = "first.txt";
    let identity_path = dirs::home_dir().unwrap().join(format!(".yubiCrypt/keys/{}", ident));

    println!("Encrypting file: {}", file_path.display());

    timer(|| {
        if !identity_path.is_file() {
            eprintln!("❌ Identity file not found: {}", identity_path.display());
            return;
        }

        // Extract recipient key
        let recipient_key = match read_recipient_key(&identity_path) {
            Ok(key) => key,
          Err(e) => {
              eprintln!("❌ Failed to read recipient key: {}", e);
              return;
          }
        };

        // Encrypt the file using `age`
        let encrypted_file = format!("{}.age", file_path.display());
        let status = Command::new("age")
        .arg("-r")
        .arg(&recipient_key)
        .arg("-o")
        .arg(&encrypted_file)
        .arg(file_path)
        .spawn()?
        .wait()?;

        if status.success() {
            println!("✅ SUCCESSFULLY ENCRYPTED: {} ==> {}", file_path.display(), encrypted_file);
            fs::remove_file(file_path)?; // Remove original file
        } else {
            eprintln!("❌ Encryption failed for: {}", file_path.display());
        }
    });

    Ok(())
}

fn read_recipient_key(path: &Path) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains("Recipient") {
            return Ok(line[16..].trim().to_string()); // Extract recipient key
        }
    }

    Err("Recipient key not found in identity file.".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().len() != 2 {
        eprintln!("Usage: {} <file_to_encrypt>", std::env::args().next().unwrap());
        std::process::exit(1);
    }

    let file_to_encrypt = Path::new(&std::env::args().nth(1).unwrap());

    if file_to_encrypt.is_file() {
        encrypt_file(file_to_encrypt)?;
    } else {
        eprintln!("❌ File not found: {}", file_to_encrypt.display());
        std::process::exit(1);
    }

    Ok(())
}
