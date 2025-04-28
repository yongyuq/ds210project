use std::error::Error;
use std::fmt;
use std::process;

#[derive(Debug, Clone)]
pub enum ColumnVal {
    One(String),
    Two(u32),
    Three(f64),
}

#[derive(Debug)]
pub struct DataFrame {
    pub labels: Vec<String>,
    pub data: Vec<Vec<ColumnVal>>,
    pub types: Vec<u32>, 
}

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

impl DataFrame {
    pub fn new() -> Self {
        let labels: Vec<String> = Vec::new();
        let data: Vec<Vec<ColumnVal>> = Vec::new();
        let types: Vec<u32> = Vec::new();
        DataFrame {
            labels,
            data,
            types,
        }
    }

    pub fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;
        let mut first_row = true;
        for result in rdr.records() {
            let r = result.unwrap();
            let mut row: Vec<ColumnVal> = vec![];
            if first_row {
                for elem in r.iter() {
                    self.labels.push(elem.to_string());
                }
                first_row = false;
                continue;
            }
            for (i, elem) in r.iter().enumerate() {
                let elem = elem.replace(",", "");
                match types[i] {
                    1 => row.push(ColumnVal::One(elem.to_string())),
                    2 => row.push(ColumnVal::Two(elem.parse::<u32>().unwrap())),
                    3 => row.push(ColumnVal::Three(elem.parse::<f64>().unwrap())),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }
            }
            self.data.push(row);
        }
        Ok(())
    }

    pub fn print(&self) {
        // print the labels
        for label in &self.labels {
            print!("{:>15} ", label);
        }
        println!();
        // print the data
        for row in &self.data {
            for val in row {
                match val {
                    ColumnVal::One(s) => print!("{:>15} ", s),
                    ColumnVal::Two(u) => print!("{:>15} ", u),
                    ColumnVal::Three(f) => print!("{:>15} ", f),
                }
            }
            println!();
        }
    }

    pub fn filter(
        &mut self,
        label: &str,
        operation: fn(&ColumnVal) -> bool,
    ) -> Result<Self, Box<dyn Error>> {
        let mut index = 0;
        for (i, current) in self.labels.iter().enumerate(){
            if current == label{
                index = i;
            }
        }
        let filtered_data: Vec<Vec<ColumnVal>> = self.data.iter().filter(|row| operation(&row[index])).cloned().collect();

        let new = DataFrame {
            labels: self.labels.clone(),
            data: filtered_data, 
            types: self.types.clone(), 
        };
        Ok(new)
    }
}
