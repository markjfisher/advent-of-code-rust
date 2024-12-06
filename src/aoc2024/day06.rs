use crate::util::{grid::*, point::*, hash::*};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn walk_grid(input: &Grid<u8>) -> Result<FastSet<Point>, FastSet<Point>> {
    let mut visited = FastSet::new();
    let mut guard_location = input.find(b'^').unwrap();
    let mut guard_direction = UP;
    let mut is_in_grid = true;
    
    visited.insert((guard_location, guard_direction));

    while is_in_grid {
        let new_location = guard_location + Point::from(guard_direction);
        if !input.contains(new_location) {
            is_in_grid = false;
            break;
        }

        if input[new_location] == b'#' {
            guard_direction = guard_direction.clockwise();
        } else {
            guard_location = new_location;
        }
        
        if !visited.insert((guard_location, guard_direction)) {
            break;
        }
    }

    let unique_positions: FastSet<_> = visited.iter()
        .map(|(pos, _)| *pos)
        .collect();
    
    if is_in_grid {
        Err(unique_positions)
    } else {
        Ok(unique_positions)
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
            walk_grid(&modified_grid).is_err()
        })
        .count() as u32
}