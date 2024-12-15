use crate::util::grid::*;
use crate::util::point::*;
use std::mem::swap;

pub fn parse(input: &str) -> (Grid<u8>, Vec<Point>) {
    let (g, m) = input.split_once("\n\n").unwrap();
    let grid = Grid::parse(g);
    let moves: Vec<Point> = m
        .chars()
        .filter(|&c| c != '\n' && c != '\r')
        .map(|c| Point::from(c as u8))
        .collect();

    (grid, moves)
}

pub fn part1(input: &(Grid<u8>, Vec<Point>)) -> u32 {
    let (grid, moves) = input;
    let mut grid = grid.clone();
    do_moves(&mut grid, &moves);
    score_grid(&grid, b'O')
}

pub fn move_object(grid: &mut Grid<u8>, pos: Point, dir: Point, object: u8) -> bool {
    let new_pos = pos + dir;

    // println!("\nmove_object: {:?} {:?} {}", pos, dir, object as char);
    // println!("new_pos: {:?}, grid[new_pos]: {}", new_pos, grid[new_pos] as char);
    match grid[new_pos] {
        b'#' => false, // Can't move into walls
        b'.' => {
            // Can move into empty space
            grid[pos] = b'.'; // Clear old position
            grid[new_pos] = object; // Move object to new position
            true
        }
        b'O' => {
            // Try to push box
            // println!("pushing box");
            if move_object(grid, new_pos, dir, b'O') {
                // println!("box moved");
                grid[pos] = b'.';
                grid[new_pos] = object;
                true
            } else {
                // println!("FAILED to move box");
                false
            }
        }
        _ => false,
    }
}

pub fn do_moves(grid: &mut Grid<u8>, moves: &Vec<Point>) {
    let mut robot = grid.find(b'@').unwrap();
    for &mv in moves {
        if move_object(grid, robot, mv, b'@') {
            robot = robot + mv;
        }
    }
}

pub fn score_grid(grid: &Grid<u8>, box_char: u8) -> u32 {
    grid.points()
        .filter(|p| grid[*p] == box_char)
        .map(|p| (p.x + p.y * 100) as u32)
        .sum()
}

// This time use bfs to move boxes rather than recursion as in part 1
pub fn part2(input: &(Grid<u8>, Vec<Point>)) -> u32 {
    let (grid, moves) = input;

    let mut grid = wide_grid(grid);
    let mut robot = grid.find(b'@').unwrap();

    do_wide_moves(&mut grid, &mut robot, moves);
    score_grid(&grid, b'[')
}

// convert from normal grid to wide box version for part 2
pub fn wide_grid(grid: &Grid<u8>) -> Grid<u8> {
    let mut next = Grid::new(grid.width * 2, grid.height, b'.');

    for y in 0..grid.height {
        for x in 0..grid.width {
            let (left, right) = match grid[Point::new(x, y)] {
                b'@' => (b'@', b'.'),
                b'O' => (b'[', b']'),
                b'#' => (b'#', b'#'),
                _ => continue,
            };

            next[Point::new(2 * x, y)] = left;
            next[Point::new(2 * x + 1, y)] = right;
        }
    }

    next
}

fn narrow(grid: &mut Grid<u8>, start: &mut Point, dir: Point) {
    let mut pos = *start + dir;
    let mut size = 2;

    while grid[pos] != b'.' && grid[pos] != b'#' {
        pos += dir;
        size += 1;
    }

    if grid[pos] == b'.' {
        let mut previous = b'.';
        let mut position = *start;

        for _ in 0..size {
            swap(&mut previous, &mut grid[position]);
            position += dir;
        }

        *start += dir;
    }
}

fn wide(
    grid: &mut Grid<u8>,
    start: &mut Point,
    dir: Point,
    todo: &mut Vec<Point>,
    seen: &mut Grid<usize>,
    id: usize,
) {
    let pos = *start;
    let next = pos + dir;

    if grid[next] == b'.' {
        grid[pos] = b'.';
        grid[next] = b'@';
        *start += dir;
        return;
    }

    todo.clear();
    todo.push(*start);
    let mut index = 0;

    while index < todo.len() {
        let next = todo[index] + dir;
        index += 1;

        let other = match grid[next] {
            b'#' => return,
            b'[' => RIGHT,
            b']' => LEFT,
            _ => continue,
        };

        // split the box into two halves
        let first = next;
        if seen[first] != id {
            seen[first] = id;
            todo.push(first);
        }

        let second = next + other;
        if seen[second] != id {
            seen[second] = id;
            todo.push(second);
        }
    }

    for &point in todo.iter().rev() {
        grid[point + dir] = grid[point];
        grid[point] = b'.';
    }

    *start += dir;
}

pub fn do_wide_moves(grid: &mut Grid<u8>, robot: &mut Point, moves: &Vec<Point>) {
    let mut todo = Vec::new();
    let mut seen = grid.same_size_with(usize::MAX);

    for (id, p) in moves.into_iter().enumerate() {
        match *p {
            LEFT => narrow(grid, robot, LEFT),
            RIGHT => narrow(grid, robot, RIGHT),
            UP => wide(grid, robot, UP, &mut todo, &mut seen, id),
            DOWN => wide(grid, robot, DOWN, &mut todo, &mut seen, id),
            _ => (),
        }
    }
}
