// cake_lib
//
// The cake library provides primatives for flat file operations.
// Use cake FileObjects to query information from files. Perform actions like
// querying to view data from your file.
pub mod file;
pub mod frame;

use file::File;
use frame::Frame;

#[allow(dead_code)]
// read csv to dataframe from path ref
fn read_csv(path: &String) -> frame::DataFrame {
    let obj = file::FileObject::new(path);
    let data = obj.get_data();
    let df = frame::DataFrame::new(&data);

    df
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let path = "../testing/test_data.csv".to_string();
        let df = read_csv(&path);

        assert_eq!(df.len, 3);
        assert_eq!(df.width, 3);
    }
}
