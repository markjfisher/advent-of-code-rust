use aoc::aoc2024::day03::*;

const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 161);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 48);
}

#[test] 
fn extract_muls_test() {
    let s = "text mul(2,345) more mul(123,45) invalid mul(1234,5)";
    assert_eq!(extract_muls(s), vec![(2, 345), (123, 45)]);
}