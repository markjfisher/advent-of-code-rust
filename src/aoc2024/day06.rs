use crate::util::{grid::*, point::*, hash::*};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn walk_grid(input: &Grid<u8>) -> Result<u32, u32> {
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
        Err(unique_positions.len() as u32)
    } else {
        Ok(unique_positions.len() as u32)
    }
}

pub fn part1(input: &Grid<u8>) -> u32 {
    walk_grid(input).unwrap_or_default()
}

pub fn part2(input: &Grid<u8>) -> u32 {
    input.points()
        .filter(|&p| input[p] == b'.')
        .filter(|&p| {
            let mut with_obstacle = input.clone();
            with_obstacle[p] = b'#';
            walk_grid(&with_obstacle).is_err()
        })
        .count() as u32
}