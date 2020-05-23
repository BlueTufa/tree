A simple tree view which prints the full path of all files on a local file system, starting at the current working directory.

This is very useful as an input to pipe operations.

# How to install
Start by installing rustup.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then do a cargo install within the root of this source tree:
```bash
cargo install --path $(pwd)
```

Make sure you have `$HOME/.cargo/bin` in your path.
```bash
export PATH=$HOME/.cargo/bin:$PATH
```

# Usage:
```bash
tree
```
## Regex support
You can pass any regex in as an optional arg and it will filter out those file names inline.  

Example: filter out any files with `.git` in the name.
```bash
tree '\.git'
```
Example: filter out any file ending with `.d`.
```bash
tree '\.d$'
```
