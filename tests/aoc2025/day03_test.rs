use aoc::aoc2025::day03::*;

const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 357);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 3121910778619);
}

#[test]
fn test_parsing_data() {
    assert_eq!(extract_highest("987654321111111", 2), 98);
    assert_eq!(extract_highest("811111111111119", 2), 89);
    assert_eq!(extract_highest("234234234234278", 2), 78);
    assert_eq!(extract_highest("818181911112111", 2), 92);

    assert_eq!(extract_highest("987654321111111", 12), 987654321111);
    assert_eq!(extract_highest("811111111111119", 12), 811111111119);
    assert_eq!(extract_highest("234234234234278", 12), 434234234278);
    assert_eq!(extract_highest("818181911112111", 12), 888911112111);
}