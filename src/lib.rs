mod prediction;
mod regression;

pub use prediction::prediction;
pub use regression::regression;

#[cfg(test)]
mod regression_tests {
    use super::regression::regression;

    #[test]
    fn test_regression_basic_case() {
        let data = vec![(1, 2.0), (2, 4.0), (3, 6.0), (4, 8.0), (5, 10.0)];

        let result = regression(&data).unwrap();
        let (slope, intercept, r_squared, mse) = result;

        assert_eq!(slope, 2.0);
        assert_eq!(intercept, 0.0);
        assert!((r_squared - 1.0).abs() < 1e-6);
        assert!((mse - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_regression_rounding() {
        let data = vec![(1, 1.01), (2, 2.01), (3, 2.98), (4, 4.02), (5, 5.01)];

        let result = regression(&data).unwrap();
        let (slope, intercept, _r_squared, _mse) = result;

        assert_eq!(slope, (slope * 100.0).round() / 100.0);
        assert_eq!(intercept, (intercept * 100.0).round() / 100.0);
    }

    #[test]
    fn test_regression_empty_data() {
        let data: Vec<(i8, f64)> = vec![];
        let result = regression(&data);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod prediction_tests {
    use super::prediction::prediction;

    #[test]
    fn test_prediction_valid_input() {
        let intercept = 2.0;
        let slope = 3.0;
        let x = 4.0;
        let y = prediction(intercept, slope, x);
        assert_eq!(y, 14.0);
    }

    #[test]
    #[should_panic(expected = "O valor de x deve estar entre 1 e 31.")]
    fn test_prediction_x_out_of_bounds() {
        prediction(1.0, 1.0, 0.0);
    }

    #[test]
    #[should_panic(expected = "A inclinação (slope) não pode ser negativa.")]
    fn test_prediction_negative_slope() {
        prediction(1.0, -1.0, 5.0);
    }

    #[test]
    #[should_panic(expected = "O intercepto (intercept) não pode ser negativo.")]
    fn test_prediction_negative_intercept() {
        prediction(-2.0, 1.0, 5.0);
    }

    #[test]
    fn test_prediction_rounding() {
        let intercept = 1.111;
        let slope = 2.222;
        let x = 3.0;
        let y = prediction(intercept, slope, x);
        let expected = (intercept + slope * x) * 100.0;
        let expected_rounded = (expected.round()) / 100.0;
        assert!((y - expected_rounded).abs() < 1e-6);
    }
}
