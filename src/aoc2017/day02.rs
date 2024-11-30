use crate::util::parse::*;

type Input = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut vs: Vec<_> = l.iter_unsigned().collect();
            vs.sort_unstable();
            vs
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().map(|l| l.last().unwrap() - l.first().unwrap()).sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|values| {
            for i in 0..values.len() {
                for j in i + 1..values.len() {
                    // 2nd value must be equal to or less than half of the first value, and must divide it exactly.
                    if values[i] <= values[j] / 2 && values[j] % values[i] == 0 {
                        return values[j] / values[i];
                    }
                }
            }
            unreachable!()
        })
        .sum()
}