use std::fs;
use sanitizer::prelude::*;

use clap::Parser;

/// Simple CLI app
#[derive(Parser, Debug)]
struct Args {
    /// Files/folders to sanitize
    files: Vec<String>,
}

/// Sanitizes 
fn sanitize(path: &String) -> String {
    let mut sanitizer_instance = StringSanitizer::from(path.to_owned());
    sanitizer_instance
        .alphanumeric()
        .to_lowercase();
    sanitizer_instance.get()
}

fn main() {
    let args = Args::parse();

    println!("Hello, world!");

    for path in args.files {
        let metadata = fs::metadata(&path).unwrap();
        let file_type = metadata.file_type();
        if file_type.is_file() {
            println!("Sanitizing file {} -> {}...", &path, sanitize(&path));
        } else if file_type.is_dir() {
            println!("Sanitizing folder {} -> {}...", &path, sanitize(&path));
        }
    }
}
