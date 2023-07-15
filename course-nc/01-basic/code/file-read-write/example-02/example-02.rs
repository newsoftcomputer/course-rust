
use std::env;
// use std::fs;

fn main() {
    // --snip--
    // println!("In file {}", file_path);

    let contents = env(".env").expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}