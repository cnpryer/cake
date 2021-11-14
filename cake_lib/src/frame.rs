// frame
//
// `cake` frames are dataframes used for running actions on interactable data.
//
// Example:
// | a | b | c |
// | 1 | 2 | 3 |
// | 4 | 5 | 6 |
//
// len() -> usize
// count of rows
//
// width() -> usize
// count of columns
trait Frame {
    fn new(data: &Vec<String>) -> DataFrame;
    fn get_header(&self) -> &Vec<String>;
    fn get_rows(&self) -> &Vec<Vec<String>>;
}

pub struct DataFrame {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    pub len: usize,
    pub width: usize,
}

impl Frame for DataFrame {
    fn new(data: &Vec<String>) -> DataFrame {
        let mut rows: Vec<Vec<String>> = Vec::new();

        if data.len() < 1 {
            return DataFrame {
                header: Vec::new(),
                rows: rows,
                len: 0,
                width: 0,
            };
        }

        let header: Vec<String> = data[0].split(",").map(|x| x.to_string()).collect();

        for r in data[1..].iter() {
            rows.push(r.split(",").map(|x| x.to_string()).collect())
        }

        let len = rows.len();
        let width = header.len();

        DataFrame {
            header,
            rows,
            len,
            width,
        }
    }

    // get header from datframe
    fn get_header(&self) -> &Vec<String> {
        &self.header
    }

    // get rows from dataframe
    fn get_rows(&self) -> &Vec<Vec<String>> {
        &self.rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataframe() {
        let data = vec!["a,b,c".to_string()];
        let df = DataFrame::new(&data);

        assert_eq!(df.width, 3);
        assert_eq!(df.len, 0)
    }
}
