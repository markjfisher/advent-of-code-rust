use crate::util::parse::ParseOps;
use std::sync::Mutex;
use crate::util::thread::*;

struct SharedState {
    p1: usize,
    p2: Vec<usize>,
}

pub fn parse(input: &str) -> (usize, usize) {
    let numbers = input.iter_unsigned().collect();
    let mutex = Mutex::new(SharedState {
        p1: 0,
        p2: vec![0; 130321],
    });

    spawn_batches(numbers, |batch| worker(&mutex, &batch));

    let result = mutex.into_inner().unwrap();
    (result.p1, result.p2.iter().max().copied().unwrap())
}

fn worker(mutex: &Mutex<SharedState>, batch: &[usize]) {
    // these are the per thread values
    let mut local_p1_total = 0;
    let mut local_totals = vec![0; 130321];
    let mut local_history = vec![usize::MAX; 130321];

    for (i, &start) in batch.iter().enumerate() {
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

        let mut curr = n4;
        let final_val = (4..2000).fold(n4, |prev, _| {
            curr = gen(prev);
            
            diffs.rotate_left(1);
            diffs[3] = shift_diff(prev, curr);
            
            let key = hash(diffs[0], diffs[1], diffs[2], diffs[3]);
            if local_history[key] != i {
                local_totals[key] += curr % 10;
                local_history[key] = i;
            }
            curr
        });
        
        local_p1_total += final_val;
    }

    // Merge results into shared state
    let mut exclusive = mutex.lock().unwrap();
    exclusive.p1 += local_p1_total;
    exclusive.p2.iter_mut().zip(local_totals.iter())
        .for_each(|(a, &b)| *a += b);
}

pub fn part1(solution: &(usize, usize)) -> usize {
    solution.0
}

pub fn part2(solution: &(usize, usize)) -> usize {
    solution.1
}

pub fn gen(n: usize) -> usize {
    // n ^= n << 6;
    // n &= 0xffffff;
    // n ^= n >> 5;
    // n &= 0xffffff;
    // n ^= n << 11;
    // n &= 0xffffff;
    // n

    ((((n ^ (n << 6)) & 0xffffff) ^ (((n ^ (n << 6)) & 0xffffff) >> 5) & 0xffffff) ^ 
    ((((n ^ (n << 6)) & 0xffffff) ^ (((n ^ (n << 6)) & 0xffffff) >> 5) & 0xffffff) << 11)) & 0xffffff
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
