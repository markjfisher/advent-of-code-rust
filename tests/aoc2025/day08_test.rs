use aoc::aoc2025::day08::*;

const EXAMPLE: &str = "\
";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 0);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 0);
}

#[test]
fn test_parsing_data() {
    let (a, b) = parse(EXAMPLE);
    assert_eq!(a, 0);
    assert_eq!(b, 0);
}
