use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use pathfinding::prelude::dijkstra;

pub fn parse(input: &str) -> Vec<(i32, i32)> {
    input.iter_unsigned().chunk::<2>().map(|c: [u32; 2]| (c[0] as i32, c[1] as i32)).collect()
}

pub fn part1(input: &[(i32, i32)]) -> u32 {
    let grid: Grid<u8> = create_grid(input, 71, 71, 1024);
    shortest_path(&grid).unwrap().1
}

// brute force find the first block that blocks the path using dijkstra instead of A* in day16
pub fn part2(input: &[(i32, i32)]) -> String {
    let mut grid: Grid<u8> = create_grid(input, 71, 71, 1024);

    // Create iterator over the remaining blocks
    for &next_block in input.iter().skip(1024) {
        grid[Point::new(next_block.0, next_block.1)] = b'#';

        if shortest_path(&grid).is_none() {
            // Path is blocked, return the coordinates as a comma-separated string
            return format!("{},{}", next_block.0, next_block.1);
        }
    }

    "0,0".to_string() // Fallback return if no blocking occurs rather than throwing an exception
}

pub fn create_grid(input: &[(i32, i32)], width: i32, height: i32, count: usize) -> Grid<u8> {
    let mut grid = Grid::new(width, height, b'.');
    for (x, y) in input.iter().take(count) {
        grid[Point::new(*x, *y)] = b'#';
    }
    grid
}

pub fn shortest_path(grid: &Grid<u8>) -> Option<(Vec<Point>, u32)> {
    let start = Point::new(0, 0);   
    let end = Point::new(grid.width - 1, grid.height - 1);
    let x: Option<(Vec<Point>, u32)> = dijkstra(
        &start,
        |p| ORTHOGONAL.iter()
            .filter_map(|d| {
                let next = *p + *d;
                if grid.contains(next) && grid[next] != b'#' {
                    Some((next, 1u32)) // simple cost
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        |p| *p == end);
    x
}