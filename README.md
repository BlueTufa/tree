A simple tree view which prints the full path of all files on a local file system.

This is very useful as an input to pipe operations.

# How to install
Start by installing rustup.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then do a cargo install within the root of the source tree:
```bash
cargo install --path $(pwd)
```
