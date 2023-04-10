pub trait FileSystemComponent {
    fn get_name(&self) -> &str;
    fn get_size(&self) -> u64;
}

pub struct File {
    name: String,
    size: u64,
}

impl File {
    pub fn new(name: &str, size: u64) -> Self {
        File {
            name: String::from(name),
            size,
        }
    }
}

impl FileSystemComponent for File {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(&self) -> u64 {
        self.size
    }
}

pub struct Directory {
    name: String,
    children: Vec<Box<dyn FileSystemComponent>>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Directory {
            name: String::from(name),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Box<dyn FileSystemComponent>) {
        self.children.push(child);
    }
}

impl FileSystemComponent for Directory {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(&self) -> u64 {
        self.children.iter().map(|child| child.get_size()).sum()
    }
}
