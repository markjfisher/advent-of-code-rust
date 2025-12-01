use aoc::aoc2025::day01::*;

const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3);
}

// #[test]
// fn part2_test() {
//     let input = parse(EXAMPLE);
//     assert_eq!(part2(&input), 0);
// }

#[test]
fn test_parsing_lines_into_100_offset() {
    let seq = parse(EXAMPLE);
    assert_eq!(seq, vec![32, 70, 48, 95, 60, 45, 99, 1, 14, 18]);
}