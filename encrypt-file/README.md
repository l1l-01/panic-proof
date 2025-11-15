# Encrypt-file — Your First Real Rust Tool

> **"Hello World" is for tutorials. This encrypts files.**  
> A dead-simple XOR-based file encryptor/decryptor — built with `clap` and zero hand-holding.

---

## What This Project Teaches (No Fluff)

| Skill | Why It Matters |
|------|----------------|
| `clap::Parser` | Real CLI apps, not `std::env::args()` hacks |
| `std::path::Path` | File system safety (no more `../etc/passwd` surprises) |
| Input validation | Fail early, fail loud — Rust style |
| String handling | `.trim()`, `.to_lowercase()` — the small things that break |
| Project structure | `Cargo.toml`, `main.rs`, and *this* README |

---

## How to Run
```bash
#Encrypt a file
cargo run -- encrypt secret.txt yourpassword
```

```bash
# Decrypt it back
cargo run -- decrypt secret.txt.enc yourpassword
```

---

## In Progress (Yes, I’m Building This Live)

> **Current State**: It *talks* like a pro. Soon it’ll *encrypt* like one.
