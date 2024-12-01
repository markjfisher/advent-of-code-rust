use crate::util::{hash::{FastMap, FastMapBuilder}, parse::ParseOps};

pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    split_odd_even_indices(&input.iter_unsigned().collect::<Vec<u32>>())
}

pub fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {    
    let mut first = input.0.clone();
    let mut second = input.1.clone();
    first.sort();
    second.sort();
    
    first.iter()
        .zip(second.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    // Precalculate frequencies of values in the second list into a lookup map
    let frequencies: FastMap<u32, u32> = input.1.iter()
        .fold(FastMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    // original implementation used filter which constantly checked the same list multiple times. this dropped time from 7000μs to 1100μs
    input.0.iter()
        .map(|&num| num * frequencies.get(&num).unwrap_or(&0))
        .sum()
}

pub fn split_odd_even_indices(input: &[u32]) -> (Vec<u32>, Vec<u32>) {
    // Using enumerate/zip and fold into 2 new vectors
    // with zip, we can control the index start value, but the args are reversed (val, i)
    input.iter()
        // .enumerate() // this starts the index at 0, then the odd/even check is off by 1
        .zip(1..)
        .fold((vec![], vec![]), |(mut odd, mut even), (val, i)| {
            if i % 2 == 0 {
                even.push(val.clone());
            } else {
                odd.push(val.clone());
            }
            (odd, even)
        })
}
