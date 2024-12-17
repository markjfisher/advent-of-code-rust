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

// This version uses the Comp to run the simulation so works on any input. It's slower than direct, as it doesn't shortcut values when they don't match expected output
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

// This version is optimized to do instructions manually rather than using the Comp
// but still using the reverse digit lookup idea. It gets the 2 bxl constants from the input by finding the first two values in a pair that match [1, bxl1], [1, bxl2]
pub fn part2(input: &[usize]) -> usize {
    let program = &input[3..];
    let mut a = 0;

    let [bxl1, bxl2] = program.chunks(2)
        .filter(|p| p[0] == 1)
        .map(|p| p[1])
        .take(2)
        .collect::<Vec<usize>>()[..] else { panic!("Did not find two bxl commands") };

    for n in 1..=program.len() {
        let target = program[program.len()-n..].to_vec();

        let mut new_a = a << 3;
        loop {
            let mut digits = Vec::new();
            let mut test_a = new_a;

            while test_a != 0 {
                let mut b = test_a & 0x07;
                b = b ^ bxl1;
                let c = test_a >> b;
                b = b ^ c;
                b = b ^ bxl2;
                test_a >>= 3;

                let test_digit = b & 0x07;
                if test_digit != *target.get(digits.len()).unwrap() {
                    // skip if the new digit doesn't match the target.
                    break;
                }

                digits.push(test_digit);
            }

            if digits == target {
                a = new_a;
                break;
            }
            new_a += 1;
        }
    }

    a
}
