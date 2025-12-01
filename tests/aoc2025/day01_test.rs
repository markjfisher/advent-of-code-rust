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
    let seq = parse(EXAMPLE);
    assert_eq!(seq.0, vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32]);
    assert_eq!(seq.1, vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 1]);
}

#[test]
fn test_parsing_data2() {
    assert_eq!(do_rotations("R1", 50), (vec![51], vec![0]));
    assert_eq!(do_rotations("R50", 50), (vec![0], vec![1]));
    assert_eq!(do_rotations("R51", 50), (vec![1], vec![1]));
    assert_eq!(do_rotations("R150", 50), (vec![0], vec![2]));
    assert_eq!(do_rotations("R151", 50), (vec![1], vec![2]));

    assert_eq!(do_rotations("L1", 50), (vec![49], vec![0]));
    assert_eq!(do_rotations("L50", 50), (vec![0], vec![1]));
    assert_eq!(do_rotations("L51", 50), (vec![99], vec![1]));
    assert_eq!(do_rotations("L150", 50), (vec![0], vec![2]));
    assert_eq!(do_rotations("L151", 50), (vec![99], vec![2]));
    assert_eq!(do_rotations("L766", 0), (vec![34], vec![7]));
}