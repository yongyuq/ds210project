use plotters::prelude::*;
use crate::dataframe::ColumnVal;
use crate::linear::fit_model;

pub fn plot_data(
    data: &Vec<Vec<ColumnVal>>, 
    labels: &Vec<String>, 
    x_col: &str, 
    y_col: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10,10,10,10);

    let mut chart = ChartBuilder::on(&root)
        .caption("Death Rates in Each Country of All Ages and Both Both Gender from 1970 - 2010", ("Arial", 15).into_font())
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(1970f64..2010f64, 0f64..3500f64)?;

    chart.configure_mesh()
        .x_desc("Years")
        .y_desc("Death Rate Per 100,000")
        .draw()?;

    let x_value = labels.iter().position(|label| label == x_col).expect("Column not found.");
    let y_value = labels.iter().position(|label| label == y_col).expect("Column not found.");
    let mut points: Vec<(f64, f64)> = Vec::new();
    for row in data{
        let x = &row[x_value];
        let y = &row[y_value];
        match(x,y){
            (ColumnVal::Two(x), ColumnVal::Three(y)) => {
                points.push((*x as f64, *y as f64));
            }
            _ => {
                continue
            }
        }
    }
    chart.draw_series(
        points.into_iter().map(|(x,y)| Circle::new((x,y), 3, RED.filled()))
    )?;

    let model = fit_model(data, labels, x_col, y_col);
    let slope = model.params()[0];
    let intercept = model.intercept();
    let mut line_points = Vec::with_capacity(2);
    for i in (1970i32..=2010i32).step_by(10){
        line_points.push((i as f64, (i as f64 * slope + intercept)));
    }
    let precision = 2;
    let label = format!(
        "y = {:.2$}x + {:.2}",
        model.params()[0],
        model.intercept(),
        precision
    );
    chart.draw_series(LineSeries::new(line_points, &BLACK))
        .unwrap()
        .label(&label);

    Ok(())
}