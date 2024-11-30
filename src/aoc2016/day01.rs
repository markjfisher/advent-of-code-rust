use crate::util::parse::ParseOps;
use crate::util::point::*;
use crate::util::hash::*;

pub fn parse(input: &str) -> Vec<(u8, i32)> {
    // input
    //     .split(", ")
    //     .map(|s| {
    //         let (d, n) = s.split_at(1);
    //         (d.chars().next().unwrap(), n.parse::<i32>().unwrap())
    //     })
    //     .collect()

    let dirs = input.bytes().filter(u8::is_ascii_uppercase);
    let lens = input.iter_signed();
    dirs.zip(lens).collect()
}

pub fn part1(input: &[(u8, i32)]) -> i32 {
    let mut pos = ORIGIN;
    let mut dir = UP;
    for &(d, n) in input {
        dir = match d {
            b'L' => dir.counter_clockwise(),
            b'R' => dir.clockwise(),
            _ => unreachable!(),
        };
        pos += dir * n;
    }
    pos.manhattan(ORIGIN)
}

pub fn part2(input: &[(u8, i32)]) -> i32 {
    let mut visited = FastSet::with_capacity(1000);
    let mut pos = ORIGIN;
    let mut dir = UP;
    for &(d, n) in input {
        dir = match d {
            b'L' => dir.counter_clockwise(),
            b'R' => dir.clockwise(),
            _ => unreachable!(),
        };
        for _ in 0..n {
            pos += dir;
            if !visited.insert(pos) {
                return pos.manhattan(ORIGIN);
            }
        }
    }
    unreachable!()
}
