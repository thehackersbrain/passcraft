# Passcraft
> Wordlist generator in Rust

Password Profiler based on User's Personal Information

## Installation

- Clone the project and change the directory
```bash
git clone https://github.com/thehackersbrain/passcraft.git
cd passcraft
```

- Build the project
```bash
cargo build --release
```

- Copy the binary into path
```bash
cp target/release/passcraft ~/.local/bin/
```

## Todo
- [x] Convert all the inputs taken from user to lowercase before doing anything else.
- [ ] Create a banner with some cool effect or with unique approach.
- [ ] Create a library to munge (convert into leet) the wordlist either the generated one or specified one.

## Author

- [**Gaurav Raj**](https://thehackersbrain.github.io/)
