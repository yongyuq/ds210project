use crate::dataframe::ColumnVal;
mod dataframe; 
mod plot; 
mod linear;

fn main() {
    let mut df = dataframe::DataFrame::new();
    let types = vec![1, 1, 2, 1, 1, 2, 3];
    let _ = df.read_csv("health.csv", &types);

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
    let filtered = filtered_for_age.filter("Sex", second_closure).unwrap();
    filtered.print();

    plot::plot_data(&filtered.data, &filtered.labels, "Year", "Death Rate Per 100,000").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear;
    #[test]
    fn it_reads(){
        let mut sample_df = dataframe::DataFrame::new();
        let new_types = vec![1, 1, 2, 3];
        let reading = sample_df.read_csv("test_data.csv", &new_types);
        assert!(reading.is_ok());
        assert_eq!(sample_df.labels, vec!["Name", "Subject", "Time Spent Studying", "Score"]);
    }
    #[test]
    fn linear_regression_works(){
        let mut sample_df = dataframe::DataFrame::new();
        let new_types = vec![1, 1, 2, 3];
        let _ = sample_df.read_csv("test_data.csv", &new_types);

        let fit_sample = linear::fit_model(&sample_df.data, &sample_df.labels, "Time Spent Studying", "Score");
        assert!((fit_sample.intercept() - 50.0).abs() < 1e-6);
        assert!((fit_sample.params()[0] - 5.0).abs() < 1e-6);
    }
}