use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> u32 {
    // 4000 lines, 8 per line, 500 keys/locks in combination, my input has exactly 250 of each
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);
    let is_lock = |grid: &Grid<u8>| grid[ORIGIN] == b'#';

    for pattern in input.split("\n\n") {
        let grid = Grid::parse(pattern);
        if is_lock(&grid) {
            locks.push(lock_hash(&grid));
        } else {
            keys.push(key_hash(&grid));
        }
    }

    // for any good key/lock pair, all the sums of the heights must be less than 6 in each matching position
    locks.iter().map(|lock| {
        keys.iter().filter(|key| {
            lock.iter().zip(key.iter())
                .all(|(l, k)| l + k < 6)
        }).count() as u32
    }).sum()
}

fn key_hash(pattern: &Grid<u8>) -> Vec<u32> {
    (0..5).map(|x| {
        let mut position = Point::new(x, 5);
        while pattern[position] == b'#' {
            position += UP;
        }
        (5 - position.y) as u32
    }).collect()
}

fn lock_hash(pattern: &Grid<u8>) -> Vec<u32> {
    (0..5).map(|x| {
        let mut position = Point::new(x, 1);
        while pattern[position] == b'#' {
            position += DOWN;
        }
        (position.y - 1) as u32
    }).collect()
}

pub fn part1(input: &u32) -> u32 {
    *input
}

pub fn part2(_input: &u32) -> u32 {
    0
}
