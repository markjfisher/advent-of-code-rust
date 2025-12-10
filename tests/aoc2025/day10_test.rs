use aoc::aoc2025::day10::*;

const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 7);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 33);
}

#[test]
fn test_parsing_data() {
    let machine = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    assert_eq!(machine.num_lights, 4);
    assert_eq!(machine.target, 6);
    assert_eq!(machine.button_masks, vec![8, 10, 4, 12, 5, 3]);
    assert_eq!(machine.joltages, vec![3, 5, 4, 7]);
}

#[test]
fn test_min_light_presses() {
    let machine = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    let min_presses = min_light_presses(&machine);
    assert_eq!(min_presses, Some(2));
}

#[test]
fn test_min_joltage_presses() {
    let machine = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    // let min_presses_alg = min_joltage_presses_alg(&machine);
    let min_presses_z3 = min_joltage_presses_z3(&machine);
    // assert_eq!(min_presses_alg, min_presses_z3);
    assert_eq!(min_presses_z3, Some(10));
}

#[test]
fn test_min_joltage_presses_n_plus_1() {
    let machine = parse_machine("[.#######] (0,3,7) (0,3,4,5,6) (0,1,2,6) (1,2,3,5,6,7) (0) (0,1,2,4,5,6) (1,3,4,5) (0,2,3,5) (4,7) {75,42,53,50,41,58,44,26}");
    // let min_presses_alg = min_joltage_presses_alg(&machine);
    let min_presses_z3 = min_joltage_presses_z3(&machine);
    // assert_eq!(min_presses_alg, min_presses_z3);
    assert_eq!(min_presses_z3, Some(89));
}

#[test]
fn test_min_joltage_presses_n_minus_2() {
    let machine = parse_machine("[.#..##...#] (1,3,4,6,8) (3,6,7,8) (0,1,9) (0,3,8) (5,6) (0,1,2,4,5,6,7,9) (1,2,3,7,9) (2) {25,37,37,19,24,18,32,24,15,28}");
    // let min_presses_alg = min_joltage_presses_alg(&machine);
    let min_presses_z3 = min_joltage_presses_z3(&machine);
    // assert_eq!(min_presses_alg, min_presses_z3);
    assert_eq!(min_presses_z3, Some(64));
}

#[test]
fn test_min_joltage_presses_n_minus_1() {
    let machine = parse_machine("[#.###.##.] (1,3,6,7) (0,3,5,6,7) (1,3) (1,6) (0,1,2,4,7,8) (0,1,3,4,6,7) (0,2,3,4,5,6) (1,2,3,4,5) {33,231,23,237,35,30,38,44,7}");
    // let min_presses_alg = min_joltage_presses_alg(&machine);
    let min_presses_z3 = min_joltage_presses_z3(&machine);
    // assert_eq!(min_presses_alg, min_presses_z3);
    assert_eq!(min_presses_z3, Some(245));
}

#[test]
fn test_min_joltage_presses_bad_case_1() {
    // this caused issues in the pivot table
    let machine = parse_machine("[#....] (0,2) (0,1,4) (0) (0,4) (0,3,4) (0,1,2,3) {53,16,16,24,27}");
    // let min_presses_alg = min_joltage_presses_alg(&machine);
    let min_presses_z3 = min_joltage_presses_z3(&machine);
    // assert_eq!(min_presses_alg, min_presses_z3);
    assert_eq!(min_presses_z3, Some(53));
}

// Takes a while to run in the IDE (5s) as it does each one in turn. 
// This was used to find cases that didn't work with the linear algebra solution.
// Also validates the Z3 solution against the linear algebra solution.
#[test]
fn test_min_joltage_presses_real_data_alg_vs_z3() {
    let input = include_str!("../../input/aoc2025/day10.txt");
    
    let machines: Vec<Machine> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_machine)
        .collect();

    for machine in machines.iter() {
        // let min_presses_alg = min_joltage_presses_alg(&machine);
        let min_presses_z3 = min_joltage_presses_z3(&machine);
        // assert_eq!(min_presses_alg, min_presses_z3);
        assert_ne!(min_presses_z3, None)
    }
}
