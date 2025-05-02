use crate::dataframe::ColumnVal;
mod dataframe; 
mod plot; 
mod linear;
//purpose of main.rs = loads the csv dataset, filter it by age and gender, visualizes the result through 
// the scatter plot and a regression line 
fn main() {
    let mut df = dataframe::DataFrame::new(); //creates a new Dataframe
    let types = vec![1, 1, 2, 1, 1, 2, 3]; //defines the column types 
    let _ = df.read_csv("health.csv", &types); //reads and parses the csv into the Dataframe

    //first filter: getting the rows where Age Group == All ages
    let first_closure =|x: &ColumnVal|{
        if let ColumnVal::One(age) = x{
           *age == "All ages"
        }else{
            false
        }
    };
    let mut filtered_for_age = df.filter("Age Group", first_closure).unwrap();
    //second filter: getting the rows where its Both gender
    let second_closure = |x: &ColumnVal|{
        if let ColumnVal::One(gender) = x{
           *gender == "Both"
        }else{
            false
        }
    };
    let filtered = filtered_for_age.filter("Sex", second_closure).unwrap();
    filtered.print();
    //plots the filtered data using the specificed columns
    plot::plot_data(&filtered.data, &filtered.labels, "Year", "Death Rate Per 100,000").unwrap();
}

#[cfg(test)]
//purpose = tests the functionality of the read_csv function and linear regression 
mod tests {
    use super::*;
    use crate::linear;
    #[test]
    //checks if it reads the csv & parses the labels 
    fn it_reads(){
        let mut sample_df = dataframe::DataFrame::new();
        let new_types = vec![1, 1, 2, 3];
        let reading = sample_df.read_csv("test_data.csv", &new_types);
        assert!(reading.is_ok());
        assert_eq!(sample_df.labels, vec!["Name", "Subject", "Time Spent Studying", "Score"]);
    }
    #[test]
    //check if the slope and y intercept are accurate
    fn linear_regression_works(){
        let mut sample_df = dataframe::DataFrame::new();
        let new_types = vec![1, 1, 2, 3];
        let _ = sample_df.read_csv("test_data.csv", &new_types);

        let fit_sample = linear::fit_model(&sample_df.data, &sample_df.labels, "Time Spent Studying", "Score");
        assert!((fit_sample.intercept() - 50.0).abs() < 1e-6);
        assert!((fit_sample.params()[0] - 5.0).abs() < 1e-6);
    }
}