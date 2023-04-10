use crate::structural::composite::{Directory, File, FileSystemComponent};

#[test]
fn folder_size_test() {
    let mut root = Directory::new("root");
    let file1 = File::new("file1.txt", 100);
    let file2 = File::new("file2.txt", 200);

    let mut sub_directory = Directory::new("sub_directory");
    let file3 = File::new("file3.txt", 300);

    sub_directory.add_child(Box::new(file3));
    root.add_child(Box::new(file1));
    root.add_child(Box::new(file2));
    root.add_child(Box::new(sub_directory));

    assert_eq!(root.get_size(), 600);
}
