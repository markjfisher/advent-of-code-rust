use crate::util::parse::*;
use crate::util::hash::*;

pub fn parse(input: &str) -> Vec<u64> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u64]) -> u64 {
    blink(&input.to_vec(), 25)
}

pub fn part2(input: &[u64]) -> u64 {
    blink(&input.to_vec(), 75)
}

pub fn blink(input: &[u64], steps: u32) -> u64 {
    // each number follows the same rules, so we can memoize the results. With foresight (i.e. running it once!), we know there are a max of 125k entries
    // The cache is hit 65k times after some analysis
    let mut memo: FastMap<(u64, u32), u64> = FastMap::with_capacity(125_000);
    
    input.iter().map(|&num| {
        count_after_steps(num, steps, &mut memo)
    }).sum()
}

fn count_after_steps(num: u64, steps: u32, memo: &mut FastMap<(u64, u32), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }

    // check if we've seen this number at this step before
    if let Some(&count) = memo.get(&(num, steps)) {
        return count;
    }

    let count = if num == 0 {
        // 0 -> 1
        count_after_steps(1, steps - 1, memo)
    } else {
        let number_len = (num as f64).log10() as u32 + 1;
        if number_len % 2 == 0 {
            // split even length number into its two parts
            let divisor = 10u64.pow(number_len / 2);
            let left = num / divisor;
            let right = num % divisor;
            count_after_steps(left, steps - 1, memo) + 
            count_after_steps(right, steps - 1, memo)
        } else {
            // default is multiply by 2024
            count_after_steps(num * 2024, steps - 1, memo)
        }
    };

    memo.insert((num, steps), count);
    count
}