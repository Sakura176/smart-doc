use std::io::Write;
use std::{collections::HashMap, fs::File, path::PathBuf};

use zip::{
    ZipWriter,
    write::{FileOptions, SimpleFileOptions},
};

pub struct DocxBuilder {
    files: HashMap<PathBuf, Vec<u8>>,
}

impl DocxBuilder {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: PathBuf, data: Vec<u8>) {
        self.files.insert(path, data);
    }

    pub fn build(&self, zip_path: PathBuf) -> Result<(), zip::result::ZipError> {
        let file = File::create(zip_path)?;
        let mut zip = ZipWriter::new(file);
        for (path, data) in &self.files {
            zip.start_file(path.display().to_string(), SimpleFileOptions::default())?;
            zip.write_all(data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let mut builder = DocxBuilder::new();
        builder.add_file(PathBuf::from("test.txt"), b"Hello, World!".to_vec());
        let zip_path = PathBuf::from("target/tmp/test.zip");
        assert!(builder.build(zip_path).is_ok());
    }
}
