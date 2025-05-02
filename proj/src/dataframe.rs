use std::error::Error;
use std::fmt;
//purpose = creates a Dataframe: reading the csv, filtering, and printing

#[derive(Debug, Clone)]
pub enum ColumnVal { //allows multiple types of data to be stored 
    One(String),  //represents string values
    Two(u32), //represents u32 values
    Three(f64), //represents f64 values
}

#[derive(Debug)]
pub struct DataFrame { //keeping related data together, a structure with column labels, data, and types
    pub labels: Vec<String>,  //names of each column
    pub data: Vec<Vec<ColumnVal>>,  //data stored as rows of ColumnVal values
    pub types: Vec<u32>,  //types of each column
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
    pub fn new() -> Self {   //creates a new & empty Dataframe
        let labels: Vec<String> = Vec::new();
        let data: Vec<Vec<ColumnVal>> = Vec::new();
        let types: Vec<u32> = Vec::new();
        DataFrame {
            labels,
            data,
            types,
        }
    }
    //reads the csv for the Dataframe
    //the types vector defines the expected type for the columns
    //returns ok(()) if successful, otherwise an Error 
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
                    self.labels.push(elem.to_string());  //stores the column labels
                }
                first_row = false;
                continue;
            }
            //processing the values based on the column type
            for (i, elem) in r.iter().enumerate() {
                let elem = elem.replace(",", "");
                match types[i] {
                    1 => row.push(ColumnVal::One(elem.to_string())),
                    2 => row.push(ColumnVal::Two(elem.parse::<u32>().unwrap())),
                    3 => row.push(ColumnVal::Three(elem.parse::<f64>().unwrap())),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }
            }
            self.data.push(row); //adds to the Dataframe
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
    //filtering the Dataframe based on operation applied to values in a specific column
    pub fn filter(
        &mut self,
        label: &str, //name of the column to filter
        operation: fn(&ColumnVal) -> bool, //the function that will be applied to the values 
    ) -> Result<Self, Box<dyn Error>> {
        //finding index of the column that match the label
        let mut index = 0; 
        for (i, current) in self.labels.iter().enumerate(){
            if current == label{
                index = i;
            }
        }
        //applying the operation to each row in that specified column & keeping ones that return true
        let filtered_data: Vec<Vec<ColumnVal>> = self.data.iter().filter(|row| operation(&row[index])).cloned().collect();
        //creates a new Dataframe with filtered data but same types & labels
        let new = DataFrame {
            labels: self.labels.clone(),
            data: filtered_data, 
            types: self.types.clone(), 
        };
        Ok(new)
    }
}
