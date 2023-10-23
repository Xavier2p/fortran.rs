//! File management
//!
//! This module contains the `File` struct and its methods.
use crate::helpers::cli::Cli;
use std::fs;

/// This struct contains the file's path, name, content, version and arguments.
#[derive(Clone)]
pub struct File {
    path: String,
    name: String,
    content: String,
    version: String,
}

/// This struct contains the file's path, name, content, version and arguments.
impl File {
    /// This function returns the file's content.
    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn debug(&self) {
        println!("File {{");
        println!("    name: {}", self.name);
        println!("    path: {}", self.path);
        println!("    version: {}", self.version);
        println!("    lines: [");
        for line in self.content.lines() {
            println!("        {:?}", line);
        }
        println!("    ]");
        println!("}}");
        println!("---- ---- ---- ---- ----");
    }
}

/// This function returns the file's arguments.
pub fn new(args: Cli) -> File {
    let path: String = args.get_path();
    let file_name_with_extension: &str = path.split('/').last().unwrap();

    return File {
        path: path.clone(),
        name: file_name_with_extension
            .split('.')
            .next()
            .unwrap()
            .to_string(),
        content: fs::read_to_string(path.clone()).unwrap(),
        version: file_name_with_extension
            .split('.')
            .last()
            .unwrap()
            .to_string(),
    };
}
