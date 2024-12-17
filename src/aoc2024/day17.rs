use crate::aoc2024::comp::Comp;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect::<Vec<usize>>()
}

pub fn part1(input: &[usize]) -> String {
    let mut comp = Comp::new(input);
    comp.run();
    comp.get_output()
}

// The difficulty here was you have to look backwards through the output.
// we see that the nth digit increments at every 8^nth step, so we have to find a value for last digit, then move backwards until they all match.
// We're effectively shifting the solution by powers of 8 after finding each number.

// This version uses the Comp to run the simulation so works on any input
pub fn _part2(input: &[usize]) -> usize {
    let program = &input[3..];
    let mut a = 0;
    // let mut found = Vec::new();

    for n in 1..=program.len() {
        let target = program[program.len()-n..].to_vec();

        let mut new_a = a << 3;
        loop {
            let mut comp = Comp::new(input);
            comp.reg_a = new_a;
            comp.run();
            
            if comp.output == target {
                // let next_number = new_a - (a << 3);
                // found.push(next_number);
                // println!("Found number: {} [o{:o}] (a = {} [o{:o}])", next_number, next_number, new_a, new_a);
                a = new_a;
                break;
            }
            new_a += 1;
        }
    }
    // println!("Final value in octal: {:o}", a);
    // println!("Final value normal:   {}", a);
    a
}

// This version is optimized to my input by manually working out the bitwise operations instead of using the Comp
// but still using the reverse digit lookup idea
pub fn part2(input: &[usize]) -> usize {
    let program = &input[3..];
    let mut a = 0;

    for n in 1..=program.len() {
        let target = program[program.len()-n..].to_vec();

        let mut new_a = a << 3;
        loop {
            let mut digits = Vec::new();
            let mut test_a = new_a;

            // I tried memoizing the values here, but it's marginally faster to keep calculating
            // the same values rather than looking up in a map
            while test_a != 0 {
                let mut b = test_a & 0x07;
                b = b ^ 1;
                let c = test_a >> b;
                b = b ^ 5;
                test_a >>= 3;
                b = b ^ c;

                let test_digit = b & 0x07;
                if test_digit != *target.get(digits.len()).unwrap() {
                    break;
                }

                digits.push(test_digit);
                // println!("test_a: {}, b: {}, c: {}, pushing {}", test_a, b, c, b & 0x07);
            }

            // println!("digits: {:?}, target: {:?}", digits, target);
            if digits == target {
                a = new_a;
                break;
            }
            new_a += 1;
        }
    }

    a
}
