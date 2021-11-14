// File
//
// Cake Files define file data structures and interfaces for interacting
// with your files.
//
// The FileObject
//
// FileObjects are used to load files into interactive data structures. FileObjects
// manage the file's data and metadata. Valid FileObjects are initialized with an
// existing file with a compatible *filetype*.
//
// Compatible *filetypes*: csv
use std::fs;

const VALID_FILETYPE: &str = "csv";

// The File
trait File {
    fn new(path: &String) -> FileObject;
    fn is_valid(&self) -> bool;
    fn validate(&self);
    fn get_metadata(&self) -> &FileMetadata;

    // TODO: display lib
    fn print_metadata(&self);
    fn print_head(&self);
}

// File
//
// metadata (FileMetadata): Stored metedata from a file target
// data (FileData): Stored data from a file target
pub struct FileObject {
    metadata: FileMetadata,
    data: FileData<String>,
}

// FileMetadata
//
// path (String): Full path to file
// name (String): Filename
// size (String): Stored file size
// filetype (String): Type of file (i.e "csv")
struct FileMetadata {
    path: String,
    name: String,
    size: u32,
    filetype: String,
    created_date: u32,
    last_modified_date: u32,
}

// FileData
//
// data (Vec<String>): Loaded file data. TODO: move away from current data types.
struct FileData<String> {
    data: Vec<String>,
}

// File
//
// Initialized File from `path` of target file.
impl File for FileObject {
    // Create file from filepath
    fn new(path: &String) -> FileObject {
        let metadata = load_metadata(&path);
        let data = load_data(&path);

        FileObject { metadata, data }
    }

    // return true if file is valid
    fn is_valid(&self) -> bool {
        &self.metadata.filetype == VALID_FILETYPE
    }

    // validate file
    fn validate(&self) {
        if !self.is_valid() {
            panic!("File is invalid");
        }

        assert!(true)
    }

    // get file metadata
    fn get_metadata(&self) -> &FileMetadata {
        &self.metadata
    }

    // print file metadata such as path, size, type
    fn print_metadata(&self) {
        let (path, name, size, created_date, last_modified_date) = (
            &self.metadata.path,
            &self.metadata.name,
            &self.metadata.size,
            &self.metadata.created_date,
            &self.metadata.last_modified_date,
        );

        println!("Filepath: {}", path);
        println!("Filename: {}", name);
        println!("Size: {}", size);
        println!("Last Modified: {}", last_modified_date);
        println!("Created: {}", created_date);
    }

    // print head of file data
    fn print_head(&self) {
        println!("Head: {:?}", &self.data.data[0..5]);
    }
}

// parses target filename from path
fn get_filename_from_path(path: &String) -> Option<String> {
    let split = path.split("/").map(|s| s.to_string());
    let filename = split.last();

    return filename;
}

// parses target filename's extension for filetype
fn get_filetype_from_path(path: &String) -> Option<String> {
    let filename = get_filename_from_path(path);
    let filetype = filename.unwrap().split(".").map(|s| s.to_string()).last();

    return filetype;
}

// generates metadata from a target filepath
fn load_metadata(path: &String) -> FileMetadata {
    let name = get_filename_from_path(&path);

    if name.is_none() {
        panic!("Invalid filename");
    }

    let filetype = get_filetype_from_path(&path);

    if filetype.is_none() {
        panic!("Ivalid filetype");
    }

    FileMetadata {
        path: path.clone(),
        name: name.unwrap(),
        size: 0,
        filetype: filetype.unwrap(),
        created_date: 0,
        last_modified_date: 0,
    }
}

// generate file data from a target filepath
fn load_data(path: &String) -> FileData<String> {
    let contents =
        fs::read_to_string(path.as_str()).expect("Something went wrong reading the file");
    let data = contents.split("/n").map(|s| s.to_string()).collect();

    FileData { data }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_file() {
        let path = "../testing/test_data.csv".to_string();
        let file = FileObject::new(&path);

        let metadata = file.get_metadata();

        assert_eq!(metadata.name, "test_data.csv");
        assert_eq!(metadata.size, 0);
        assert_eq!(metadata.path, path);
        assert!(file.is_valid());
    }
}
