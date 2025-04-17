use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * Loads data from a CSV file and returns a vector of tuples.
 * Each tuple contains a day (i8) and a value (f64).
 * @param file_path: The path to the CSV file.
 * @return: A Result containing a vector of tuples or an error.
 */
pub fn load_data(file_path: &str) -> Result<Vec<(i8, f64)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        };

        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 2 {
            continue;
        }

        let day: i8 = parts[0].parse()?;
        let value: f64 = parts[1].parse()?;
        data.push((day, value));
    }

    Ok(data)
}
