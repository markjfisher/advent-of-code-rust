use aoc::aoc2024::day20::*;
use aoc::util::grid::*;

const EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[test]
fn part1_test() {
    let grid = Grid::parse(EXAMPLE);
    let bfs_times = bfs_times(&grid);
    assert_eq!(count_improvements(&bfs_times, 2, 40).0, 2);
    assert_eq!(count_improvements(&bfs_times, 2, 38).0, 3);
    assert_eq!(count_improvements(&bfs_times, 2, 36).0, 4);
    assert_eq!(count_improvements(&bfs_times, 2, 20).0, 5);
    assert_eq!(count_improvements(&bfs_times, 2, 12).0, 8);
    assert_eq!(count_improvements(&bfs_times, 2, 10).0, 10);
    assert_eq!(count_improvements(&bfs_times, 2, 8).0, 14);
    assert_eq!(count_improvements(&bfs_times, 2, 6).0, 16);
    assert_eq!(count_improvements(&bfs_times, 2, 4).0, 30);
    assert_eq!(count_improvements(&bfs_times, 2, 2).0, 44);
}

#[test]
fn part2_test() {
    let grid = Grid::parse(EXAMPLE);
    let bfs_times = bfs_times(&grid);
    assert_eq!(count_improvements(&bfs_times, 20, 76).1, 3);
    assert_eq!(count_improvements(&bfs_times, 20, 74).1, 7);
    assert_eq!(count_improvements(&bfs_times, 20, 72).1, 29);
    assert_eq!(count_improvements(&bfs_times, 20, 70).1, 41);
    assert_eq!(count_improvements(&bfs_times, 20, 68).1, 55);
    assert_eq!(count_improvements(&bfs_times, 20, 66).1, 67);
    assert_eq!(count_improvements(&bfs_times, 20, 64).1, 86);
    assert_eq!(count_improvements(&bfs_times, 20, 62).1, 106);
    assert_eq!(count_improvements(&bfs_times, 20, 60).1, 129);
    assert_eq!(count_improvements(&bfs_times, 20, 58).1, 154);
    assert_eq!(count_improvements(&bfs_times, 20, 56).1, 193);
    assert_eq!(count_improvements(&bfs_times, 20, 54).1, 222);
    assert_eq!(count_improvements(&bfs_times, 20, 52).1, 253);
    assert_eq!(count_improvements(&bfs_times, 20, 50).1, 285);
}