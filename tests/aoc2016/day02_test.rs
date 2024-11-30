use aoc::aoc2016::day02::*;

const EXAMPLE: &str = "\
ULL
RRDDD
LURDL
UUUUD";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "1985".to_string());
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "5DB3".to_string());
}
