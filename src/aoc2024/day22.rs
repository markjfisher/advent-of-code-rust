use crate::util::parse::ParseOps;


pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[usize]) -> usize {
    input.iter()
        .map(|&number| (0..2000).fold(number, |n, _| gen(n)))
        .sum()
}

pub fn part2(input: &[usize]) -> usize {
    // see hash function for explanation of why the size is 130321
    let mut totals = vec![0; 130321];
    let mut history = vec![usize::MAX; 130321];

    input.iter().enumerate().for_each(|(i, &start)| {
        // generate first 4 numbers in sequence
        let [n1, n2, n3, n4] = std::array::from_fn(|i| {
            (0..=i).fold(start, |acc, _| gen(acc))
        });

        // fun with slices
        // create a sliding window of differences
        let mut diffs = [
            shift_diff(start, n1),
            shift_diff(n1, n2),
            shift_diff(n2, n3),
            shift_diff(n3, n4),
        ];

        (4..2000).fold(n4, |prev, _| {
            let curr = gen(prev);
            
            // rotate differences and add new one
            diffs.rotate_left(1);
            diffs[3] = shift_diff(prev, curr);
            
            // calculate hash and update totals if unseen
            let key = hash(diffs[0], diffs[1], diffs[2], diffs[3]);
            // only add the total once, as the sequence could appear multiple times! OUCH here
            if history[key] != i {
                totals[key] += curr % 10;
                history[key] = i;
            }
            
            curr
        });
    });

    totals.iter().max().copied().unwrap()
}

pub fn gen(mut n: usize) -> usize {
    n ^= n << 6;
    n &= 0xffffff;
    n ^= n >> 5;
    n &= 0xffffff;
    n ^= n << 11;
    n &= 0xffffff;
    n
}

// there's only a range of -9 to 9 in differences (or 0 to 18 after shifting), so we can create a hash of the 4 numbers by using a multiple of 19 (being first prime number after 18)
// The range of this is 0 to 18, so max value is 130,320 (19^3 * 18 + 19^2 * 18 + 19 * 18 + 18)
pub fn hash(a: usize, b: usize, c: usize, d: usize) -> usize {
    // 19^3 = 6859, 19^2 = 361, 19^1 = 19, 19^0 = 1
    a + b * 19 + c * 361 + d * 6859
}

// calculate the difference between the last digit of a and b, adding 9 to make it positive in range 0 to 18
pub fn shift_diff(a: usize, b: usize) -> usize {
    // lesson! 9 has to start so that the values don't underflow when a > b, so we can use usize and not i32
    9 + (b % 10) - (a % 10)
}
