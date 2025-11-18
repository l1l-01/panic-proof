use argon2::Argon2;
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit},
};
use clap::Parser;
use core::convert::TryFrom;
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

    // Read password
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

    // Read the encrypted file
    let file_data = fs::read(&cli.file).expect("Failed to read file");

    // If decrypting -> extract salt + nonce + ciphertext
    let (salt, nonce_bytes, ciphertext) = if mode == "decrypt" {
        if file_data.len() < 16 + 12 {
            panic!("Corrupted file: too small");
        }

        let salt = &file_data[0..16];
        let nonce_bytes = &file_data[16..28];
        let ciphertext = &file_data[28..];

        (salt.to_vec(), nonce_bytes.to_vec(), ciphertext.to_vec())
    } else {
        // If encrypting -> generate new salt & nonce
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);

        (salt.to_vec(), nonce.to_vec(), file_data)
    };

    // Derive key from password + salt (same for encrypt/decrypt)
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password, &salt, &mut key)
        .expect("Argon2 key derivation failed");

    // Create cipher
    let cipher = ChaCha20Poly1305::new((&key).into());

    // Nonce must be 12 bytes
    let nonce = Nonce::try_from(&nonce_bytes[..]).expect("Invalid nonce");

    // Encrypt or decrypt
    let output = if mode == "encrypt" {
        cipher
            .encrypt(&nonce, ciphertext.as_ref())
            .expect("Encryption failed")
    } else {
        cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .expect("Decryption failed")
    };

    // Build output name
    let out_name = if mode == "encrypt" {
        format!("{}.enc", cli.file)
    } else {
        cli.file.replace(".enc", "")
    };

    // When encrypting -> prepend salt + nonce for the decryptor
    let mut final_data = Vec::new();
    if mode == "encrypt" {
        final_data.extend_from_slice(&salt);
        final_data.extend_from_slice(&nonce_bytes);
    }
    final_data.extend_from_slice(&output);

    fs::write(&out_name, final_data).expect("Failed to write output file");

    if mode == "encrypt" {
        println!("Encryption was done!");
    } else {
        println!("Decryption was done!");
    }
}
