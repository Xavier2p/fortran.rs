//! # File Traitement
//!
//! This module contains the `File` struct and its methods.
use crate::preprocess::{get_path, Args};
use std::fs;

/// This struct contains the file's path, name, content, version and arguments.
#[allow(dead_code)]
#[derive(Clone)]
pub struct File {
    path: String,
    name: String,
    content: String,
    version: String,
    args: Args,
}

/// This struct contains the file's path, name, content, version and arguments.
impl File {
    // pub fn get_path(&self) -> &String {
    //     &self.path
    // }

    /// This function returns the file's name.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// This function returns the file's content.
    pub fn get_content(&self) -> &String {
        &self.content
    }

    // pub fn get_version(&self) -> &String {
    //     &self.version
    // }

    /// This function returns the file's arguments.
    pub fn get_args(&self) -> &Args {
        &self.args
    }

    /// This function returns the file's arguments.
    pub fn new(args: Args) -> File {
        let path: String = get_path(&args);
        let file_name_with_extension: &str = path.split('/').last().unwrap();

        return File {
            path: get_path(&args),
            name: file_name_with_extension
                .split('.')
                .next()
                .unwrap()
                .to_string(),
            content: fs::read_to_string(get_path(&args)).unwrap(),
            version: file_name_with_extension
                .split('.')
                .last()
                .unwrap()
                .to_string(),
            args,
        };
    }
}
