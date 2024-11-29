/// Extended Euclidean algorithm
/// Find the the greatest common denominator of two integers a,b
/// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
#[allow(clippy::many_single_char_names)]
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b.abs(), 0, b.signum())
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_egcd() {
        // Test case 1: Coprime numbers
        let (g, x, y) = egcd(15, 28);
        assert_eq!(g, 1);  // GCD should be 1
        assert_eq!(15 * x + 28 * y, g);  // Verify Bézout's identity

        // Test case 2: Numbers with common factors
        let (g, x, y) = egcd(48, 18);
        assert_eq!(g, 6);  // GCD should be 6
        assert_eq!(48 * x + 18 * y, g);  // Verify Bézout's identity

        // Test case 3: One number is zero
        let (g, x, y) = egcd(0, 7);
        assert_eq!(g, 7);
        assert_eq!(x, 0);
        assert_eq!(y, 1);

        // Test case 4: Negative numbers
        let (g, x, y) = egcd(-24, 15);
        assert_eq!(g, 3);  // GCD should be 3
        assert_eq!(-24 * x + 15 * y, g);  // Verify Bézout's identity
    }
}