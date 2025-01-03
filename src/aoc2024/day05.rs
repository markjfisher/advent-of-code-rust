use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> (usize, usize) {
    let (rules_data, checks_data) = input.split_once("\n\n").unwrap();
    
    // Create and populate the rules array
    let mut rules = [[false; 100]; 100];
    rules_data
        .iter_unsigned::<usize>()
        .chunk::<2>()
        .for_each(|[before, after]| {
            rules[before][after] = true;
        });

    checks_data.lines().fold((0, 0), |(sum_valid_middles, sum_invalid_middles), test_line| {
        let mut is_correct = true;
        let mut check_values: Vec<usize> = test_line.iter_unsigned().collect();
        let mut copied_data = Vec::new();

        while !check_values.is_empty() {
            let sequence_start_index = check_values
                .iter()
                .enumerate()
                .position(|(i, &from)| {
                    check_values[i + 1..].iter().all(|&to| rules[from][to])
                })
                .unwrap();

            let value = check_values.remove(sequence_start_index);
            copied_data.push(value);
            is_correct &= sequence_start_index == 0;
        }

        let middle_value = copied_data[copied_data.len() / 2];
        if is_correct {
            (sum_valid_middles + middle_value, sum_invalid_middles)
        } else {
            (sum_valid_middles, sum_invalid_middles + middle_value)
        }
    })
}

pub fn part1(input: &(usize, usize)) -> usize {
    input.0
}

pub fn part2(input: &(usize, usize)) -> usize {
    input.1
}
