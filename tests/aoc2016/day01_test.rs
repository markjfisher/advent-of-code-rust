use aoc::aoc2016::day01::*;

#[test]
fn part1_test() {
    let i1 = parse("R2, L3");
    assert_eq!(part1(&i1), 5);
    let i2 = parse("R2, R2, R2");
    assert_eq!(part1(&i2), 2);
    let i3 = parse("R5, L5, R5, R3");
    assert_eq!(part1(&i3), 12);
}

#[test]
fn part2_test() {
    let i1 = parse("R8, R4, R4, R8");
    assert_eq!(part2(&i1), 4);
}
