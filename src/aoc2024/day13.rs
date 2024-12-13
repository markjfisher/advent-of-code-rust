use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\n\n")
        .map(|group| group.iter_unsigned().collect())
        .collect()
}

// just do linear algebra. very simple day
// For both x, and y (our variables) we know they are integers, so they must
// divide perfectly. Thus we can do modulus checks before dividing. If the remainder isn't 0
// then we know the solution is not an integer, which means when it is, we don't worry about floats and rounding errors.
fn solve_equations(coeffs: &[u64]) -> Option<(i64, i64)> {
    let a = coeffs[0] as i64;
    let b = coeffs[2] as i64;
    let c = coeffs[1] as i64;
    let d = coeffs[3] as i64;
    let e = coeffs[4] as i64;
    let f = coeffs[5] as i64;

    let x_denom = b * c - a * d;
    if x_denom == 0 {
        // sanity check the determinant is not going to be 0 (e.g. equations are not linearly independent)
        return None;
    }

    let x_numerator = f * b - d * e;
    if x_numerator % x_denom != 0 {
        return None;
    }
    let x = x_numerator / x_denom;

    let y_numerator = e - a * x;
    // in our input, b is never 0, so we the division is safe
    if y_numerator % b != 0 {
        return None;
    }
    let y = y_numerator / b;

    Some((x, y))
}

pub fn part1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .filter_map(|coeffs| solve_equations(coeffs))
        .map(|(a, b)| (3 * a + b) as u64)
        .sum()
}

pub fn part2(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .filter_map(|coeffs| {
            let mut adjusted = coeffs.clone();
            adjusted[4] += 10_000_000_000_000;
            adjusted[5] += 10_000_000_000_000;
            solve_equations(&adjusted)
        })
        .map(|(a, b)| (3 * a + b) as u64)
        .sum()
}
