use aoc::aoc2024::day16::*;
use aoc::util::point::*;
use indoc::indoc;
use pretty_assertions::assert_eq;

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
    let grid = parse(EXAMPLE1);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    let (path, total_cost) = reindeer_path(&grid, start, RIGHT, end).unwrap();
    let result = reindeer_path_to_string(&grid, &path);
    // println!("total_cost: {}", total_cost);
    // println!("{}", result);
    assert_eq!(total_cost, 7036);
    assert_eq!(result, indoc! {"\
        ███████████████
        █       █    E█
        █ █ ███ █ ███^█
        █     █ █   █^█
        █ ███ █████ █^█
        █ █ █       █^█
        █ █ █████ ███^█
        █    >>>>>>v█^█
        ███ █^█████v█^█
        █   █^    █v█^█
        █ █ █^███ █v█^█
        █>>>>^█   █v█^█
        █^███ █ █ █v█^█
        █S  █     █>>^█
        ███████████████"});
}

#[test]
fn reindeer_points_to_string_test1() {
    let grid = parse(EXAMPLE1);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    let points = all_reindeer_points(&grid, start, RIGHT, end);
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
    let grid = parse(EXAMPLE2);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    let points = all_reindeer_points(&grid, start, RIGHT, end);
    let result = reindeer_points_to_string(&grid, &points);
    // println!("{}", result);
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