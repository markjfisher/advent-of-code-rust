use aoc::aoc2015::day02::*;

#[test]
// A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
fn example1() {
    assert_eq!(part1(&parse("2x3x4")), 58);
}

#[test]
// A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
fn example2() {
    assert_eq!(part1(&parse("1x1x10")), 43);
}

#[test]
// A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
fn example3() {
    assert_eq!(part2(&parse("2x3x4")), 34);
}

#[test]
// A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.
fn example4() {
    assert_eq!(part2(&parse("1x1x10")), 14);
}