use crate::{aoc2024::comp::Comp, util::parse::ParseOps};

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

pub fn part2(input: &[usize]) -> i64 {
    let program = &input[3..];
    let mut a = 0;
    // let mut found = Vec::new();

    for n in 1..=program.len() {
        let target = program[program.len()-n..]
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<i64>>();

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
