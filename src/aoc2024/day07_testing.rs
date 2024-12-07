use super::day07::*;

pub fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines()
        .map(|line| {
            let nums: Vec<u64> = line.split(|c: char| !c.is_numeric())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            (nums[0], nums[1..].to_vec())
        })
        .collect()
}

pub fn compare_solutions(input: &[(u64, Vec<u64>)], use_concat: bool) {
    for (target, nums) in input {
        let bitwise_result = can_make_target(*target, nums, use_concat, false);
        
        let mut validate_nums = Vec::with_capacity(nums.len() + 1);
        validate_nums.push(*target);
        validate_nums.extend(nums);
        
        let recursive_result = validate(&validate_nums, *target, nums.len(), use_concat);
        
        if bitwise_result != recursive_result {
            println!("\nMismatch found for target {} with nums {:?}", target, nums);
            println!("  bitwise_result: {}", bitwise_result);
            println!("  recursive_result: {}", recursive_result);
            
            println!("\nDetailed debug output:");
            println!("Bitwise approach:");
            can_make_target(*target, nums, use_concat, true);
            
            println!("\nRecursive approach:");
            let mut debug_str = String::new();
            _validate_with_debug(&validate_nums, *target, nums.len(), use_concat, &mut debug_str);
            println!("Trace:\n{}", debug_str);
        }
    }
}

pub fn compare_solutions_verbose(input: &[(u64, Vec<u64>)], use_concat: bool) {
    for (target, nums) in input {
        let bitwise_result = can_make_target(*target, nums, use_concat, false);
        
        // Create a new vector with target as first element for validate
        let mut validate_nums = Vec::with_capacity(nums.len() + 1);
        validate_nums.push(*target);
        validate_nums.extend(nums);
        
        let recursive_result = validate(&validate_nums, *target, nums.len(), use_concat);
        
        println!("Target {} with nums {:?}", target, nums);
        println!("  bitwise_result: {}", bitwise_result);
        println!("  recursive_result: {}", recursive_result);
        if bitwise_result != recursive_result {
            println!("  ^^^ MISMATCH! ^^^");
        }
        println!();
    }
}

pub fn part1(_input: &[(u64, Vec<u64>)]) -> u64 {
    0
}

pub fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    compare_solutions(input, true);
    0
}

fn can_make_target(target: u64, nums: &[u64], use_concat: bool, debug: bool) -> bool {
    let op_count = nums.len() - 1;
    let bits_per_op = if use_concat { 2 } else { 1 };
    let combinations = 1 << (bits_per_op * op_count);

    for op_combination in 0..combinations {
        let mut result = nums[0];
        let mut expression = if debug { format!("{}", nums[0]) } else { String::new() };
        let mut valid = true;
        
        for i in 0..op_count {
            let next_num = nums[i + 1];
            let op_bits = if use_concat {
                (op_combination >> (2 * i)) & 0b11
            } else {
                (op_combination >> i) & 0b1
            };

            let (op, result_new) = if use_concat {
                match op_bits {
                    0 => ('+', result + next_num),
                    1 => ('*', result * next_num),
                    2 => ('|', {
                        let concat = format!("{}{}", result, next_num)
                            .parse::<u64>()
                            .unwrap_or(0);
                        concat
                    }),
                    _ => {
                        valid = false;
                        break;
                    }
                }
            } else {
                match op_bits {
                    0 => ('+', result + next_num),
                    1 => ('*', result * next_num),
                    _ => unreachable!(),
                }
            };
            
            if debug {
                expression.push_str(&format!(" {} {}", op, next_num));
            }
            result = result_new;
        }
        
        if valid && result == target {
            if debug {
                println!("{} = {}", expression, result);
            }
            return true;
        } else if debug && valid {
            println!("{} = {}", expression, result);
        }
    }
    
    if debug {
        println!("Target {} not possible after trying all combinations", target);
    }
    false
}
