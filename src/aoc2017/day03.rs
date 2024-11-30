use crate::util::{parse::ParseOps, point::*, hash::*};

pub fn parse(input: &str) -> u32 {
    input.unsigned()
}

pub fn part1(input: &u32) -> u32 {
    spiral().nth(*input as usize - 1).unwrap().manhattan(ORIGIN) as u32
}

pub fn part2(input: &u32) -> u32 {
    spiral_sum_sequence()
        .find(|&x| x > *input as i64)
        .unwrap() as u32
}

pub fn spiral() -> impl Iterator<Item = Point> {
    let mut current = ORIGIN;
    let mut step = 1;
    let mut direction = RIGHT;
    let mut steps_in_current_direction = 0;

    std::iter::from_fn(move || {
        let result = current;
        
        // Move to next position immediately after yielding current
        current = current + direction;
        steps_in_current_direction += 1;

        // Check if we need to change direction
        if steps_in_current_direction == step {
            direction = direction.counter_clockwise();
            steps_in_current_direction = 0;
            
            // Increase step size after completing two sides
            if direction.x != 0 {
                step += 1;
            }
        }

        Some(result)
    })
}

pub fn spiral_sum_sequence() -> impl Iterator<Item = i64> {
    let mut points = FastMap::new();
    
    // Initialize with first value at origin
    points.insert(ORIGIN, 1);
    
    // Combine first value with rest of the sequence
    std::iter::once(1).chain(
        spiral()
            .skip(1)
            .map(move |point| {
                let sum = point.adjacent_with_diagonals()
                    .iter()
                    .map(|&adj_point| points.get(&adj_point).copied().unwrap_or(0))
                    .sum();
                
                points.insert(point, sum);
                sum
            })
    )
}

