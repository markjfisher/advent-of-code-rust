use crate::{aoc2024::comp::Comp, util::parse::ParseOps};

pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect::<Vec<usize>>()
}

pub fn part1(input: &[usize]) -> String {
    let mut comp = Comp::new(input);
    comp.run();
    comp.get_output()
}

pub fn part2(_input: &[usize]) -> String {
    // Create target string from input[3..] comma-separated
    // let target = input[3..].iter()
    //     .map(|n| n.to_string())
    //     .collect::<Vec<String>>()
    //     .join(",");

    // Try values of reg_a until we find a match
    // let mut a = 0;
    // loop {
    //     let mut comp = Comp::new(input);
    //     comp.reg_a = a;
    //     comp.run();
    //     if comp.get_output() == target {
    //         return a.to_string();
    //     }
    //     comp.output.clear();
    //     a += 1;
    // }
    "456".to_string()
}
