## Git - my implmementation of some git commands (WIP)

This is a simple implementation of some git commands in Rust. It is a work in progress and currently supports the following commands:
- `init`: Initialize a new git repository
- `cat-file`: Display the content of a file in the git object database
- `hash-object`: Compute the object ID of a file and optionally store it in the git object database
- `ls-tree`: List the contents of a tree object
- `write-tree`: Create a tree object from the current index

### Installation 
1. Make sure you have Rust installed. You can install it from [here](https://www.rust-lang.org/tools/install).
2. Install with cargo:
   ```bash
   cargo install git-rust
   ```
3. After installation, you can run the commands using:
   ```bash
   git-rust <command> [args]
   ```

### Example
```bash
# Initialize a new git repository
git-rust init

# Create a new file
echo "Hello, World!" > hello.txt

# Compute the object ID of the file and store it in the git object database
git-rust hash-object -w hello.txt

# Display the content of the file in the git object database
git-rust cat-file -p <object-id>

# List the contents of the tree object
git-rust ls-tree <tree-object-id>

# Create a tree object from the current index
git-rust write-tree
```
