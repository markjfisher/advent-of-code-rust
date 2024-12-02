use aoc::aoc2024::day02::*;
use assert_unordered::assert_eq_unordered;

const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 4);
}

#[test]
fn parse_input_test() {
    let input = parse(EXAMPLE);
    assert_eq!(input, vec![
        vec![7, 6, 4, 2, 1], 
        vec![1, 2, 7, 8, 9], 
        vec![9, 7, 6, 2, 1], 
        vec![1, 3, 2, 4, 5], 
        vec![8, 6, 4, 4, 1], 
        vec![1, 3, 6, 7, 9]
    ]);
}

#[test]
fn is_valid_increasing_test() {
    assert!(is_valid_increasing(&[1, 2, 3, 4, 5]));
    assert!(!is_valid_increasing(&[1, 2, 3, 5, 4]));
    assert!(!is_valid_increasing(&[1, 1, 1, 1, 1]));
    assert!(is_valid_increasing(&[3, 4, 7, 10, 13]));
}

#[test]
fn is_valid_decreasing_test() {
    assert!(is_valid_decreasing(&[5, 4, 3, 2, 1]));
    assert!(!is_valid_decreasing(&[5, 4, 3, 1, 2]));
    assert!(!is_valid_decreasing(&[1, 1, 1, 1, 1]));
    assert!(is_valid_decreasing(&[13, 10, 7, 4, 3]));
}

#[test]
fn generate_subsequences_test() {
    let input = parse(EXAMPLE);
    assert_eq_unordered!(subsequences_missing_one_entry(&input[0]), 
        vec![
            vec![7, 6, 4, 2],
            vec![7, 6, 4, 1], 
            vec![7, 6, 2, 1], 
            vec![7, 4, 2, 1], 
            vec![6, 4, 2, 1], 
        ]);
}
