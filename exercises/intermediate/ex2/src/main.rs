use std::path::PathBuf;

fn main() {
    let dir_path = todo!("read directory path from command line arguments");

    let dir_content = todo!("get contents of the directory");

    // `Vec<(PathBuf, u32)>` is a dynamically sized array
    // of tuples containing `PathBuf` and `u32`
    let ratings: Vec<(PathBuf, u32)> = todo!("
        for entry in dir_content
            clear the screen
            present contents of a file to the user
            ask user to input his rating from a 1-5 scale and store it in `ratings`,
    ");
    
    todo!("Present the ratings summary to the user");
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
