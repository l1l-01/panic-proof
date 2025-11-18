# Encrypt-file — Simple Rust File Encryptor/Decryptor

> **"Hello World" is for tutorials. This encrypts files.**  
> This encrypts files securely. A simple and safe file encryptor/decryptor — built with clap, argon2, and ChaCha20Poly1305.

---

## Features

- Encrypt or decrypt any file with a password.

- Secure key derivation using Argon2 (password → 32-byte key).

- Authenticated encryption using ChaCha20-Poly1305.

- Salt and nonce stored in file so you can safely decrypt later.

- Safe CLI parsing with input validation.

---

## How It Works
1. User provides a mode (encrypt or decrypt), a file path, and a password.
2. Key derivation:
 - Argon2 generates a secure 32-byte key from the password + random salt.
3. Encryption:
 - A random 12-byte nonce is generated.
 - File content is encrypted using ChaCha20Poly1305.
4. Decryption
 - Reads salt + nonce from the file.
 - Derives the same key from password + salt.
 - Decrypts the ciphertext back to the original file content.
 - Only the password is secret. Salt and nonce are stored in the file.

---

## How to Run

### Encrypt a file
```bash
cargo run -- encrypt <file_path> <password>
```
example:
```bash
cargo run -- encrypt secret.txt mystrongpassword
```

### Decrypt it back
```bash
cargo run -- decrypt <encrypted_file_path> <password>
```
exmaple:
```bash
# Decrypt it back
cargo run -- decrypt secret.txt.enc mystrongpassword
```

---

## Learning Goals

- Real CLI apps, easy argument parsing: clap::Parser
- Modern, secure password-based key derivation: Argon2
- Fast, safe authenticated encryption: ChaCha20Poly1305
- Reading/writing files safely, storing metadata (salt + nonce): File handling
- Fail early, informative messages for users: Error handling

## Current Status
> Fully working: encryption & decryption.
