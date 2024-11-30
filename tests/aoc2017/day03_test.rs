use aoc::aoc2017::day03::*;
use aoc::util::point::*;

#[test]
fn part1_test() {
    assert_eq!(part1(&parse("1")), 0);
    assert_eq!(part1(&parse("12")), 3);
    assert_eq!(part1(&parse("23")), 2);
    assert_eq!(part1(&parse("1024")), 31);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse("11")), 23);
    assert_eq!(part2(&parse("23")), 25);
    assert_eq!(part2(&parse("50")), 54);
    assert_eq!(part2(&parse("200")), 304);
}

#[test]
fn test_spiral_sum_sequence() {
    let sequence: Vec<i64> = spiral_sum_sequence().take(23).collect();
    assert_eq!(sequence, vec![1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747, 806]);
}

#[test]
fn test_spiral() {
    let spiral_seq: Vec<Point> = spiral().take(10).collect();
    assert_eq!(spiral_seq, vec![Point::new(0,0), Point::new(1,0), Point::new(1,-1), Point::new(0,-1), Point::new(-1,-1), Point::new(-1,0), Point::new(-1,1), Point::new(0,1), Point::new(1,1), Point::new(2,1)]);
}