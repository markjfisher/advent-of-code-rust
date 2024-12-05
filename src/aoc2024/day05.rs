use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<(usize, usize)> {
    let (rules_data, checks_data) = input.split_once("\n\n").unwrap();
    // initialize a 100x100 matrix of false values
    let mut rules = [[false; 100]; 100];

    // parse the rules information about which page comes before another
    for [before, after] in rules_data.iter_unsigned::<usize>().chunk::<2>() {
        rules[before][after] = true;
    }

    let mut check_values = Vec::new();
    let mut copied_data = Vec::new();

    checks_data.lines().map(|test_line| {
        let mut is_correct = true;
        check_values.extend(test_line.iter_unsigned::<usize>());
        copied_data.clear();

        while !check_values.is_empty() {
            let next = check_values
                .iter()
                .enumerate()
                .position(|(i, &from)| {
                    let valid = check_values[i + 1..].iter().all(|&to| rules[from][to]);
                    valid
                })
                .unwrap();

            let value = check_values.remove(next);
            
            copied_data.push(value);
            is_correct &= next == 0;
        }

        if is_correct {
            (copied_data[copied_data.len() / 2], 0)
        } else {
            (0, copied_data[copied_data.len() / 2])
        }
    }).collect()
}


pub fn part1(input: &[(usize, usize)]) -> usize {
    input.iter().map(|(v, _)| v).sum()
}

pub fn part2(input: &[(usize, usize)]) -> usize {
    input.iter().map(|(_, v)| v).sum()
}