/**
 * Author: Lucas Matos
 * Curso: Análise e Desenvolvimento de Sistemas
 */
use prediction::prediction;

mod helpers;
mod load_data;
mod prediction;
mod regression;

const FILE_PATH: &str = "examples/mocked_data.csv";
const X_VALUE: f64 = 11.0;

fn main() {
    let loaded_data = load_data::load_data(FILE_PATH);

    if !helpers::file_validation::file_validation(&loaded_data) {
        eprintln!("Data validation failed.");
        return;
    }

    match loaded_data {
        Ok(data) => match regression::regression(&data) {
            Ok((slope, intercept, r_squared, mse)) => {
                let prediction_fixed =
                    (prediction(intercept, slope, X_VALUE) * 100.0).round() / 100.0;

                println!("=================================");
                println!("  Dados carregados com sucesso!");
                println!("=================================");
                println!("-> Inclinação (slope): {:.2}", slope);
                println!("-> Intercepto (intercept): {:.2}", intercept);
                println!(
                    "-> Previsão para o dia {}: {:.2}",
                    X_VALUE, prediction_fixed
                );
                println!("-> MSE: {:.2}", mse);
                println!("-> R²: {:.2}", r_squared);
                println!("================================");
            }
            Err(e) => {
                eprintln!("Error performing regression: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Error loading data: {}", e);
        }
    }
}
