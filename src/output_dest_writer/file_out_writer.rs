use crate::output_dest_writer::OutputDestWriter;
use std::boxed::Box;
use std::fs;
use std::io::Write;
use std::path;

pub struct FileOutWriter {
    directory: String,
}

impl FileOutWriter {
    pub fn new(directory: String) -> Self {
        Self { directory }
    }
}

impl OutputDestWriter for FileOutWriter {
    fn get_writer(&mut self, file_path_parts: &[&str]) -> Box<Write> {
        let mut dir_path = path::PathBuf::new();
        dir_path.push(&self.directory);

        for i in 0..file_path_parts.len() - 1 {
            dir_path.push(&file_path_parts[i]);
            println!(" dir push ... {:?}", file_path_parts[i]);
        }

        let file_path = dir_path.join(format!("{}.rs", &file_path_parts.last().unwrap()));
        println!("write to ... {:?}", file_path);
        fs::create_dir_all(dir_path).unwrap();
        let file = fs::File::create(&file_path).unwrap();
        Box::new(file)
    }
}
