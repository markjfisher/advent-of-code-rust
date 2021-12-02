/// Extended Euclidean algorithm
/// Find the the greatest common denominator of two integers a,b
/// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
#[allow(clippy::many_single_char_names)]
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}