## Git - my implmementation of some git commands (WIP)

This is a simple implementation of some git commands in Rust. It is a work in progress and currently supports the following commands:
- `init`: Initialize a new git repository
- `cat-file`: Display the content of a file in the git object database
- `hash-object`: Compute the object ID of a file and optionally store it in the git object database
- `ls-tree`: List the contents of a tree object
- `write-tree`: Create a tree object from the current index


### Usage
1. Make sure you have Rust installed. You can install it from [here](https://www.rust-lang.org/tools/install).
2. Clone this repository:
   ```bash
   git clone https://github.com/arjav0703/git-rust.git
    cd git-rust
    ```
3. Build the project:
    ```bash
    cargo build --release
    ```

4. Run the commands:
    ```bash
    target/release/git <command> [args]
    ```

### Example
```bash
# Initialize a new git repository
target/release/git init

# Create a new file
echo "Hello, World!" > hello.txt

# Compute the object ID of the file and store it in the git object database
target/release/git hash-object -w hello.txt

# Display the content of the file in the git object database
target/release/git cat-file -p <object-id>

# List the contents of the tree object
target/release/git ls-tree <tree-object-id>

# Create a tree object from the current index
target/release/git write-tree
```
