use std::process;
use crate::dataframe::ColumnVal;
mod dataframe; 

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
}
