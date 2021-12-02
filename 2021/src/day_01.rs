use itertools::Itertools;

use crate::common;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    common::input_vec(input)
}

#[aoc(day1, part1)]
pub fn solve_part_01(input: &[u32]) -> u32 {
    calculate_windowed_increases(input, 1)
}

#[aoc(day1, part2)]
pub fn solve_part_02(input: &[u32]) -> u32 {
    calculate_windowed_increases(input, 3)
}

fn calculate_windowed_increases(input: &[u32], window_size: usize) -> u32 {
    input
        .windows(window_size)
        .map(|es| es.iter().sum::<u32>())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count() as u32
}
