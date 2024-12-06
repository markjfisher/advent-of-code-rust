use crate::util::{grid::*, hash::*, point::*};
use rayon::prelude::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn walk_grid(input: &Grid<u8>) -> FastSet<Point> {
    let mut positions = FastSet::with_capacity((input.width * input.height) as usize);
    let mut guard_location = input.find(b'^').unwrap();
    let mut guard_direction = UP;

    positions.insert(guard_location);

    loop {
        let mut new_location = guard_location;

        loop {
            let next = new_location + Point::from(guard_direction);
            if !input.contains(next) {
                return positions;  // Found exit
            }
            if input[next] == b'#' {
                guard_direction = guard_direction.clockwise();
                break;
            }
            new_location = next;
            positions.insert(new_location);
        }
        guard_location = new_location;
    }
}

pub fn part1(input: &Grid<u8>) -> u32 {
    walk_grid(input).len() as u32
}

fn is_loop(input: &Grid<u8>, start_position: Point, blocked_point: Point) -> bool {
    let mut visited = FastSet::with_capacity(150);
    let mut guard_location = start_position;
    let mut guard_direction = UP;

    visited.insert((guard_location, guard_direction));

    loop {
        let mut new_location = guard_location;

        loop {
            let next = new_location + Point::from(guard_direction);
            if !input.contains(next) {
                return false;
            }
            if input[next] == b'#' || next == blocked_point {
                guard_direction = guard_direction.clockwise();
                if visited.contains(&(new_location, guard_direction)) {
                    return true;
                }
                visited.insert((new_location, guard_direction));
                // This break ensures we don't move before finding a direction that has a free location
                break;
            }
            new_location = next;
        }
        guard_location = new_location;
    }
}

pub fn part2(input: &Grid<u8>) -> u32 {
    let guard_pos = input.find(b'^').unwrap();

    // Use part1 to find all the locations the guard would walk, so we can put obstacles in their way!
    let mut initial_positions = walk_grid(input);
    initial_positions.remove(&guard_pos);

    // Replace every visited location (except the start which we removed already) with a block and test if they walk in a loop
    // Uses par_iter to parallelize the loop detection. Reduces the time from 23,000us to 2,300us (10x roughly)
    initial_positions.into_iter()
        .collect::<Vec<_>>()
        .par_iter()
        // .with_min_len(50) // has some benefit to the parallelization if we batch - leaving as a comment for futre self
        .filter(|&p| is_loop(input, guard_pos, *p))
        .count() as u32
}
