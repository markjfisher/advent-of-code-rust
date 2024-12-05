use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

fn topological_sort(rules: &[[bool; 100]; 100], values: &[usize]) -> (Vec<usize>, bool) {
    let mut result = Vec::new();
    let mut remaining: FastSet<usize> = values.iter().copied().collect();
    let mut is_correct = true;
    
    while !remaining.is_empty() {
        // Find a value that can precede all other remaining values
        let (next, original_position) = remaining
            .iter()
            .map(|&val| (val, values.iter().position(|&x| x == val).unwrap()))
            .find(|&(val, _)| remaining
                .iter()
                .all(|&other| val == other || rules[val][other]))
            .unwrap();
            
        // Check if we're removing values in order
        is_correct &= original_position == values.len() - remaining.len();
        
        result.push(next);
        remaining.remove(&next);
    }
    
    (result, is_correct)
}

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

    checks_data.lines().fold(
        (0, 0),
        |(sum_valid_middles, sum_invalid_middles), test_line| {
            let check_values: Vec<usize> = test_line.iter_unsigned().collect();
            let (corrected_sequence, is_correct) = topological_sort(&rules, &check_values);

            let middle_value = corrected_sequence[corrected_sequence.len() / 2];
            if is_correct {
                (sum_valid_middles + middle_value, sum_invalid_middles)
            } else {
                (sum_valid_middles, sum_invalid_middles + middle_value)
            }
        },
    )
}

pub fn part1(input: &(usize, usize)) -> usize {
    input.0
}

pub fn part2(input: &(usize, usize)) -> usize {
    input.1
}
