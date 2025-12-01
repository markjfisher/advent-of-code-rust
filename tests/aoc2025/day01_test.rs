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

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 6);
}

#[test]
fn test_parsing_data() {
    let (exact_zeros, total_crossings) = parse(EXAMPLE);
    // From the expected sequences: 3 exact zeros, 6 total crossings
    assert_eq!(exact_zeros, 3);
    assert_eq!(total_crossings, 6);
}

#[test]
fn test_parsing_data2() {
    // Test individual moves - checking crossing counts
    assert_eq!(do_rotations("R1", 50), (0, 0));   // No zero, no crossing
    assert_eq!(do_rotations("R50", 50), (1, 1));  // Ends at 0, crossed once
    assert_eq!(do_rotations("R51", 50), (0, 1));  // No zero, crossed once
    assert_eq!(do_rotations("R150", 50), (1, 2)); // Ends at 0, crossed twice
    assert_eq!(do_rotations("R151", 50), (0, 2)); // No zero, crossed twice
    assert_eq!(do_rotations("R100", 0), (1, 1));  // Ends at 0, crossed once

    assert_eq!(do_rotations("L1", 50), (0, 0));   // No zero, no crossing
    assert_eq!(do_rotations("L50", 50), (1, 1));  // Ends at 0, crossed once
    assert_eq!(do_rotations("L51", 50), (0, 1));  // No zero, crossed once
    assert_eq!(do_rotations("L150", 50), (1, 2)); // Ends at 0, crossed twice
    assert_eq!(do_rotations("L151", 50), (0, 2)); // No zero, crossed twice
    assert_eq!(do_rotations("L100", 0), (1, 1));  // Ends at 0, clicked 1 times
    assert_eq!(do_rotations("L766", 0), (0, 7));  // No zero, crossed 7 times
}