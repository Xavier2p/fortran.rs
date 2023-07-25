use crate::preprocess::{get_path, Args};
use std::fs;

#[allow(dead_code)]
#[derive(Clone)]
pub struct File {
    path: String,
    name: String,
    content: String,
    version: String,
    args: Args,
}

#[allow(dead_code)]
impl File {
    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn get_args(&self) -> &Args {
        &self.args
    }

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
