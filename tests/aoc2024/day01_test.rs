use aoc::aoc2024::day01::*;

const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 11);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 31);
}

#[test]
fn test_split_odd_even_indices() {
    let input = vec![6, 2, 1, 4, 5, 3];
    let (odd_indices, even_indices) = split_odd_even_indices(&input);
    assert_eq!(odd_indices, vec![6, 1, 5]);
    assert_eq!(even_indices, vec![2, 4, 3]);
}