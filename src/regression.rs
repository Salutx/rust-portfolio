use std::error::Error;

/**
* This module provides a function to perform linear regression on a dataset.
* It calculates the slope and intercept of the best-fit line using the least squares method.
* @param file: The result of loading the data from the CSV file.
* @return: A tuple containing the slope and intercept of the best-fit line.
* @throws: Returns an error if the data is invalid or if there was an error loading the data.
*/
pub fn regression(data: &Vec<(i8, f64)>) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    let n = data.len() as f64;

    if n == 0.0 {
        return Err("No data points".into());
    }

    let sum_x: f64 = data.iter().map(|(x, _)| *x as f64).sum();
    let sum_y: f64 = data.iter().map(|(_, y)| *y).sum();
    let sum_xy: f64 = data.iter().map(|(x, y)| *x as f64 * *y).sum();
    let sum_x_squared: f64 = data.iter().map(|(x, _)| (*x as f64).powi(2)).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x_squared - sum_x.powi(2));
    let intercept = (sum_y - slope * sum_x) / n;

    // Calcular o R² e o MSE
    let y_hat: Vec<f64> = data
        .iter()
        .map(|(x, _)| slope * *x as f64 + intercept)
        .collect();
    let ss_total: f64 = data.iter().map(|(_, y)| (*y - sum_y / n).powi(2)).sum();
    let ss_residual: f64 = data
        .iter()
        .zip(y_hat.iter())
        .map(|((_, y), y_hat)| (*y - *y_hat).powi(2))
        .sum();

    let r_squared = 1.0 - ss_residual / ss_total;
    let mse = ss_residual / n;

    // Limitar o slope e intercept para 2 casas decimais
    let slope_fixed = (slope * 100.0).round() / 100.0;
    let intercept_fixed = (intercept * 100.0).round() / 100.0;

    Ok((slope_fixed, intercept_fixed, r_squared, mse))
}
