use aoc::aoc2024::day16::*;
use indoc::indoc;
use pretty_assertions::assert_eq;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

const EXAMPLE1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 7036);
    assert_eq!(part1(&parse(EXAMPLE2)), 11048);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE1)), 45);
    assert_eq!(part2(&parse(EXAMPLE2)), 64);
}

#[test]
fn reindeer_path_to_string_test() {
    let (grid, solution, _total_cost) = parse(EXAMPLE1);

    let paths: Vec<_> = solution.into_iter().collect();
    assert_eq!(paths.len(), 3, "Expected exactly 3 possible paths");

    // they come back in random order, so check their hashes equate
    let mut actual_hashes: Vec<_> = paths.iter()
        .map(|p| hash(&reindeer_path_to_string(&grid, p)))
        .collect();
    actual_hashes.sort();

    let expected1 = indoc! {"\
        ███████████████
        █       █    E█
        █ █ ███ █ ███^█
        █     █ █   █^█
        █ ███ █████ █^█
        █ █ █       █^█
        █ █ █████ ███^█
        █  ^>>>>>>>>█^█
        ███^█ █████v█^█
        █  ^█     █v█^█
        █ █^█ ███ █v█^█
        █^>>  █   █v█^█
        █^███ █ █ █v█^█
        █S  █     █v>>█
        ███████████████"};

    let expected2 = indoc! {"\
        ███████████████
        █       █    E█
        █ █ ███ █ ███^█
        █     █ █   █^█
        █ ███ █████ █^█
        █ █ █       █^█
        █ █ █████ ███^█
        █    ^>>>>>>█^█
        ███ █^█████v█^█
        █   █^    █v█^█
        █ █ █^███ █v█^█
        █^>>>>█   █v█^█
        █^███ █ █ █v█^█
        █S  █     █v>>█
        ███████████████"};

    let expected3 = indoc! {"\
        ███████████████
        █       █    E█
        █ █ ███ █ ███^█
        █     █ █   █^█
        █ ███ █████ █^█
        █ █ █       █^█
        █ █ █████ ███^█
        █  ^>>>>>>>>█^█
        ███^█ █████v█^█
        █^>>█     █v█^█
        █^█ █ ███ █v█^█
        █^    █   █v█^█
        █^███ █ █ █v█^█
        █S  █     █v>>█
        ███████████████"};

    let mut expected_hashes: Vec<_> = vec![expected1, expected2, expected3]
        .into_iter()
        .map(|s| hash(s))
        .collect();
    expected_hashes.sort();

    assert_eq!(actual_hashes, expected_hashes, 
        "Path hashes don't match expected solutions");
}

#[test]
fn reindeer_points_to_string_test1() {
    let (grid, solution, _cost) = parse(EXAMPLE1);
    let points = all_reindeer_points(solution.clone());
    let result = reindeer_points_to_string(&grid, &points);
    // println!("{}", result);
    assert_eq!(result, indoc! {"\
        ███████████████
        █       █    O█
        █ █ ███ █ ███O█
        █     █ █   █O█
        █ ███ █████ █O█
        █ █ █       █O█
        █ █ █████ ███O█
        █  OOOOOOOOO█O█
        ███O█O█████O█O█
        █OOO█O    █O█O█
        █O█O█O███ █O█O█
        █OOOOO█   █O█O█
        █O███ █ █ █O█O█
        █O  █     █OOO█
        ███████████████"});
}

#[test]
fn reindeer_points_to_string_test2() {
    let (grid, solution, total_cost) = parse(EXAMPLE2);
    let points = all_reindeer_points(solution.clone());
    let result = reindeer_points_to_string(&grid, &points);
    // println!("{}", result);
    assert_eq!(total_cost, 11048);
    assert_eq!(result, indoc! {"\
        █████████████████
        █   █   █   █  O█
        █ █ █ █ █ █ █ █O█
        █ █ █ █   █   █O█
        █ █ █ █ ███ █ █O█
        █OOO█ █ █     █O█
        █O█O█ █ █ █████O█
        █O█O  █ █ █OOOOO█
        █O█O█████ █O███O█
        █O█O█  OOOOO█OOO█
        █O█O███O█████O███
        █O█O█OOO█  OOO█ █
        █O█O█O█████O███ █
        █O█O█OOOOOOO  █ █
        █O█O█O█████████ █
        █O█OOO          █
        █████████████████"});
}