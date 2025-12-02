use aoc::aoc2025::day02::*;

const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 1227775554);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 4174379265);
}

#[test]
fn test_parsing_data() {
    assert_eq!(parse("1-31"), (33, 0)); // 11+22
    assert_eq!(parse("95-115"), (99, 0)); // just 99
    assert_eq!(parse("8000-20000"), (180790, 0)); // 8080+8181+..+9999
}
