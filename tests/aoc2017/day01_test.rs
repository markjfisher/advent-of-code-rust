use aoc::aoc2017::day01::*;

#[test]
fn part1_test() {
    assert_eq!(part1(&parse("1122")), 3);
    assert_eq!(part1(&parse("1111")), 4);
    assert_eq!(part1(&parse("1234")), 0);
    assert_eq!(part1(&parse("91212129")), 9);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse("1212")), 6);
    assert_eq!(part2(&parse("1221")), 0);
    assert_eq!(part2(&parse("123425")), 4);
    assert_eq!(part2(&parse("123123")), 12);
    assert_eq!(part2(&parse("12131415")), 4);
}