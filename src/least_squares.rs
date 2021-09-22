use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Line {
    pub slope: f64,
    pub intersect: f64,
}

pub fn find_best_line(points: &Vec<Point>) -> Result<Line, String> {
    if points.len() == 0 {
        return Err("Unable to approximate empty dataset.".to_owned());
    }

    let n = points.len() as f64;
    let x_sum: f64 = points.iter().map(|p| p.x).sum();
    let y_sum: f64 = points.iter().map(|p| p.y).sum();
    let xy_sum: f64 = points.iter().map(|p| p.x * p.y).sum();
    let xx_sum: f64 = points.iter().map(|p| p.x * p.x).sum();

    if n * xx_sum == x_sum * x_sum {
        return Err("Dataset can be approximated with infinite amount of \"best\" lines.".to_owned());
    }

    let slope = (n * xy_sum - x_sum * y_sum) / (n * xx_sum - x_sum * x_sum);
    let intersect = (y_sum - slope * x_sum) / n;

    Ok(Line{slope: slope, intersect: intersect})
}
