use aoc::aoc2024::day22::*;

const EXAMPLE1: &str = "\
1
10
100
2024";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part1(&input), 37327623);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part2(&input), 456);
}