use std::ops::BitAnd;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (u32, Vec<u32>) {
    // return the bit_length and values of input data as u32 array from string binary representation
    let arr = input.lines().map(|line| {
        u32::from_str_radix(line.trim(), 2).unwrap()
    }).collect();
    let bit_length = input.lines().next().unwrap().len() as u32;
    (bit_length, arr)
}

#[aoc(day3, part1)]
pub fn solve_part_01(input: &(u32, Vec<u32>)) -> u32 {
    let bit_length = input.0;
    let data = &input.1;
    let data_len = data.len();

    // AND every number in the array with 2^(size - index - 1) to get that bit's masked value
    // e.g. size 3, i=0, 101 && 100 == 100, then count all those > 0.

    let gamma = (0..bit_length).fold(0u32, |acc, bit| {
        let current_pow2 = 1 << (bit_length - bit - 1);
        let column_count_1s = count_1s(data, current_pow2);
        acc + if column_count_1s > (data_len / 2) { current_pow2 } else { 0 }
    });

    // this is the binary 1's complement of gamma, e.g. gamma = 1101, epsilon = 0010
    let epsilon = (1 << bit_length) - gamma - 1;

    gamma * epsilon
}

fn count_1s(data: &Vec<u32>, mask: u32) -> usize {
    data.iter()
        .filter(|d| d.bitand(mask) != 0)
        .count()
}

#[aoc(day3, part2)]
pub fn solve_part_02(input: &(u32, Vec<u32>)) -> u32 {
    let bit_length = input.0;
    let data = &input.1;

    let o2_gen_rating = calc_rating(bit_length, data, &msb);
    let co2_scrub_rating = calc_rating(bit_length, data, &lsb);

    o2_gen_rating * co2_scrub_rating
}

fn calc_rating(bit_length: u32, data: &Vec<u32>, bit_fn: &dyn Fn(&Vec<u32>, u32) -> u32) -> u32 {
    let mut mut_arr = data.clone();
    let mut current_bit = 0;
    while (mut_arr.len() > 1) && (current_bit < bit_length) {
        let current_pow2 = 1 << (bit_length - current_bit - 1);
        let val = bit_fn(&mut_arr, current_pow2);
        mut_arr.retain(|d| d.bitand(current_pow2) == val);
        current_bit = current_bit + 1;
    }
    *mut_arr.first().unwrap()
}

pub fn msb(arr: &Vec<u32>, mask: u32) -> u32 {
    let ones_count = count_1s(arr, mask);
    let zeros_count = arr.len() - ones_count;
    if ones_count >= zeros_count { mask } else { 0 }
}

pub fn lsb(arr: &Vec<u32>, mask: u32) -> u32 {
    let ones_count = count_1s(arr, mask);
    let zeros_count = arr.len() - ones_count;
    if ones_count < zeros_count { mask } else { 0 }
}


#[cfg(test)]
mod day_03_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_data = create_test_data();
        let bin_data = input_generator(&test_data);
        assert_eq!(solve_part_01(&bin_data), 198);
    }

    #[test]
    fn test_part_2() {
        let test_data = create_test_data();
        let bin_data = input_generator(&test_data);
        assert_eq!(solve_part_02(&bin_data), 230);
    }

    fn create_test_data() -> String {
        String::from(
        "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010")
    }
}
