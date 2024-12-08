use crate::util::grid::*;
use crate::util::point::*;
use itertools::Itertools;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u32 {
    create_antinodes(input, false).bytes.iter().filter(|&&b| b == b'#').count() as u32
}

pub fn part2(input: &Grid<u8>) -> u32 {
    create_antinodes(input, true).bytes.iter().filter(|&&b| b == b'#').count() as u32
}

pub fn create_antinodes(input: &Grid<u8>, extended: bool) -> Grid<u8> {
    let mut antinode_grid = Grid::new(input.width, input.height, b'.');
    
    let points_by_value: Vec<(u8, Vec<Point>)> = input.points()
        .filter(|&p| input[p] != b'.')
        .map(|p| (input[p], p))
        .into_group_map()
        .into_iter()
        .collect();

    for (_, points) in points_by_value {
        for pair in points.into_iter().combinations(2) {
            let (p1, p2) = (pair[0], pair[1]);
            let antinodes = if extended {
                find_extended_antinode_points(p1, p2, input)
            } else {
                find_antinode_points(p1, p2, input)
            };
            
            for antinode in antinodes {
                antinode_grid[antinode] = b'#';
            }
        }
    }

    antinode_grid
}

fn find_antinode_points(p1: Point, p2: Point, grid: &Grid<u8>) -> Vec<Point> {
    let direction = p2 - p1;
    let mut antinodes = Vec::new();
    
    // Check both potential antinode points
    let antinode1 = p1 - direction;
    let antinode2 = p2 + direction;
    
    if grid.contains(antinode1) {
        antinodes.push(antinode1);
    }
    if grid.contains(antinode2) {
        antinodes.push(antinode2);
    }
    
    antinodes
}

// p1 and p2 are now points for antinodes, and any multiple of the distances between them that exist in the grid.
// this could possibly be merged with previous function, but I can't be arsed
fn find_extended_antinode_points(p1: Point, p2: Point, grid: &Grid<u8>) -> Vec<Point> {
    let direction = p2 - p1;
    let mut points = Vec::new();
    
    let mut current = p1;
    while grid.contains(current) {
        points.push(current);
        current = current - direction;
    }
    
    current = p2;
    while grid.contains(current) {
        points.push(current);
        current = current + direction;
    }
    
    points
}