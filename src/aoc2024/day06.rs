use crate::util::{grid::*, point::*, hash::*};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn walk_grid(input: &Grid<u8>) -> Option<FastSet<Point>> {
    let mut visited = vec![[false; 4]; (input.width * input.height) as usize];
    let mut positions = FastSet::with_capacity((input.width * input.height) as usize);
    let mut guard_location = input.find(b'^').unwrap();
    let mut guard_direction = UP;
    
    positions.insert(guard_location);
    visited[(guard_location.y * input.width + guard_location.x) as usize][guard_direction.to_index()] = true;

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
        
        if visited[(guard_location.y * input.width + guard_location.x) as usize][guard_direction.to_index()] {
            return None;
        }
        visited[(guard_location.y * input.width + guard_location.x) as usize][guard_direction.to_index()] = true;
    }
}

pub fn part1(input: &Grid<u8>) -> u32 {
    walk_grid(input).unwrap_or_default().len() as u32
}

pub fn part2(input: &Grid<u8>) -> u32 {
    let initial_positions = walk_grid(input).unwrap_or_default();
    
    initial_positions.iter()
        .filter(|&&p| input[p] == b'.')
        .filter(|&&p| {
            let mut modified_grid = input.clone();
            modified_grid[p] = b'#';
            walk_grid(&modified_grid).is_none()
        })
        .count() as u32
}