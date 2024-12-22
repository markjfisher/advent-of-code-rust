use crate::util::parse::ParseOps;


pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[usize]) -> usize {
    input.iter()
        .map(|&number| (0..2000).fold(number, |n, _| gen(n)))
        .sum()
}

pub fn part2(_input: &[usize]) -> u32 {
    456
}

pub fn gen(mut n: usize) -> usize {
    n ^= n << 6;
    n &= 0xffffff;
    n ^= n >> 5;
    n &= 0xffffff;
    n ^= n << 11;
    n &= 0xffffff;
    n
}