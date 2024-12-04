use aoc::aoc2024::day03::*;

#[test]
fn part1_test() {
    let input = parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    assert_eq!(part1(&input), 161);
}

#[test]
fn part2_test() {
    let input = parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    assert_eq!(part2(&input), 48);
}

#[test] 
fn extract_muls_test() {
    assert_eq!(extract_muls("text mul(2,345) more mul(123,45) invalid mul(1234,5)"), vec![(2, 345, true), (123, 45, true)]);
    assert_eq!(extract_muls("mul(2,345) don't() mul(123,45) do() mul(1234,5)mul(1,2)"), vec![(2, 345, true), (123, 45, false), (1, 2, true)]);
}

#[test]
fn extract_muls_tr_test() {
    assert_eq!(extract_muls_tr("text mul(2,345) more mul(123,45) invalid mul(1234,5)"), vec![(2, 345, true), (123, 45, true)]);
    assert_eq!(extract_muls_tr("mul(2,345) don't() mul(123,45) do() mul(1234,5)mul(1,2)"), vec![(2, 345, true), (123, 45, false), (1, 2, true)]);
}
