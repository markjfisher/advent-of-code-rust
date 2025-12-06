use aoc::aoc2025::day06::*;

const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 4277556);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 3263827);
}

#[test]
fn test_parsing_data() {
    let (a, b) = parse(EXAMPLE);
    assert_eq!(a, 0);
    assert_eq!(b, 0);
}
