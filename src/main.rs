use std::env;
use std::fs;
use std::path::Path;

fn file_to_string(file_path: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let full_path = current_dir.join(file_path);
    let contents = fs::read_to_string(full_path)
        .expect("Failed to read the file");

    contents
}

fn main() {
    let file_path = "example.txt";
    let contents = file_to_string(file_path);
        
    println!("With text:\n{}", contents);
}
