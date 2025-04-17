use std::error::Error;

/**
* File Validation Module
* This module contains functions to validate the data loaded from a CSV file.
* It checks if the days are within the valid range (1-31) and if the values are non-negative.
* @param file: The result of loading the data from the CSV file.
* @return: A boolean indicating whether the data is valid or not.
* @throws: Prints an error message if the data is invalid or if there was an error loading the data.
*/
pub fn file_validation(file: &Result<Vec<(i8, f64)>, Box<dyn Error>>) -> bool {
    match file {
        Ok(data) => {
            for (day, value) in data {
                if *day < 1 || *day > 31 {
                    eprintln!("Invalid day: {}", day);
                    return false;
                }
                if value < &0.0 {
                    eprintln!("Invalid value: {}", value);
                    return false;
                }
            }
            true
        }
        Err(e) => {
            eprintln!("Error loading data: {}", e);
            false
        }
    }
}
