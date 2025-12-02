fn digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut d = 0;
    while n > 0 {
        n /= 10;
        d += 1;
    }
    d
}

// We are counting numbers of the form:
//
//   n = kk
//
// where k is a d-digit number (e.g. k=12 -> n=1212).
//
// This can be written as:
//   n = k * 10^d + k           e.g. = 12 * 10^2 + 12 = 1200 + 12 = 1212
//     = k * (10^d + 1)
//
// So for each possible block length d, n takes the form:
//
//   n = k * factor, where factor = 10^d + 1
//
// Constraints:
//   - k must be a d-digit number: 10^(d-1) <= k <= 10^d - 1
//   - lower <= n <= upper
//
// This gives a range for k:
//
//   ceil(lower / factor) <= k <= floor(upper / factor)
//
// Intersect this with the valid d-digit range for k,
// which gives a range [k_lo, k_hi].
//
// The sum of all valid n is:
//
//   factor * (sum of k from k_lo to k_hi)
//
// Note:
// - The number of digits of n is 2d, so d <= digits(upper) / 2.
// - This avoids iterating over the entire [lower, upper] range
// Special extra note!
// - This is the r = 2 special case of the general repeated-block formula used in part 2.
// For r = 2, factor(d, r) = (10^(2d) - 1) / (10^d - 1) simplifies to 10^d + 1.
pub fn sum_pairs_in_range(lower: u64, upper: u64) -> u64 {
    if lower > upper {
        return 0;
    }

    let mut total_sum = 0u64;

    // doubled number has 2*d digits, so d is at most digits(upper)/2
    let max_d = digits(upper) / 2;

    for d in 1..=max_d {
        let ten_d = 10u64.pow(d);      // 10^d
        let factor = ten_d + 1;        // pair(k) = k * factor

        let min_k = 10u64.pow(d - 1);  // smallest d-digit k
        let max_k = ten_d - 1;         // largest d-digit k

        // k_lo = ceil(lower / factor)
        let mut k_lo = (lower + factor - 1) / factor;
        if k_lo < min_k {
            k_lo = min_k;
        }

        // k_hi = floor(upper / factor)
        let mut k_hi = upper / factor;
        if k_hi > max_k {
            k_hi = max_k;
        }

        if k_hi >= k_lo {
            let count = k_hi - k_lo + 1;

            // sum of k from k_lo to k_hi
            let sum_k = (k_lo + k_hi) * count / 2;

            let sum_pairs = factor * sum_k;

            total_sum += sum_pairs;
        }
    }

    total_sum
}

// brute force approach - initial solution. sorry perfect me.
pub fn is_repeated_block(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for block_len in 1..=len / 2 {
        if len % block_len != 0 {
            continue;
        }

        let repeats = len / block_len;
        if repeats < 2 {
            continue;
        }

        let pattern = &s[..block_len];
        let candidate = pattern.repeat(repeats);

        if candidate == s {
            return true;
        }
    }

    false
}

// PART 2 — numbers made of some digit block repeated at least twice
// Conceptually actually very similar to part 1, but we vary the repetitions
// and don't just fix it to 2. However, I'm keeping them separate as they were
// developed that way, and the general version in p1 is a reduction of this
// version.
//
// So... the maths for me when I reread this next year :D See you in 2026.
// We are counting numbers whose decimal representation is:
//
//   n = kkkkk...k  (block k repeated r times, r >= 2)
//
// Let:
//   - k be a d-digit block
//   - r be the number of repetitions (r >= 2)
//
// Then n can be written as a geometric series:
//
//   n = k * 10^{d(r-1)} + k * 10^{d(r-2)} + ... + k
//
// Which simplifies to:
//
//   n = k * (10^{dr} - 1) / (10^d - 1)
//
// See how this compares to p1!
//
// Let:
//
//   factor(d, r) = (10^{dr} - 1) / (10^d - 1)
//
// So every valid repeated-block number has the form:
//
//   n = k * factor(d, r)
//
// Constraints:
//   - k must be a d-digit number: 10^(d-1) <= k <= 10^d - 1
//   - r >= 2
//   - lower <= n <= upper
//
// This gives:
//
//   ceil(lower / factor) <= k <= floor(upper / factor)
//
// Intersected with the d-digit k range.
//
// Algorithm:
//   - Iterate over all (d, r) such that d*r <= digits(upper)
//   - For each (d, r), compute factor(d, r)
//   - Solve for the valid k range
//   - Generate n = k * factor(d, r)
//
// Some numbers (e.g. 111111) can be represented by multiple (d, r) pairs,
// e.g. "111111"x1, "111"x2, "11"x3, "1"x6
// so we collect all candidates, then sort + dedupe.
//
// Properties:
//   - Runtime depends only on digit count, not range size
//   - Avoids brute-force iteration over [lower, upper]
//   - Massively faster than brute-force for large ranges

pub fn sum_repeated_in_range(lower: u64, upper: u64) -> u64 {
    if lower > upper {
        return 0;
    }

    let max_total_digits = digits(upper);
    let mut candidates: Vec<u64> = Vec::new();

    // u128 to avoid overflow!
    let lower128 = lower as u128;
    let upper128 = upper as u128;

    // d is the number of digits in the repeated block, r is the number of blocks
    for d in 1..=max_total_digits {
        for r in 2..=max_total_digits / d {
            let total_digits = d * r;

            // 10^d and 10^(d*r)
            let pow10_d = 10u128.pow(d as u32);
            let pow10_total = 10u128.pow(total_digits as u32);

            let numerator = pow10_total - 1;
            let denominator = pow10_d - 1;
            let factor128 = numerator / denominator; // exact division

            if factor128 > upper128 {
                continue; // even k = 1 would be too big
            }

            let min_k128 = 10u128.pow((d - 1) as u32);
            let max_k128 = pow10_d - 1;

            // ceil(lower / factor)
            let mut k_lo = (lower128 + factor128 - 1) / factor128;
            if k_lo < min_k128 {
                k_lo = min_k128;
            }

            // floor(upper / factor)
            let mut k_hi = upper128 / factor128;
            if k_hi > max_k128 {
                k_hi = max_k128;
            }

            if k_lo > k_hi {
                continue;
            }

            for k in k_lo..=k_hi {
                let n = k * factor128;
                if n >= lower128 && n <= upper128 {
                    // we dedupe later, e.g. "1"x6, "11"x3, "111"x2, "111111"x1
                    candidates.push(n as u64);
                }
            }
        }
    }

    // Handy way to ensure it's not in release version if we keep debug lines
    // #[cfg(debug_assertions)]
    // {
    //     dbg!(&candidates);
    // }
        
    // dbg!(&candidates.len());
    // Deduplicate numbers that can be written with multiple (d, r)
    candidates.sort_unstable();
    candidates.dedup();
    // dbg!(&candidates.len());


    candidates.into_iter().sum::<u64>()
}

pub fn parse(input: &str) -> (u64, u64) {
    let (mut p1, mut p2) = (0u64, 0u64);

    for s in input.split(',') {
        let mut parts = s.split('-');
        let lower = parts.next().unwrap().parse::<u64>().unwrap();
        let upper = parts.next().unwrap().parse::<u64>().unwrap();

        p1 += sum_pairs_in_range(lower, upper);
        p2 += sum_repeated_in_range(lower, upper);
    }

    (p1, p2)
}

pub fn part1(input: &(u64, u64)) -> u64 {
    input.0
}

pub fn part2(input: &(u64, u64)) -> u64 {
    input.1
}