use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\n\n")
        .map(|group| group.iter_unsigned().collect())
        .collect()
}

// just do linear algebra. very simple day
fn solve_equations(coeffs: &[u64], check_upper_bound: bool) -> Option<(i64, i64)> {
    let a = coeffs[0] as i64;
    let b = coeffs[2] as i64;
    let c = coeffs[1] as i64;
    let d = coeffs[3] as i64;
    let e = coeffs[4] as i64;
    let f = coeffs[5] as i64;

    let denominator = b * c - a * d;
    if denominator == 0 {
        return None;
    }

    let numerator = f * b - d * e;
    if numerator % denominator != 0 {
        return None;
    }

    let x = numerator / denominator;
    let y = (e - a * x) / b;

    if x < 0 || y < 0 || (check_upper_bound && (x > 100 || y > 100)) {
        return None;
    }

    if c * x + d * y != f {
        return None;
    }

    Some((x, y))
}

pub fn part1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .filter_map(|coeffs| solve_equations(coeffs, true))
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
            solve_equations(&adjusted, false)
        })
        .map(|(a, b)| (3 * a + b) as u64)
        .sum()
}
