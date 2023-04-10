pub trait FileReader {
    fn read_contents(&mut self) -> &String;
}
 
pub struct RealFileReader {
    file_path: String,
    contents: String,
}

impl RealFileReader {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            contents: String::new(),
        }
    }

    fn read_file(&mut self) {
        // Read the file and store its contents in self.contents
        let content = std::fs::read_to_string(&self.file_path).expect("Unable to read file");
        self.contents = content;
    }
}

impl FileReader for RealFileReader {
    fn read_contents(&mut self) -> &String {
        if self.contents.is_empty() {
            self.read_file();
        }
        &self.contents
    }
}

pub struct FileReaderProxy {
    file_path: String,
    real_file_reader: Option<RealFileReader>,
}

impl FileReaderProxy {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            real_file_reader: None,
        }
    }
}

impl FileReader for FileReaderProxy {
    fn read_contents(&mut self) -> &String {
        if self.real_file_reader.is_none() {
            let real_file_reader = RealFileReader::new(self.file_path.clone());
            self.real_file_reader = Some(real_file_reader);
        }
        self.real_file_reader.as_mut().unwrap().read_contents()
    }
}
