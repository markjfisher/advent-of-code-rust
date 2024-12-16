use crate::util::grid::*;
use crate::util::hash::{FastMap, FastSet};
use crate::util::point::*;
use pathfinding::prelude::{astar_bag, AstarSolution};

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Reindeer {
    pub pos: Point,
    pub dir: Point,
}

type Input = (Grid<u8>, AstarSolution<Reindeer>, u32);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    let reindeer = Reindeer {
        pos: start,
        dir: RIGHT,
    };
    let (solution, cost) = astar_bag(
        &reindeer,
        |r| get_successors(r, &grid),
        |r| r.pos.manhattan(end) as u32,
        |r| r.pos == end,
    ).unwrap();
    (grid, solution, cost)
}

pub fn part1(input: &Input) -> u32 {
    input.2
}

pub fn part2(input: &Input) -> u32 {
    let solution = input.1.clone();
    let all_points = all_reindeer_points(solution);
    all_points.len() as u32
}

pub fn get_successors(r: &Reindeer, grid: &Grid<u8>) -> Vec<(Reindeer, u32)> {
    let mut potential_positions = vec![];

    let ahead = r.pos + r.dir;
    if grid[ahead] != b'#' {
        potential_positions.push((Reindeer { pos: ahead, dir: r.dir }, 1u32));
    }

    // we could just use turns here, i.e. same position but in different direction, but 
    // using the next positions is quicker as there are less steps overall by combining turn and move into a single step
    let left = r.pos + r.dir.counter_clockwise();
    if grid[left] != b'#' {
        potential_positions.push((Reindeer { pos: left, dir: r.dir.counter_clockwise() }, 1001));
    }

    let right = r.pos + r.dir.clockwise();
    if grid[right] != b'#' {
        potential_positions.push((Reindeer { pos: right, dir: r.dir.clockwise() }, 1001));
    }

    let behind = r.pos - r.dir;
    if grid[behind] != b'#' {
        potential_positions.push((Reindeer { pos: behind, dir: r.dir.clockwise().clockwise() }, 2001));
    }

    // println!("pos: {:?}, dir: {:?}, potential_positions: {:?}", r.pos, r.dir, potential_positions);
    potential_positions
}

pub fn all_reindeer_points(solution: AstarSolution<Reindeer>) -> FastSet<Point> {
    solution.fold(FastSet::default(), |mut set, rs| {
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
