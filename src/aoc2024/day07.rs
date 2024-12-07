use crate::util::parse::*;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let mut equation = Vec::new();
    let mut part_one = 0;
    let mut part_two = 0;

    for line in input.lines() {
        equation.extend(line.iter_unsigned::<u64>());

        // let mut debug = format!(" [{}]", line);
        // if _validate_with_debug(&equation, equation[0], equation.len() - 1, false, &mut debug) {
        if validate(&equation, equation[0], equation.len() - 1, false) {
            // println!("Part 1&2 match!");
            part_one += equation[0];
            part_two += equation[0];
        } else {
            // if _validate_with_debug(&equation, equation[0], equation.len() - 1, true, &mut debug) {
            if validate(&equation, equation[0], equation.len() - 1, true) {
                // println!("Part 2 match!");
                part_two += equation[0];
            }
        }

        equation.clear();
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

pub fn validate(terms: &[u64], test_value: u64, index: usize, concat: bool) -> bool {
    // check final value matches last term
    if index == 1 {
        return test_value == terms[1];
    } else {
        (concat
            && test_value % next_power_of_ten(terms[index]) == terms[index]
            && validate(terms, test_value / next_power_of_ten(terms[index]), index - 1, concat))
            || (test_value % terms[index] == 0
                && validate(terms, test_value / terms[index], index - 1, concat))
            || (test_value >= terms[index]
                && validate(terms, test_value - terms[index], index - 1, concat))
    }
}

fn next_power_of_ten(n: u64) -> u64 {
    let mut power = 10;

    while power <= n {
        power *= 10;
    }

    power
}

// Original bitwise solution that takes forever, but is at least working now
pub fn _can_make_target(target: u64, nums: &[u64], use_concat: bool) -> bool {
    let op_count = nums.len() - 1;
    let combinations = 1 << (if use_concat { 2 * op_count } else { op_count });

    'next_combination: for op_combination in 0..combinations {
        let mut result = nums[0];
        
        for i in 0..op_count {
            let next_num = nums[i + 1];
            let op_bits = if use_concat {
                (op_combination >> (2 * i)) & 0b11
            } else {
                (op_combination >> i) & 0b1
            };

            result = if use_concat {
                match op_bits {
                    0 => result + next_num,
                    1 => result * next_num,
                    2 => format!("{}{}", result, next_num)
                        .parse::<u64>()
                        .unwrap_or(0),
                    _ => continue 'next_combination,
                }
            } else {
                match op_bits {
                    0 => result + next_num,
                    1 => result * next_num,
                    _ => unreachable!(),
                }
            };
        }
        
        if result == target {
            return true;
        }
    }
    
    false
}

// A verbose version of the recursive solution to debug it
pub fn _validate_with_debug(terms: &[u64], test_value: u64, index: usize, concat: bool, debug: &mut String) -> bool {
    if test_value == 0 {
        if index == 0 {
            // println!("Found match: {}", debug);
            return true;
        }
        return false;
    }
    
    if index == 0 {
        return false;
    }

    // Try concatenation
    if concat && test_value % next_power_of_ten(terms[index]) == terms[index] {
        let mut new_debug = debug.clone();
        new_debug = format!("({} || {}){}", 
            test_value / next_power_of_ten(terms[index]), 
            terms[index], 
            new_debug);
        if _validate_with_debug(terms, test_value / next_power_of_ten(terms[index]), index - 1, concat, &mut new_debug) {
            *debug = new_debug;
            return true;
        }
    }

    // Try multiplication
    if test_value % terms[index] == 0 {
        let mut new_debug = debug.clone();
        new_debug = format!("({} * {}){}", 
            test_value / terms[index], 
            terms[index], 
            new_debug);
        if _validate_with_debug(terms, test_value / terms[index], index - 1, concat, &mut new_debug) {
            *debug = new_debug;
            return true;
        }
    }

    // Try addition
    if test_value >= terms[index] {
        let mut new_debug = debug.clone();
        new_debug = format!("({} + {}){}", 
            test_value - terms[index], 
            terms[index], 
            new_debug);
        if _validate_with_debug(terms, test_value - terms[index], index - 1, concat, &mut new_debug) {
            *debug = new_debug;
            return true;
        }
    }

    false
}
