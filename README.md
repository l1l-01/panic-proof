# 🦀 Panic-Proof: Beginner Rust projects that won't panic ... most of the time.

> **"Learn Rust by breaking it — safely."**
> A curated playground of bite-sized, beginner-friendly.


![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Beginner](https://img.shields.io/badge/level-beginner-brightgreen?style=for-the-badge)
![Zero Panic](https://img.shields.io/badge/panic%20mode-off-blue?style=for-the-badge)

---

## 🚀 Why `panic-proof`?

You’ve seen the memes. You’ve felt the fear.
**"The borrow checker is judging me."**
**"Why is my `main.rs` 47 errors deep?"**

This repo is your **safe zone**.
Every project here is:

- **Not Self-contained** *(Cargo.toml drama is inevitable — embrace the `[dependencies]` chaos)*  
- **Clearly explained** *(comments so detailed, even your grandma could debug it)*  
- **Progressively harder** *(starts with XOR, ends with your soul in a `Box<Any>`)*  
- **Panic-proof** *(go ahead, `unwrap()` like a madman — we dare you)*  
- **Every project gets its own mini-README** *(because `main.rs` shouldn’t do all the talking)*

---

## 📂 Project Map (Click to Jump)

| # | Project | Skill Focus | Difficulty |
|---|--------|-------------|------------|
| 01 | [encrypt-file] | CLI args, file I/O, encryption | ⭐ |

More coming soon — PRs welcome!

---

## ⚙️ Getting Started (3 Steps, No PhD Required)

```bash
# 1. Clone the repo
```bash
git clone https://github.com/l1l-01/panic-proof.git
cd panic-proof
```

# 2. Pick a project, for example:
```bash
cd projects/encrypt-file
```

# 3. Build it & Run it!
```bash
cargo build
cargo run
```
---

> **Note**: This repo doesn’t do hand-holding.
> If `cargo build` doesn’t make you sweat, you’re probably overqualified.
