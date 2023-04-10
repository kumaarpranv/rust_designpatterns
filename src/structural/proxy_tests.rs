use crate::structural::proxy::{FileReaderProxy, FileReader};

#[test]
fn test_read_proxy() {
    let file_path = String::from("/home/pranav/test.txt");
    let mut file_reader_proxy = FileReaderProxy::new(file_path);

    // The file will only be read when it's needed
    // println!("File contents: {}", file_reader_proxy.read_contents());
    // The contents are cached, so the file won't be read again
    // println!("File contents: {}", file_reader_proxy.read_contents());

    assert_eq!(file_reader_proxy.read_contents(), "hello\n")
}
