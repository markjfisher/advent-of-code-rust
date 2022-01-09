use std::fmt;
use std::hash::Hash;

use cached::proc_macro::cached;

use crate::day_06::Action::{Off, On, Toggle};

#[derive(PartialEq, Hash, Debug, Clone)]
pub enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Hash, Debug, Clone)]
pub struct Instruction {
    action: Action,
    coordinates: (u32, u32, u32, u32),
}

impl PartialEq<Self> for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates && self.action == other.action
    }
}

impl Eq for Instruction {}

#[derive(PartialEq, Hash, Debug, Clone)]
pub struct Results(u32, u32);

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let mut ss = line.trim().split(" ");
        let mut action = ss.next().unwrap();
        if action == "turn" { action = ss.next().unwrap(); }
        let lxy = ss.next().unwrap();
        let mut mlxy = lxy.split(",").map(|v| v.parse().unwrap());
        let lx = mlxy.next().unwrap();
        let ly = mlxy.next().unwrap();
        ss.next();
        let uxy = ss.next().unwrap();
        let mut muxy = uxy.split(",").map(|v| v.parse().unwrap());
        let ux = muxy.next().unwrap();
        let uy = muxy.next().unwrap();
        let action = match action {
            "on" => On,
            "off" => Off,
            "toggle" => Toggle,
            _ => unreachable!()
        };
        Instruction { action, coordinates: (lx, ly, ux, uy) }
    }).collect()
}

#[cached]
fn solve(instructions: Vec<Instruction>) -> Results {
    let mut grid1: [u32; 1_000_000] = [0; 1_000_000];
    let mut grid2: [u32; 1_000_000] = [0; 1_000_000];

    for i in instructions {
        let (lx, ly, ux, uy) = i.coordinates;
        for y in ly..=uy {
            for x in lx..=ux {
                let i1 = (x + y * 1_000) as usize;
                match i.action {
                    On => {
                        grid1[i1] = 1;
                        grid2[i1] += 1;
                    }
                    Off => {
                        grid1[i1] = 0;
                        grid2[i1] = if grid2[i1] > 0 { grid2[i1] - 1 } else { 0 };
                    }
                    Toggle => {
                        grid1[i1] = 1 - grid1[i1];
                        grid2[i1] += 2;
                    }
                }
            }
        }
    }

    Results(grid1.iter().sum(), grid2.iter().sum())
}

#[aoc(day6, part1)]
pub fn part1(instructions: &Vec<Instruction>) -> u32 {
    solve(instructions.clone()).0
}

#[aoc(day6, part2)]
pub fn part2(instructions: &Vec<Instruction>) -> u32 {
    solve(instructions.clone()).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_instructions() {
        assert_eq!(*input_generator("turn on 887,9 through 959,629").first().unwrap(), Instruction { action: On, coordinates: (887, 9, 959, 629) });
        assert_eq!(*input_generator("turn off 539,243 through 559,965").first().unwrap(), Instruction { action: Off, coordinates: (539, 243, 559, 965) });
        assert_eq!(*input_generator("toggle 720,196 through 897,994").first().unwrap(), Instruction { action: Toggle, coordinates: (720, 196, 897, 994) });
    }
}