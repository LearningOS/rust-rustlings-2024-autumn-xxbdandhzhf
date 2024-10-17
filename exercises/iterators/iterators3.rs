// Define the error types for division
#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    ZeroDivision,
    Other(String),
}

fn divide(numerator: i32, denominator: i32) -> Result<i32, DivisionError> {
    if denominator == 0 {
        Err(DivisionError::ZeroDivision)
    } else {
        Ok(numerator / denominator)
    }
}

fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    division_results.collect()
}
