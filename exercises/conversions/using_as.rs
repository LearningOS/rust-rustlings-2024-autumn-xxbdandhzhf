fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        let total = values.iter().sum::<f64>();
        Some(total / values.len() as f64) // Cast `usize` to `f64` to perform floating point division
    }
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    match average(&values) {
        Some(avg) => println!("{}", avg),
        None => println!("Cannot calculate the average of an empty list."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), Some(7.125));
    }

    #[test]
    fn returns_none_for_empty_slice() {
        assert_eq!(average(&[]), None);
    }
}