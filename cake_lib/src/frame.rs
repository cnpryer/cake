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
    fn len(&self) -> usize;
    fn width(&self) -> usize;
}

pub struct DataFrame {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Frame for DataFrame {
    fn new(data: &Vec<String>) -> DataFrame {
        let mut rows: Vec<Vec<String>> = Vec::new();

        if data.len() < 1 {
            return DataFrame {
                header: Vec::new(),
                rows: rows,
            };
        }

        let header = data[0].split(",").map(|x| x.to_string()).collect();

        for r in data[1..].iter() {
            rows.push(r.split(",").map(|x| x.to_string()).collect())
        }

        DataFrame { header, rows }
    }

    // get the number of rows in the dataframe
    fn len(&self) -> usize {
        self.rows.len()
    }

    // get the number of columns in the dataframe
    fn width(&self) -> usize {
        self.header.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataframe() {
        let data = vec!["a,b,c".to_string()];
        let df = DataFrame::new(&data);

        assert_eq!(df.width(), 3);
        assert_eq!(df.len(), 0)
    }
}
