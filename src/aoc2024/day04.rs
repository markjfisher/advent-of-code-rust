use crate::util::{grid::Grid, point::{Point, DIAGONAL, JUST_DIAGONALS}};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

// Search for "XMAS" in a single direction from every "X" in the grid
pub fn part1(input: &Grid<u8>) -> u32 {
    let mut result = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            if input[Point::new(x, y)] == b'X' {
                for &direction in &DIAGONAL {
                    if check_for_xmas(input, Point::new(x, y), direction, 1) {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

// We are searching from every "A" in the grid for the letters MAS in an "X" shape
// This boils down to checking the 4 corners for any of the 4 possible patterns
// e.g.
//  M.S
//  .A.
//  M.S
// which is effectively checking "MSSM" in a clockwise direction from the top left
// The other patters allowed are simply rotations of this
// Note we have to ensure we use the same order as the value JUST_DIAGONALS, which is UL, UR, DR, DL
pub fn part2(input: &Grid<u8>) -> u32 {
    let mut result = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            let center = Point::new(x, y);
            if input[center] == b'A' && check_for_mas_x(input, center) {
                result += 1;
            }
        }
    }
    result
}

fn check_for_mas_x(grid: &Grid<u8>, center: Point) -> bool {
    let patterns = [
        b"MSSM",
        b"SSMM",
        b"SMMS",
        b"MMSS",
    ];

    let mut corners = Vec::with_capacity(4);
    
    // Check if all corners exist and collect their values
    for &d in &JUST_DIAGONALS {
        let corner = center + d;
        if !grid.contains(corner) {
            // fast exit as the corner isn't on the grid (like TRON)
            return false;
        }
        corners.push(grid[corner]);
    }

    // Now check if they match any of the allowed patterns
    // This uses the slice function [..] to make the types the same so they can be compared
    // This is the whole range version of something like pattern[1..3]
    // pattern is &[u8; 4], and the slice returns a &[u8]. The first can't be compared to Vec<u8> but the second can
    // This is a slightly annoying thing to watch out for in rust
    patterns.iter().any(|pattern| corners == pattern[..])
}

// First recursive function this year! :tada:
// For future self: point and direction (Point objects) are Copy types, so we don't need to pass them by
// reference as they are cheap to use this way. Using references would take as much memory.
// usize is the same (Copy and cheap) so again we don't use a reference
fn check_for_xmas(grid: &Grid<u8>, point: Point, direction: Point, index: usize) -> bool {
    let word = b"XMAS";
    if index == word.len() {
        return true;
    }

    let next_point = point + direction;
    if grid.contains(next_point) && grid[next_point] == word[index] {
        return check_for_xmas(grid, next_point, direction, index + 1);
    }

    false
}
