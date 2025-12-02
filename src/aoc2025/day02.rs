pub fn parse(input: &str) -> (u64, u64) {
    fn sum_pairs_in_range(lower: u64, upper: u64) -> u64 {
        if lower > upper {
            return 0;
        }

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

    fn is_repeated_block(n: u64) -> bool {
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

    fn sum_repeated_in_range(lower: u64, upper: u64) -> u64 {
        if lower > upper {
            return 0;
        }

        let mut sum = 0u64;
        for n in lower..=upper {
            if is_repeated_block(n) {
                sum += n;
            }
        }
        sum
    }

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