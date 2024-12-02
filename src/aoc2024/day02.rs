use crate::util::parse::*;

type Input = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.iter_unsigned().collect()).collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter()
        .filter(|numbers| is_valid_increasing(numbers) || is_valid_decreasing(numbers))
        .count() as u32
}

pub fn part2(input: &Input) -> u32 {
    // same as part1 but checking a least one subsequence is valid by removing an entry
    input
        .iter()
        .filter(|numbers| {
            let subsequences = subsequences_missing_one_entry(numbers);
            subsequences.iter().any(|seq| {
                is_valid_increasing(seq) || is_valid_decreasing(seq)
            })
        })
        .count() as u32
}

pub fn is_valid_increasing(numbers: &[u32]) -> bool {
    numbers.windows(2).all(|pair| {
        pair[1] > pair[0] && (pair[1] - pair[0]) <= 3
    })
}

pub fn is_valid_decreasing(numbers: &[u32]) -> bool {
    numbers.windows(2).all(|pair| {
        pair[0] > pair[1] && (pair[0] - pair[1]) <= 3
    })
}

pub fn subsequences_missing_one_entry(numbers: &[u32]) -> Vec<Vec<u32>> {
    let mut result = Vec::with_capacity(numbers.len());
    
    // use the index to skip that round's entry
    for skip_idx in 0..numbers.len() {
        let subsequence: Vec<u32> = numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, &num)| num)
            .collect();
        result.push(subsequence);
    }
    
    result
}
