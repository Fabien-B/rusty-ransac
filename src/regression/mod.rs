
use std::fs;
use std::error::Error;
use nalgebra::base::{Matrix, U1, DVector, DMatrix, dimension::Dynamic};
// use serde::Deserialize;
use super::
use gnuplot::{Figure, AxesCommon, Caption, LineWidth, AutoOption, Coordinate};

// #[derive(Debug, Clone, Deserialize)]
// pub struct Point {
//     pub x: f64,
//     pub y: f64
// }

pub fn get_betas(points: &Vec<Point>, order: usize) -> DVector<f64>{
    let (x, y) = get_x_y(&points, order);
    linear_regression(x, y)
}

pub fn linear_regression(x: DMatrix<f64>, y: DVector<f64>) -> DVector<f64> {
    let svd = x.svd(true, true);
    let (u, v, s) = (svd.u.unwrap(), svd.v_t.unwrap().transpose(), svd.singular_values);

    let alpha = u.transpose() * y;

    let mdata = alpha.iter().zip(s.iter()).map(|(ai, si)| ai/si).collect::<Vec<f64>>();

    let sinv_alpha = DVector::from_vec(mdata);
    let betas = v * sinv_alpha;
    betas
}

pub fn parse_points(filename: &str) -> Result<Vec<Point>, Box<Error>> {
    let file = (fs::File::open(filename))?;
    let mut points = vec![];
    let mut reader = csv::Reader::from_reader(file);
        for result in reader.deserialize() {
        let record: Point = result?;
        points.push(record);
        }
        Ok(points)
}

pub fn get_x_y(points: &Vec<Point>, order: usize) -> (DMatrix<f64>, DVector<f64>) {
    let mut xs:Vec<Matrix<f64, U1, Dynamic, _>> = Vec::new();
    let mut ys:Vec<f64> = Vec::new();

    for point in points {
        xs.push(DVector::from_vec((0..order).map(|i| point.x.powi(i as i32)).collect::<Vec<_>>()).transpose());
        ys.push(point.y);
    }

    let y = DVector::from_vec(ys);
    let x = DMatrix::from_rows(&xs[..]);

    (x, y)
}

pub fn get_error(betas: &DVector<f64>, points: &Vec<Point>) -> f64 {
    let line = |x: f64| betas.iter().enumerate().fold(0.0, |sum, (i, b)| sum + b * x.powi(i as i32));

    points.iter().fold(0.0, |acc, p| acc + (line(p.x) - p.y).powi(2 as i32)) / points.len() as f64
}

#[allow(dead_code)]
pub fn plot_stuff(betas: &DVector<f64>, points: &Vec<Point>) {
    plot_stuff_label(betas, points, "");
}

pub fn plot_stuff_label(betas: &DVector<f64>, points: &Vec<Point>, label: &str) {

    let line = |x: f64| betas.iter().enumerate().fold(0.0, |sum, (i, b)| sum + b * x.powi(i as i32));

    //TODO: fix this 0-centered thing (center it on data, not 0)
    let max_x = points.iter().fold(0.0, |max, p| p.x.max(max)) + 1.0;
    let min_x = points.iter().fold(0.0, |min, p| p.x.min(min)) - 1.0;

    // make a vector between min_x and max_x in steps of 0.1
    let mut x_steps = vec![];
    for i in (min_x as i32)..(max_x as i32) {
        for j in 0..10 {
            x_steps.push((i as f64) + 0.1 * (j as f64));
        }
    }
    
    let y_steps = x_steps.iter().map(|x| line(*x) ).collect::<Vec<_>>();

    let mut fig = Figure::new();
    fig.axes2d()
    .points(points.iter().map(|p| p.x), 
                        points.iter().map(|p| p.y), 
                        &[Caption("Datapoints"), LineWidth(1.5)])
        .set_x_range(AutoOption::Auto, AutoOption::Auto)
        .set_y_range(AutoOption::Auto, AutoOption::Auto)
        .set_x_label("x", &[])
        .set_y_label("y", &[])
        .label(label, Coordinate::Graph(0.5), Coordinate::Graph(0.9), &[])
        .lines(x_steps, y_steps, &[Caption("Regression")]);
    fig.show();
}
