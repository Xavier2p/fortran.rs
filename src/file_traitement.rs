use std::env;
use std::fs;

#[allow(dead_code)]
#[derive(Clone)]
pub struct File {
    path: String,
    name: String,
    content: String,
    version: String,
}

// #[allow(dead_code)]
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
}

fn get_file_path() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    file_path.to_string()
}

fn get_file_name(file_path: &str) -> (String, String) {
    let file_name_with_extension: &str = file_path.split('/').last().unwrap();
    let file_name: &str = file_name_with_extension.split('.').next().unwrap();
    let version: &str = file_name_with_extension.split('.').last().unwrap();
    return (file_name.to_string(), version.to_string());
}

fn get_file_content(file_path: &str) -> String {
    let file_content: String = fs::read_to_string(file_path).unwrap();
    file_content
}

pub fn open_file() -> File {
    let file_path: String = get_file_path();
    let file_names: (String, String) = get_file_name(&file_path);
    let file_content: String = get_file_content(&file_path);

    return File {
        path: file_path,
        name: file_names.0,
        content: file_content,
        version: file_names.1,
    };
}
