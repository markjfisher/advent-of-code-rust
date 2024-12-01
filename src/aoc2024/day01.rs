use crate::util::parse::ParseOps;


pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut first, mut second) = split_odd_even_indices(&input.iter_unsigned().collect::<Vec<_>>());
    first.sort();
    second.sort();
    (first, second)
}

pub fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {    
    input.0.iter()
        .zip(input.1.iter())
        .map(|(a, b)| {
            let diff = a.abs_diff(*b);
            // println!("{} vs {} = diff {}", a, b, diff);
            diff
        })
        .sum()
}

pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    input.0.iter()
        .map(|&num| {
            let frequency = input.1.iter()
                .filter(|&&x| x == num)
                .count() as u32;
            num * frequency
        })
        .sum()
}

pub fn split_odd_even_indices<T>(input: &[T]) -> (Vec<T>, Vec<T>) 
where 
    T: Clone,
{
    input.iter()
        .step_by(2)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .zip(
            input.iter()
                .skip(1)
                .step_by(2)
                .cloned()
                .collect::<Vec<_>>()
        )
        .unzip()
}
