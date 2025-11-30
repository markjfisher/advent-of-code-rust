use md5::{Md5, Digest};
use itoa::Buffer;

pub fn parse(input: &str) -> &str {
    input.trim()
}

// This is a brute force question, starting from very close to the answer to reduce the time
pub fn part1(secret: &str) -> u32 {
    part1_solve(secret, 254570)
}

pub fn part1_solve(secret: &str, start_at: u32) -> u32 {
    solver(secret, start_at, |hash| hash[0..2] == [0; 2] && (hash[2] & 0xF0) == 0)
}

// This is a brute force question, starting from very close to the answer to reduce the time
pub fn part2(secret: &str) -> u32 {
    part2_solve(secret, 1038730)
}

fn part2_solve(secret: &str, start_at: u32) -> u32 {
    solver(secret, start_at, |hash| hash[0..3] == [0; 3])
}

fn solver(secret: &str, start_at: u32, is_valid: impl Fn(&[u8]) -> bool) -> u32 {
    let secret_bytes = secret.as_bytes();
    let mut buf = Buffer::new(); // stack-allocated, reusable

    for i in start_at..=u32::MAX {
        let mut hasher = Md5::new();

        // feed "secret"
        hasher.update(secret_bytes);

        // convert `i` to decimal *without* heap allocation
        let dec: &str = buf.format(i);
        hasher.update(dec.as_bytes());

        let hash = hasher.finalize();

        if is_valid(hash.as_slice()) {
            return i;
        }
    }

    unreachable!("Search should always find a solution before u32::MAX");
}
