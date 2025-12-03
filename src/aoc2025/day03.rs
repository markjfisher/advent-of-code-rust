pub fn parse(input: &str) -> (u64, u64) {

    let p1 = input.lines().map(|line| {
       extract_highest(line, 2) 
    }).sum::<u64>();

    let p2 = input.lines().map(|line| {
       extract_highest(line, 12) 
    }).sum::<u64>();

    (p1, p2)
}

pub fn part1(input: &(u64, u64)) -> u64 {
    input.0
}

pub fn part2(input: &(u64, u64)) -> u64 {
    input.1
}

pub fn extract_highest(input: &str, count: usize) -> u64 {
    let bytes = input.as_bytes();
    let len = bytes.len();

    assert!(count > 0 && count <= len, "count must be between 1 and input.len()");

    let mut result: u64 = 0;
    let mut start = 0usize;

    for taken in 0..count {
        let remaining = count - taken;
        // The first digit we choose in this step must be at or before this index,
        // so that we still have enough room for the remaining digits
        let last_start = len - remaining;

        let mut best_digit = b'0';
        let mut best_idx = start;

        // Scan the allowed window to find the highest digit
        for idx in start..=last_start {
            let b = bytes[idx];
            if b > best_digit {
                best_digit = b;
                best_idx = idx;

                // Can't get better than '9'
                if best_digit == b'9' {
                    break;
                }
            }
        }

        // Add found digit to 10 * current result
        result = result * 10 + (best_digit - b'0') as u64;
        // Next search must start after the chosen digit
        start = best_idx + 1;
    }

    result
}