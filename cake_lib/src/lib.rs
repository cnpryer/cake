// File
//
// metadata (FileMetadata): Stored metedata from a file target
// data (FileData): Stored data from a file target
pub struct File {
    metadata: FileMetadata,
    data: FileData<String>
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
    filetype: String
}

// FileData
//
// data (Vec<String>): Loaded file data. TODO: move away from current data types.
struct FileData<String> { 
    data: Vec<String>
}

// File
//
// Initialized File from `path` of target file.
impl File {

    // Create file from filepath
    pub fn new(path: &String) -> File {
        let metadata = init_file_metadata_from_filepath(&path);
        let data = FileData { data: vec![] };

        File { metadata, data }
    }

    // get path to file
    pub fn get_path(&self) -> &String {
       &self.metadata.path
    }

    // get size of file
    pub fn get_size(&self) -> &u32 {
      &self.metadata.size
     }

    // get name of file
    pub fn get_name(&self) -> &String {
        &self.metadata.name
    }

    // get file's type
    pub fn get_type(&self) -> &String {
        &self.metadata.filetype
    }

    // print file metadata such as path, size, type
    pub fn print_metadata(self) {
        println!("Filepath: {}", self.get_path());
        println!("Filename: {}", self.get_name());
        println!("Size: {}", self.get_size());
    }

    // print head of file data
    pub fn print_head(self) {
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
fn init_file_metadata_from_filepath(path: &String) -> FileMetadata {
    let name = get_filename_from_path(&path);

    if name.is_none() {
        panic!("Invalid filename");
    }

    let filetype = get_filetype_from_path(&path);

    if filetype.is_none() {
        panic!("Ivalid filetype");
    }
    
    FileMetadata {path: path.clone(), name: name.unwrap(), size: 0, filetype: filetype.unwrap()}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_file() {
        let path = "./test/file.csv".to_string();
        let file = File::new(&path);
        
        assert_eq!(*file.get_name(), "file.csv");
        assert_eq!(*file.get_size(), 0);
        assert_eq!(*file.get_path(), path);
    }
}
