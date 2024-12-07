use aoc::aoc2024::day07::*;

const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

const EXAMPLE2: &str = "\
432832280199: 3 286 4 3 17 682 7 7 9 2
6606690226623: 605 42 3 47 26 6 622
4920367: 8 8 896 6 4 955 9 44 6 9
221825533: 124 5 714 905 66 5 33
";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 3749);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 11387);
}
