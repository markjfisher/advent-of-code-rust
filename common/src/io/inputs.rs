use std::{fmt::Debug, str::FromStr};

fn clean_input(input: &str) -> impl Iterator<Item=&str> {
    input.lines().map(|l| l.trim()).filter(|l| !l.is_empty())
}

/// Trims lines and removes any empty rows
/// Return a `Vec<T>`
pub fn input_vec<T>(input: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
{
    clean_input(input).map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_input() {
        let input = "  line1  \n\n  line2\nline3   \n\n";
        let result: Vec<&str> = clean_input(input).collect();
        assert_eq!(result, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_input_vec() {
        let input = "42\n  123  \n\n  999\n";
        let result: Vec<i32> = input_vec(input);
        assert_eq!(result, vec![42, 123, 999]);
    }
}