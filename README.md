# Password Manager

This is a password manager made in Rust. The purpose of this small project is to practice this new language I'm learning.

## Overview

- Data model: username, password, website URL (structs)

- Encryption: AES?, ChaCha20? Libraries: 'ring', 'rust-crypto'

- Storage: file? database? Libraries: 'rusqlite' for encrypted SQLite DB or 'sled' for an encrypted key-value store

- User Interface: Command line or GUI. Libraries: 'structopt' for parsing CLI args and 'gtk-rs' or 'druid' for GUI

- Password Generation: Generate randonm strong passwords. Libraries: 'rand' for random number generation and character selection.

- User Authentication: Implement user authentication (maybe using a master password??)

