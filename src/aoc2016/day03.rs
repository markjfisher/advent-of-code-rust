use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u32]) -> usize {
    triangle_test(input.iter().copied())
}

pub fn part2(input: &[u32]) -> usize {
    triangle_test(
        // rearrange the input into columns
        input
            .chunks(9) // Split into groups of 9 (3 rows × 3 columns)
            .flat_map(|chunk| {
                // Split each group into 3 columns
                let col1 = [chunk[0], chunk[3], chunk[6]]; // 0,3,6
                let col2 = [chunk[1], chunk[4], chunk[7]]; // 1,4,7
                let col3 = [chunk[2], chunk[5], chunk[8]]; // 2,5,8
                // Chain the columns together
                col1.into_iter()
                    .chain(col2.into_iter())
                    .chain(col3.into_iter())
            }),
    )
}

fn triangle_test(iter: impl Iterator<Item = u32>) -> usize {
    iter.chunk::<3>()
        .filter(|&[a, b, c]| a + b > c && a + c > b && b + c > a)
        .count()
}
