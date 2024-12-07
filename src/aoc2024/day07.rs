use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<(u128, Vec<u128>)> {
    input.lines().map(|line| {
        // println!("{}", line);
        let nums: Vec<u128> = line.iter_unsigned().collect();
        // println!("{:?}", nums);
        (nums[0], nums[1..].to_vec())
    }).collect()
}

// p1: 5030892084481
// p2: 91377501990670 too high
// p2: 91377448645408 too high

pub fn part1(input: &[(u128, Vec<u128>)]) -> u128 {
    input.iter()
        .filter(|(target, nums)| can_make_target(*target, nums, false))
        .map(|(target, _)| target)
        .sum()
}

pub fn part2(input: &[(u128, Vec<u128>)]) -> u128 {
    input.iter()
        .filter(|(target, nums)| can_make_target(*target, nums, true))
        .map(|(target, _)| target)
        .sum()
}

fn can_make_target(target: u128, nums: &[u128], use_concat: bool) -> bool {
    let op_count = nums.len() - 1;
    let bits_per_op = if use_concat { 2 } else { 1 };
    let combinations = 1 << (bits_per_op * op_count);

    for op_combination in 0..combinations {
        let mut result = nums[0];
        // let mut expression = format!("{}", nums[0]);
        
        for i in 0..op_count {
            let next_num = nums[i + 1];
            let op_bits = if use_concat {
                (op_combination >> (2 * i)) & 0b11
            } else {
                (op_combination >> i) & 0b1
            };

            // let (op, result_new) = if use_concat {
            let (_, result_new) = if use_concat {
                match op_bits {
                    0 => ('+', result + next_num),
                    1 => ('*', result * next_num),
                    2 => ('|', {
                        let concat = format!("{}{}", result, next_num)
                            .parse::<u128>()
                            .unwrap_or(0);
                        concat
                    }),
                    _ => continue,
                }
            } else {
                match op_bits {
                    0 => ('+', result + next_num),
                    1 => ('*', result * next_num),
                    _ => unreachable!(),
                }
            };
            
            // expression.push_str(&format!(" {} {}", op, next_num));
            result = result_new;
        }
        
        if result == target {
            // println!("Found target {}: {} = {}", target, expression, result);
            return true;
        } else {
            // println!("No match for {}: {} = {}", target, expression, result);
        }
    }
    
    // println!("Target {} not possible after trying all combinations", target);
    false
}
