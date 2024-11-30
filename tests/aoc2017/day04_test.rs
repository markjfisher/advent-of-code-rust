use aoc::aoc2017::day04::*;


#[test]
fn part1_test() {
    assert_eq!(part1(&parse("aa bb cc dd ee")), 1);
    assert_eq!(part1(&parse("aa bb cc dd aa")), 0);
    assert_eq!(part1(&parse("aa bb cc dd aaa")), 1);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse("abcde fghij")), 1);
    assert_eq!(part2(&parse("abcde xyz ecdab")), 0);
    assert_eq!(part2(&parse("a ab abc abd abf abj")), 1);
    assert_eq!(part2(&parse("iiii oiii ooii oooi oooo")), 1);
    assert_eq!(part2(&parse("oiii ioii iioi iiio")), 0);
}