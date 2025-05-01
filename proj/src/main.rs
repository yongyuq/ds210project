use std::process;
use crate::dataframe::ColumnVal;
mod dataframe; 
mod plot; 
mod linear;

fn main() {
    let mut df = dataframe::DataFrame::new();
    let types = vec![1, 1, 2, 1, 1, 2, 3];
    let _ = df.read_csv("health.csv", &types);
    //df.print();
   // println!();

    let first_closure =|x: &ColumnVal|{
        if let ColumnVal::One(age) = x{
           *age == "All ages"
        }else{
            false
        }
    };
    let mut filtered_for_age = df.filter("Age Group", first_closure).unwrap();
    filtered_for_age.print();
    println!();

    println!("Filtered all");
    let second_closure = |x: &ColumnVal|{
        if let ColumnVal::One(gender) = x{
           *gender == "Both"
        }else{
            false
        }
    };
    let mut filtered = filtered_for_age.filter("Sex", second_closure).unwrap();
    filtered.print();

    let graph = plot::plot_data(&filtered.data, &filtered.labels, "Year", "Death Rate Per 100,000").unwrap();
    graph;

    let linear_reg = linear::fit_model(&filtered.data, &filtered.labels, "Year", "Death Rate Per 100,000");
    linear_reg;

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_reads(){
        let mut sample_df = dataframe::DataFrame::new();
        let new_types = vec![1, 1, 2, 2];
        let reading = sample_df.read_csv("test.csv", &new_types);
        assert!(reading.is_ok());
        assert_eq!(sample_df.labels, vec!["Name", "Subject", "Time Spent Studying", "Score"]);
    }

    fn linear_regression_works(){
        let mut sample_df = dataframe::DataFrame::new();
        let new_types = vec![1, 1, 2, 2];
        let _ = sample_df.read_csv("test.csv", &new_types);

        let fit_sample = linear::fit_model(&sample_df.data, &sample_df.labels, "Time Spent Studying", "Score");
        assert_eq!(fit_sample.params()[0], 5.0);
        assert_eq!(fit_sample.intercept(), 50.0);
    }
}