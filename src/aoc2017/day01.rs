use std::u8;

use crate::util::parse::ParseByte;

pub fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

pub fn part1(input: &[u8]) -> u32 {
    captcha(input, 1)
}

pub fn part2(input: &[u8]) -> u32 {
    captcha(input, input.len() / 2)
}

fn captcha(input: &[u8], offset: usize) -> u32 {
    // rotate the input by offset, then compare each element with the original
    // sum up the matches

    let rotated = input.iter().cycle().skip(offset);
    input
        .iter()
        .zip(rotated)
        .filter(|(a, b)| a == b)
        .map(|(a, _)| (a.to_decimal()) as u32)
        .sum()
}
