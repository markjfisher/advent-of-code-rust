use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> u32 {
    // 4000 lines, 8 per line, 500 keys/locks in combination, my input has exactly 250 of each
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);
    let mut matching = 0;

    let is_lock = |grid: &Grid<u8>| grid[ORIGIN] == b'#';

    for pattern in input.split("\n\n") {
        let grid = Grid::parse(pattern);

        if is_lock(&grid) {
            locks.push(lock_hash(&grid));
        } else {
            keys.push(key_hash(&grid));
        }
    }

    for lock in &locks {
        for key in &keys {
            // we use nybbles to represent each key/lock position
            // In the sum, if any nybble is 6 or greater, this means we would overlap as we only have 5 positions that can be free.
            // Non matching case:
            // Lock: 0000 0101 0011 0100 0011
            // Key:  0101 0000 0010 0001 0011
            // Sum:  0101 0101 0101 0101 0110
            //                           ^^^^ overlap
            // In above example the last nybble is 6, which means we would overlap

            // Matching case:
            // Lock: 0000 0101 0011 0100 0011
            // Key:  0011 0000 0010 0000 0001
            // Sum:  0011 0101 0101 0100 0100
            // All nybbles are <= 6, so the key and lock fit.
            // The puzzle doesn't require that the keys fit perfectly, just that there is no overlap.

            // The trick here is to add 2 to each nybble, and then check if any are 8 or over, which is same as saying the last bit
            // of the nybble is set.
            // (e.g. 1+5+2, 2+4+2, 3+3+2, 4+2+2, 5+1+2)
            // If not, then the sum of heights from both sides is less than 6, and so we have a match
            if (lock + key + 0x22222) & 0x88888 == 0 {
                // println!("Matching:");
                matching += 1;
            }
            // _print_nybbles(*lock, *key);
        }
    }

    matching
}

fn key_hash(pattern: &Grid<u8>) -> i32 {
    (0..5).fold(0, |heights, x| {
        let mut position = Point::new(x, 5);
        while pattern[position] == b'#' {
            position += UP;
        }
        (heights << 4) + (5 - position.y)
    })
}

fn lock_hash(pattern: &Grid<u8>) -> i32 {
    (0..5).fold(0, |heights, x| {
        let mut position = Point::new(x, 1);
        while pattern[position] == b'#' {
            position += DOWN;
        }
        (heights << 4) + (position.y - 1)
    })
}

fn _print_nybbles(lock: i32, key: i32) {
    print!("Lock: ");
    for i in (0..5).rev() {
        let nybble = (lock >> (i * 4)) & 0xF;
        print!("{:04b} ", nybble);
    }
    println!();

    print!("Key:  ");
    for i in (0..5).rev() {
        let nybble = (key >> (i * 4)) & 0xF;
        print!("{:04b} ", nybble);
    }
    println!("");

    print!("Sum:  ");
    for i in (0..5).rev() {
        let nybble = ((lock + key) >> (i * 4)) & 0xF;
        print!("{:04b} ", nybble);
    }
    println!("\n");
}

pub fn part1(input: &u32) -> u32 {
    *input
}

pub fn part2(_input: &u32) -> u32 {
    0
}
