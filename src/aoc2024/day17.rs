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
// We're effectively shifting the solution by powers of 8 after finding each digit.

pub fn part2(input: &[usize]) -> String {
    let program = &input[3..];
    let mut a = 0;
    let mut octal_digits = Vec::new();

    for n in 1..=program.len() {
        let target = program[program.len()-n..]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let mut new_a = a << 3;
        loop {
            let mut comp = Comp::new(input);
            comp.reg_a = new_a;
            comp.run();
            
            if comp.get_output() == target {
                let octal_digit = new_a & 0x7;
                octal_digits.push(octal_digit);
                println!("Found digit: {} (a = {})", octal_digit, new_a);
                a = new_a;
                break;
            }
            new_a += 1;
        }
    }

    println!("Final value in octal: {:o}", a);
    println!("Final value normal:   {}", a);
    a.to_string()
}
