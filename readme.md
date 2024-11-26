# Geet

**Geet** is a lightweight Git implementation built with Rust. This README provides a step-by-step guide to building, running, and using Geet commands.

---

## Build and Setup

1. Build the `geet` binary using Cargo:

   ```bash
   cargo build
   ```

2. Add the `geet` binary to your system's PATH:
   ```bash
   export PATH=$PATH:$(pwd)/target/debug
   ```

---

## Commands and Usage

### Initialize a New Repository

Create a test repository and initialize it with Geet:

```bash
mkdir test
cd test
geet init
```

### Stage a File

Add (stage) a file to the repository:

```bash
geet add test.txt
```

### Make a Commit

Commit staged changes with a message:

```bash
geet commit -m "first one"
```

### Check Repository Status

Check the current status of the repository (what is staged and not staged)

```bash
geet status
```

### Show File or Commit Content

View the content of a file or commit details:

```bash
geet cat test.txt
geet cat <commit-hash>
```

### Checkout a Previous Commit

Switch to a previous commit using its hash:

```bash
geet checkout a145d0486463ceb2840f5c871608f142b713736f
```

### Clean Up the Repository

Reset the repository for a fresh start:

```bash
geet cleanup
```

---

## Notes

- After the initial setup, all commands can be run directly with `geet` instead of `cargo run --`.
- Replace `<commit-hash>` in commands with the actual hash of the commit you want to reference.
- The cat command supports both current files and commit objects with formatted output.
- The cleanup command safely removes the .geet directory, allowing for a fresh repository initialization.

---

Feel free to update this README as you add more features to Geet!

# todo

- right now we can make identical commits multiple times. Check if the tree_hash is the same, if it is don't create a new commit
- right now we can re-init a repository, disable that
- refactor cli parser error handling using ?
- refactor command handling code to be a group of helper functions, no more enums
- when we make a new commit, we only change HEAD to point to it. We need to make the current branch to point to it as well

# known bugs

- add/remove treats path with "./" prefix as unique from the same path without it
- status command treats empty files the same, because empty content gets hashed to the same value
