use aoc::aoc2024::day18::*;
use pretty_assertions::assert_eq;
use aoc::util::grid::*;
use aoc::util::iter::*;
use aoc::util::point::*;
use aoc::util::parse::*;

const INPUT1: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[test]
fn part1_test() {
}

#[test]
fn part2_test() {
}

#[test]
fn do_bfs_test_part1() {
    let mut grid = Grid::new(7, 7, u16::MAX);
    for (i, [x, y]) in INPUT1.iter_signed().chunk::<2>().enumerate() {
        grid[Point::new(x, y)] = i as u16;
    }

    let result = do_bfs(&grid, 12);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 22);
}
