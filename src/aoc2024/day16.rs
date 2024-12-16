use crate::util::grid::*;
use crate::util::hash::{FastMap, FastSet};
use crate::util::point::*;
use pathfinding::prelude::{astar_bag, astar};

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Reindeer {
    pos: Point,
    dir: Point,
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    let (_path, cost) = reindeer_path(grid, start, RIGHT, end).unwrap();
    // println!("{}", reindeer_path_to_string(grid, &path));
    cost
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    let all_points = all_reindeer_points(grid, start, RIGHT, end);

    // println!("{}", reindeer_points_to_string(grid, &all_points));
    // println!("not a wall count: {}, olympics path count: {}", grid.points().filter(|&p| grid[p] != b'#').count(), all_points.len());
    all_points.len() as u32
}

fn get_successors(r: &Reindeer, grid: &Grid<u8>) -> Vec<(Reindeer, u32)> {
    let mut potential_positions = vec![];

    let ahead = r.pos + r.dir;
    if grid.contains(ahead) && grid[ahead] != b'#' {
        potential_positions.push((Reindeer { pos: ahead, dir: r.dir }, 1u32));
    }

    // we only consider left and right turns. It also works with moving in those directions, but then doesn't print as pretty :D
    let left_dir = r.dir.counter_clockwise();
    if grid.contains(r.pos + left_dir) && grid[r.pos + left_dir] != b'#' {
        potential_positions.push((Reindeer { pos: r.pos, dir: left_dir }, 1000));
    }
    let right_dir = r.dir.clockwise();
    if grid.contains(r.pos + right_dir) && grid[r.pos + right_dir] != b'#' {
        potential_positions.push((Reindeer { pos: r.pos, dir: right_dir }, 1000));
    }

    // println!("pos: {:?}, dir: {:?}, potential_positions: {:?}", r.pos, r.dir, potential_positions);
    potential_positions
}

pub fn reindeer_path(grid: &Grid<u8>, start: Point, dir: Point, end: Point) -> Option<(Vec<Reindeer>, u32)> {
    let reindeer = Reindeer {
        pos: start,
        dir,
    };
    astar(
        &reindeer,
        |r| get_successors(r, grid),
        |r| r.pos.manhattan(end) as u32,
        |r| r.pos == end,
    )
}

pub fn all_reindeer_points(grid: &Grid<u8>, start: Point, dir: Point, end: Point) -> FastSet<Point> {
    let reindeer = Reindeer {
        pos: start,
        dir,
    };
    let (solution, _cost) = astar_bag(
        &reindeer,
        |r| get_successors(r, grid),
        |r| r.pos.manhattan(end) as u32,
        |r| r.pos == end,
    ).unwrap();

    solution.into_iter().fold(FastSet::default(), |mut set, rs| {
        set.extend(rs.iter().map(|r| r.pos));
        set
    })
}

pub fn reindeer_path_to_string(grid: &Grid<u8>, path: &[Reindeer]) -> String {
    let mut result = grid.clone();

    for reindeer in path {
        let dir_char = match reindeer.dir {
            UP => b'^',
            DOWN => b'v',
            LEFT => b'<',
            RIGHT => b'>',
            _ => panic!("Invalid direction"),
        };
        result[reindeer.pos] = dir_char;
    }

    // put start and end markers back over top
    result[grid.find(b'S').unwrap()] = b'S';
    result[grid.find(b'E').unwrap()] = b'E';

    let mut char_map = FastMap::default();
    char_map.insert(b'#', "█");
    char_map.insert(b'.', " ");
    result.to_grid_string_with_map(Some(&char_map))
}

pub fn reindeer_points_to_string(grid: &Grid<u8>, points: &FastSet<Point>) -> String {
    let mut result = grid.clone();
    for point in points {
        result[*point] = b'O';
    }
    let mut char_map = FastMap::default();
    char_map.insert(b'#', "█");
    char_map.insert(b'.', " ");
    result.to_grid_string_with_map(Some(&char_map))
}
