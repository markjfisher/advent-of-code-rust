use crate::util::{grid::*, hash::*, point::*};
use rayon::prelude::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn walk_grid(input: &Grid<u8>) -> Option<FastSet<Point>> {
    let mut visited = vec![[false; 4]; (input.width * input.height) as usize];
    let mut positions = FastSet::with_capacity((input.width * input.height) as usize);
    let mut guard_location = input.find(b'^').unwrap();
    let mut guard_direction = UP;

    positions.insert(guard_location);
    visited[(guard_location.y * input.width + guard_location.x) as usize]
        [guard_direction.to_index()] = true;

    loop {
        let new_location = guard_location + Point::from(guard_direction);
        if !input.contains(new_location) {
            return Some(positions);
        }

        if input[new_location] == b'#' {
            guard_direction = guard_direction.clockwise();
        } else {
            guard_location = new_location;
            positions.insert(guard_location);
        }

        if visited[(guard_location.y * input.width + guard_location.x) as usize]
            [guard_direction.to_index()]
        {
            return None;
        }
        visited[(guard_location.y * input.width + guard_location.x) as usize]
            [guard_direction.to_index()] = true;
    }
}

pub fn part1(input: &Grid<u8>) -> u32 {
    walk_grid(input).unwrap_or_default().len() as u32
}

fn is_loop(input: &Grid<u8>, start_position: Point) -> bool {
    let mut visited = FastSet::with_capacity(100 as usize);
    let mut guard_location = start_position;
    let mut guard_direction = UP;

    // Only add turning points to visited set
    visited.insert((guard_location, guard_direction));

    loop {
        let mut new_location = guard_location;

        // Continue walking until we hit a wall or need to turn
        loop {
            let next = new_location + Point::from(guard_direction);
            if !input.contains(next) {
                return false; // Found exit
            }
            if input[next] == b'#' {
                guard_direction = guard_direction.clockwise();
                // Only store location and direction when we turn
                if visited.contains(&(new_location, guard_direction)) {
                    return true; // Found a loop
                }
                visited.insert((new_location, guard_direction));
                break;
            }
            new_location = next;
        }
        guard_location = new_location;
    }
}

pub fn part2(input: &Grid<u8>) -> u32 {
    let guard_pos = input.find(b'^').unwrap();
    let initial_positions: Vec<_> = walk_grid(input)
        .unwrap_or_default()
        .iter()
        .filter(|&&p| p != guard_pos)  // Remove guard position upfront
        .copied()
        .collect();
    
    initial_positions.par_iter()
        .with_min_len(500)
        .filter(|&p| {
            let mut modified_grid = input.clone();
            modified_grid[*p] = b'#';
            is_loop(&modified_grid, guard_pos)
        })
        .count() as u32
}
