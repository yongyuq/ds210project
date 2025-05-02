use plotters::prelude::*;
use crate::dataframe::ColumnVal;
use crate::linear::fit_model;
//purpose = generates a scatter plot with regression line from the data using plotters

pub fn plot_data(
    data: &Vec<Vec<ColumnVal>>, 
    labels: &Vec<String>, 
    x_col: &str, //name of column for x axis
    y_col: &str //name of column for y axis
) -> Result<(), Box<dyn std::error::Error>> { //returns a result to see if it's successful or not during plotting
    //sets up the drawing area and the chart dimensions
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10,10,10,10);
    //builds the chart with a title and also the ranges for the axis
    let mut chart = ChartBuilder::on(&root)
        .caption("Death Rates in Each Country of All Ages and Both Both Gender from 1970 - 2010", ("Arial", 15).into_font())
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(1970f64..2010f64, 0f64..3500f64)?; 
    //my range for x axis is the years (1970-2010) & for y axis, it is from 0 to the max of the death rate
    //setting up the mesh & axis labels for the chart
    chart.configure_mesh()
        .x_desc("Years")
        .y_desc("Death Rate Per 100,000")
        .draw()?;
    //finding the index of the x and y columns 
    let x_value = labels.iter().position(|label| label == x_col).expect("Column not found.");
    let y_value = labels.iter().position(|label| label == y_col).expect("Column not found.");
    let mut points: Vec<(f64, f64)> = Vec::new();
    //extracts & collects the (x,y) pairs into a vec of f64
    for row in data{
        let x = &row[x_value];
        let y = &row[y_value];
        match(x,y){ //matches the two values 
            (ColumnVal::Two(x), ColumnVal::Three(y)) => {
                points.push((*x as f64, *y as f64));
            }
            _ => {
                continue
            }
        }
    }
    //draws the scatter plot
    chart.draw_series(
        points.into_iter().map(|(x,y)| Circle::new((x,y), 3, RED.filled()))
    )?;
    //fits the linear regression model to the data 
    let model = fit_model(data, labels, x_col, y_col);
    let slope = model.params()[0]; //this is the slope
    let intercept = model.intercept(); //this is the y-intercept
    let mut line_points = Vec::with_capacity(2); //creates a mut vec to store (x,y) coordinates
    for i in (1970i32..=2010i32).step_by(10){  //range from 1970 - 2010 inclusive, incrementing by 10 so it goes like 1970, 1980..
        line_points.push((i as f64, (i as f64 * slope + intercept)));
    }
    let precision = 2; //precision of 2 decimal places for the slope and y-intercept 
    //formating the regression line equation
    let label = format!( 
        "y = {:.2$}x + {:.2}",
        model.params()[0],
        model.intercept(),
        precision
    );
    //printing the linear regression equation
    println!("{}", label);
    //draws the line through LineSeries using the line points 
    chart.draw_series(LineSeries::new(line_points, &BLACK))
        .unwrap()
        .label(&label);

    Ok(()) 
}