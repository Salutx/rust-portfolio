/**
* This module contains the function to calculate the prediction of the linear regression model.
* It takes the intercept and slope of the regression line and a value x to predict y.
* @param intercepto: The y-intercept of the regression line.
* @param slope: The slope of the regression line.
* @param x: The x value for which to predict y.
* @return: The predicted y value.
* @throws: Panics if x is not in the range of 1 to 31, or if slope or intercept are negative.
*/
pub fn prediction(intercepto: f64, slope: f64, x: f64) -> f64 {
    if x < 1.0 || x > 31.0 {
        panic!("O valor de x deve estar entre 1 e 31.");
    }
    if slope < 0.0 {
        panic!("A inclinação (slope) não pode ser negativa.");
    }
    if intercepto < 0.0 {
        panic!("O intercepto (intercept) não pode ser negativo.");
    }

    let y = intercepto + (slope * x);
    let y_fixed = (y * 100.0).round() / 100.0;
    y_fixed
}
