use aoc::aoc2024::day18::*;
use pretty_assertions::assert_eq;
use indoc::indoc;

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
    let input = parse(INPUT1);
    let grid = create_grid(&input, 7, 7, 12);
    assert_eq!(shortest_path(&grid).unwrap().1, 22);
}

#[test]
fn part2_test() {
    let input = parse(INPUT1);
    assert_eq!(part2(&input), 456);
}

#[test]
fn create_grid_test() {
    let input = parse(INPUT1);
    let grid = create_grid(&input, 7, 7, 12);
    assert_eq!(grid.width, 7);
    assert_eq!(grid.height, 7);
    // println!("{}", grid.to_grid_string());
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ...#...
        ..#..#.
        ....#..
        ...#..#
        ..#..#.
        .#..#..
        #.#...."});
}

