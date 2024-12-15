use aoc::aoc2024::day15::*;
use aoc::util::point::*;
use indoc::indoc;
use pretty_assertions::assert_eq;

const EXAMPLE1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

const EXAMPLE2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 2028);
    assert_eq!(part1(&parse(EXAMPLE2)), 10092);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE2)), 9021);
}

#[test]
fn parse_test1() {
    let (grid, moves) = parse(EXAMPLE1);
    assert_eq!(grid.width, 8);
    assert_eq!(grid.height, 8);
    assert_eq!(moves.len(), 15);

    assert_eq!(grid[Point::new(0, 0)], b'#');
    assert_eq!(grid[Point::new(2, 2)], b'@');
}

#[test]
fn parse_test2() {
    let (grid, moves) = parse(EXAMPLE2);
    assert_eq!(grid.width, 10);
    assert_eq!(grid.height, 10);
    assert_eq!(moves.len(), 700);
    assert_eq!(grid[Point::new(0, 0)], b'#');
    assert_eq!(grid[Point::new(4, 4)], b'@');
}

#[test]
fn move_test_individual_moves() {
    let (mut grid, _moves) = parse(EXAMPLE1);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########"});
    do_moves(&mut grid, &vec![LEFT]);

    // first move is blocked by wall
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########"});

    // second move is up into space
    do_moves(&mut grid, &vec![UP]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #.@O.O.#
        ##..O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########"});

    // 3rd move is up - hits wall
    do_moves(&mut grid, &vec![UP]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #.@O.O.#
        ##..O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########"});

    // 4th move is right - pushes 1 box
    do_moves(&mut grid, &vec![RIGHT]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #..@OO.#
        ##..O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########"});

    // 5th move is right - pushes 2 boxes
    do_moves(&mut grid, &vec![RIGHT]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #...@OO#
        ##..O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########"});

    // 6th move is down - pushes 4 boxes
    do_moves(&mut grid, &vec![DOWN]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##..@..#
        #...O..#
        #.#.O..#
        #...O..#
        #...O..#
        ########"});

    // 7th move is down - pushes nothing
    do_moves(&mut grid, &vec![DOWN]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##..@..#
        #...O..#
        #.#.O..#
        #...O..#
        #...O..#
        ########"});

    // 8th move is left into space
    do_moves(&mut grid, &vec![LEFT]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##.@...#
        #...O..#
        #.#.O..#
        #...O..#
        #...O..#
        ########"});

    // 9th move is down into space
    do_moves(&mut grid, &vec![DOWN]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##.....#
        #..@O..#
        #.#.O..#
        #...O..#
        #...O..#
        ########"});

    // 10th move is RIGHT moving 1 box
    do_moves(&mut grid, &vec![RIGHT]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##.....#
        #...@O.#
        #.#.O..#
        #...O..#
        #...O..#
        ########"});

    // 11th move is RIGHT moving 1 box
    do_moves(&mut grid, &vec![RIGHT]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##.....#
        #....@O#
        #.#.O..#
        #...O..#
        #...O..#
        ########"});

    // 12th move is down into space
    do_moves(&mut grid, &vec![DOWN]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##.....#
        #.....O#
        #.#.O@.#
        #...O..#
        #...O..#
        ########"});

    // 13th move is left moving 1 box
    do_moves(&mut grid, &vec![LEFT]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##.....#
        #.....O#
        #.#O@..#
        #...O..#
        #...O..#
        ########"});

    // 14th move is left moving nothing
    do_moves(&mut grid, &vec![LEFT]);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##.....#
        #.....O#
        #.#O@..#
        #...O..#
        #...O..#
        ########"});

}

#[test]
fn move_test_all_together() {
    let (mut grid, moves) = parse(EXAMPLE1);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########"});
    do_moves(&mut grid, &moves);

    // after all moves, should be same as first test done individually
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ########
        #....OO#
        ##.....#
        #.....O#
        #.#O@..#
        #...O..#
        #...O..#
        ########"});

}

#[test]
fn move_larger_test() {
    let (mut grid, moves) = parse(EXAMPLE2);
    assert_eq!(grid.to_grid_string(), indoc! {"\
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########"});
    do_moves(&mut grid, &moves);

    assert_eq!(grid.to_grid_string(), indoc! {"\
        ##########
        #.O.O.OOO#
        #........#
        #OO......#
        #OO@.....#
        #O#.....O#
        #O.....OO#
        #O.....OO#
        #OO....OO#
        ##########"});
}

#[test]
fn can_widen_grid() {
    let (grid, _) = parse(EXAMPLE2);
    let wide_grid = wide_grid(&grid);
    assert_eq!(wide_grid.to_grid_string(), indoc! {"\
        ####################
        ##....[]....[]..[]##
        ##............[]..##
        ##..[][]....[]..[]##
        ##....[]@.....[]..##
        ##[]##....[]......##
        ##[]....[]....[]..##
        ##..[][]..[]..[][]##
        ##........[]......##
        ####################"});
}

#[test]
fn can_do_wide_moves() {
    let (grid, moves) = parse(EXAMPLE2);
    let mut wide_grid = wide_grid(&grid);
    let mut robot = wide_grid.find(b'@').unwrap();
    do_wide_moves(&mut wide_grid, &mut robot, &moves);

    assert_eq!(wide_grid.to_grid_string(), indoc! {"\
        ####################
        ##[].......[].[][]##
        ##[]...........[].##
        ##[]........[][][]##
        ##[]......[]....[]##
        ##..##......[]....##
        ##..[]............##
        ##..@......[].[][]##
        ##......[][]..[]..##
        ####################"});
}
