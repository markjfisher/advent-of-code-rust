use crypto::digest::Digest;
use crypto::md5::Md5;

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

fn solver(secret: &str, start_at: u32, is_valid: impl Fn(&[u8; 16]) -> bool) -> u32 {
    let mut hash = [0; 16];

    let mut hasher = Md5::new();
    hasher.input_str(secret);

    // simple brute force search
    (start_at..=u32::MAX)
        .find(|i| {
            let mut hasher = hasher;
            hasher.input_str(&i.to_string());
            hasher.result(&mut hash);
            is_valid(&hash)
        }).unwrap()

}