use clap::Parser;
use std::path::Path;

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

    // TODO: Add XOR encryption/decryption logic here
    println!("(Encryption logic coming soon... stay tuned!)");
}
