use base16ct;
use core::panic;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::prelude::Write;
use std::path;

pub struct FileCache {
    base_dir: Box<path::Path>,
}

impl FileCache {
    pub fn new(base_dir: Box<path::Path>) -> Self {
        if base_dir.exists() && !base_dir.is_dir() {
            panic!("The cache base dir exists and is not a directory");
        }

        if !base_dir.exists() {
            fs::create_dir_all(&base_dir).expect("Error creating the base dir for the file cache");
        }

        FileCache { base_dir }
    }

    pub fn store(&self, file_name: &str, contents: &str) {
        let true_filename = self.full_path(file_name);
        println!("{:?}", &true_filename);
        fs::File::create(true_filename)
            .expect("Failed to create a file in disk")
            .write_all(contents.as_bytes())
            .expect("Failed to write file to disk");
    }

    pub fn retrieve(&self, file_name: &str) -> Option<String> {
        return match fs::read_to_string(self.full_path(file_name)) {
            Ok(contents) => Some(contents),
            _ => None,
        };
    }

    fn full_path(&self, file_name: &str) -> Box<path::Path> {
        let true_filename = base16ct::lower::encode_string(&Sha256::digest(file_name));
        return self.base_dir.join(true_filename).into_boxed_path();
    }
}
