use std::fs;
use csv::ReaderBuilder;

use super::least_squares::Point;

pub fn load_dataset(path: &String) -> Result<Vec<Point>, String> {
    match fs::read_to_string(path) {
        Ok(src) => {
            let mut points = Vec::new();
            let mut reader = ReaderBuilder::new().from_reader(src.as_bytes());
            for result in reader.deserialize::<Point>() {
                match result {
                    Ok(point) => points.push(point),
                    Err(e) => return Err(format!("Unable to parse line in file: {}\n", e))
                }
            }
            Ok(points)
        },
        Err(e) => Err(format!("Unable to read file {}: {}\n", path, e))
    }
}
