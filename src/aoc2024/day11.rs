use crate::util::parse::ParseOps;


pub fn parse(input: &str) -> Vec<u64> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u64]) -> u64 {
    blink(&input.to_vec(), 25)
}

pub fn part2(input: &[u64]) -> u64 {
    // blink(&input.to_vec(), 75)
    0
}

pub fn blink(input: &[u64], steps: u32) -> u64 {
    if steps == 0 {
        return input.len() as u64;
    }
    println!("steps: {}, len: {}", steps, input.len());

    // Pre-allocate with estimated capacity to avoid reallocations
    let mut new_input = Vec::with_capacity(input.len() * 2);

    for &v in input {
        let number_len = (v as f64).log10() as u32 + 1;
        if v == 0 {
            new_input.push(1);
        } else if number_len % 2 == 0 {
            let divisor = 10u64.pow(number_len / 2);
            let (left, right) = (v / divisor, v % divisor);
            new_input.push(left);
            new_input.push(right);
        } else {
            new_input.push(v * 2024)
        }
    }

    blink(&new_input, steps - 1)
}