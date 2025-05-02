use ndarray::Array1;
use ndarray::Array2;
use linfa::Dataset;
use linfa_linear::LinearRegression;
use linfa_linear::FittedLinearRegression;
use linfa::prelude::*;
use crate::dataframe::ColumnVal;
//purpose = implements linear regression on the Dataframe data 

//fits a linear regression model using specified columns from the data 
pub fn fit_model(
    data: &Vec<Vec<ColumnVal>>, 
    labels: &Vec<String>, 
    x_name: &str, //name of column for x variable 
    y_name: &str, //name of column for y variable 
) -> FittedLinearRegression<f64> { //returns a FittedLinearRegression<f64> model
    
    //finds the index of the x and y column names 
    let x_val = labels.iter().position(|label| label == x_name).expect("Column not found.");
    let y_val = labels.iter().position(|label| label == y_name).expect("Column not found.");
    let mut x_points:Vec<f64> = Vec::new();
    let mut y_points:Vec<f64>  = Vec::new();
    //gets the values from each row that match the types 
    for row in data{
        let x = &row[x_val];
        let y = &row[y_val];
        match(x,y){
            (ColumnVal::Two(x), ColumnVal::Three(y)) => {
                x_points.push(*x as f64); //casts as f64 so it's consistent with the other values
                y_points.push(*y);
            }
            _ => {
                continue
            }
        }
    }
    //converting it into ndarray which Linfa expects 
    let x_data = Array2::from_shape_vec((x_points.len(), 1), x_points).unwrap();
    let y_data = Array1::from_vec(y_points);
    //creates a dataset & fits a linear regression model to it
    let dataset = Dataset::new(x_data, y_data);
    let lin_reg: LinearRegression = LinearRegression::new();
    let model: FittedLinearRegression<f64> = lin_reg.fit(&dataset).unwrap();

    return model;
}