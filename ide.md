# Rust Toolchain
[Download](https://rustup.rs/)

On windows:
* Open the installer
* Press 1 and then press enter to proceed with default installation

# Visual Studio Code
[Download](https://code.visualstudio.com/)

## rust-analyzer extension
You will need this for syntax highlighting
[Download](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Creating a new project
1. Open VS Code
2. Create a folder for your project
3. Click `File>Open Folder...`
4. Press ctrl+j to open terminal
5. Run `cargo init` command to generate a minimal hello world project
6. Run `cargo run` command to run your program

## Verify that rust-analyzer works properly
Open `src/main.rs/` and hover over `println!()` macro function and see if the documentation pops out. If not, something isn't working. Close your VS Code, open it and try again. If it still doesn't work, contact your coordinator.