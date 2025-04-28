use ndarray::Array1;
use ndarray::Array2;
use linfa::Dataset;
use linfa_linear::LinearRegression;
use linfa_linear::FittedLinearRegression;
use linfa::prelude::*;
use crate::dataframe::ColumnVal;


pub fn fit_model(
    data: &Vec<Vec<ColumnVal>>, 
    labels: &Vec<String>,
    x_name: &str,
    y_name: &str,
) -> FittedLinearRegression<f64> {
    
    let x_val = labels.iter().position(|label| label == x_name).expect("Column not found.");
    let y_val = labels.iter().position(|label| label == y_name).expect("Column not found.");
    let mut x_points:Vec<f64> = Vec::new();
    let mut y_points:Vec<f64>  = Vec::new();
    //let mut points: Vec<(f64, f64)> = Vec::new();
    for row in data{
        let x = &row[x_val];
        let y = &row[y_val];
        match(x,y){
            (ColumnVal::Two(x), ColumnVal::Three(y)) => {
                x_points.push(*x as f64);
                y_points.push(*y);
                //points.push((*x as f64, *y as f64));
            }
            _ => {
                continue
            }
        }
    }
    let x_data = Array2::from_shape_vec((x_points.len(), 1), x_points).unwrap();
    let y_data = Array1::from_vec(y_points);
    let dataset = Dataset::new(x_data, y_data);
    let lin_reg: LinearRegression = LinearRegression::new();
    let model: FittedLinearRegression<f64> = lin_reg.fit(&dataset).unwrap();
    let ypred = model.predict(&dataset);
    let loss = (dataset.targets() - ypred)
        .mapv(|x| x.abs())
        .mean();

    println!("{:?}", loss);
    return model;
}