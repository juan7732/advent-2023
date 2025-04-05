use std::fs::File;
use std::io::{self, Read};

fn main() {
    let file_path = "./src/data/test.txt";
    match load_data(file_path) {
        Ok(()) => println!("File read successfully"),
        Err(e) => println!("Error reading file: {:?}", e),
    }
}

fn load_data(file_path: &str) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("in file {}", file_path);
    println!("With text:\n{contents}");
    Ok(())
}

