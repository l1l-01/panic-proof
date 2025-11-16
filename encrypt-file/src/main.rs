use argon2::Argon2;
use clap::Parser;
use rand::{RngCore, rngs::OsRng};
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "encryp-file")]
#[command(about = "Encrypt or decrypt files with a password")]
struct Cli {
    /// Encrypt or decrypt
    #[arg(value_name = "MODE")]
    mode: String,

    /// Path to the file
    #[arg(value_name = "FILE")]
    file: String,

    /// Password (keep it secret!)
    #[arg(value_name = "PASSWORD")]
    password: String,
}

fn main() {
    let cli = Cli::parse();

    // Validate mode
    let mode = cli.mode.trim().to_lowercase();
    if mode != "encrypt" && mode != "decrypt" {
        println!("Error: Mode must be 'encrypt' or 'decrypt'");
        return;
    }

    // Validate file exists
    let path = Path::new(&cli.file);
    if !path.exists() {
        println!("Error: File '{}' not found!", cli.file);
        return;
    }
    if path.is_dir() {
        println!(
            "Error: '{}' is a directory! Please provide a file.",
            cli.file
        );
        return;
    }

    // Validate password
    if cli.password.trim().is_empty() {
        println!("Error: Password cannot be empty!");
        return;
    }

    println!(
        "Running: {} on file '{}' with password (hidden)",
        mode.to_uppercase(),
        cli.file
    );

    // Read the original file (_ for now)
    let _file = fs::read(cli.file).expect("Failed to read file");
    let password = cli.password.as_bytes();

    // Generate random salt
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    // produce a 32-byte key buffer
    let mut key = [0u8; 32];

    // Argon2 derives key directly into the buffer
    Argon2::default()
        .hash_password_into(password, &salt, &mut key)
        .expect("Argon2 key derivation failed");

    // key now contains the 32-byte encryption key
    println!("Key derived successfully: {} bytes", key.len());

    // TODO: finish XOR encryption/decryption logic here
}
