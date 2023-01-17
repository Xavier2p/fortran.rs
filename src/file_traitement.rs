use std::fs;

#[allow(dead_code)]
#[derive(Clone)]
pub struct File {
    path: String,
    name: String,
    content: String,
    version: String,
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

    pub fn new(path: &String) -> File {
        let file_name_with_extension: &str = path.split('/').last().unwrap();

        return File {
            path: path.to_string(),
            name: file_name_with_extension
                .split('.')
                .next()
                .unwrap()
                .to_string(),
            content: fs::read_to_string(path).unwrap(),
            version: file_name_with_extension
                .split('.')
                .last()
                .unwrap()
                .to_string(),
        };
    }
}
