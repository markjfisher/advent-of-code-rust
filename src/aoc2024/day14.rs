use crate::util::parse::ParseOps;
use crate::util::point::*;
use crate::util::hash::*;

pub struct Robot {
    pub position: Point,
    pub velocity: Point,
}

pub fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(|line| {
        let vs = line.iter_signed().collect::<Vec<_>>();
        Robot {
            position: Point::new(vs[0], vs[1]),
            velocity: Point::new(vs[2], vs[3]),
        }
    }).collect()
}

pub fn part1(input: &[Robot]) -> u32 {
    score_p1(input, 101, 103, 100)
}

pub fn part2(input: &[Robot]) -> u32 {
    let (step, _positions) = find_tree_step(input, 101, 103, 10);
    // for LOLs, print the grid at the solution step
    // println!("\nGrid at step {}:", step);
    // print_grid(&positions, 101, 103);
    step
}

pub fn score_p1(input: &[Robot], width: u32, height: u32, steps: u32) -> u32 {
    let positions = move_robots(input, width, height, steps);
    let counts = quadrant_counts(positions, width, height);
    score_quadrant_counts(&counts)
}

pub fn move_robots<'a>(robots: &'a [Robot], width: u32, height: u32, steps: u32) -> impl Iterator<Item = (u32, u32)> + 'a {
    robots.iter().map(move |robot| {
        let width = width as i32;
        let steps = steps as i32;
        let velocity_x = robot.velocity.x as i32;
        let position_x = robot.position.x as i32;
        
        // Calculate (position + steps * velocity) % width
        // = (position % width + (steps * velocity) % width) % width
        // but position is already bound, so can skip its modulo
        let new_x = (
            position_x + 
            ((steps * velocity_x).rem_euclid(width))
        ).rem_euclid(width);

        let height = height as i32;
        let velocity_y = robot.velocity.y as i32;
        let position_y = robot.position.y as i32;
        
        let new_y = (
            position_y + 
            ((steps * velocity_y).rem_euclid(height))
        ).rem_euclid(height);

        (new_x as u32, new_y as u32)
    })
}

pub fn quadrant_counts<'a>(positions: impl Iterator<Item = (u32, u32)>, width: u32, height: u32) -> Vec<u32> {
    let mid_x = width / 2;
    let mid_y = height / 2;
    
    // Initialize counts for [upper_left, upper_right, lower_left, lower_right]
    let mut counts = vec![0; 4];

    // determine which quadrant each position is in
    for (x, y) in positions {
        let quadrant = match (x.cmp(&mid_x), y.cmp(&mid_y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 0,     // upper left
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 1,  // upper right
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 2,  // lower left
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 3, // lower right
            _ => continue, // skips points exactly on the middle lines!
        };
        counts[quadrant] += 1;
    }
    
    counts
}

pub fn score_quadrant_counts(counts: &[u32]) -> u32 {
    counts.iter().map(|&x| x as u32).product()
}

pub fn has_horizontal_line(positions: &[(u32, u32)], min_length: u32) -> bool {
    // Group positions by y coordinate
    let mut by_row: FastMap<u32, Vec<u32>> = FastMap::new();
    for &(x, y) in positions {
        by_row.entry(y).or_default().push(x);
    }
    
    // for each row, look for continuous sequence. given the random distribution, the solution will
    // be found in the first row that has a sequence of length >= min_length
    for xs in by_row.values() {
        let mut xs = xs.clone();
        xs.sort();
        
        let mut current_len = 1;
        let mut prev = xs[0];
        
        for &x in &xs[1..] {
            if x == prev + 1 {
                current_len += 1;
                if current_len >= min_length {
                    return true;
                }
            } else {
                current_len = 1;
            }
            prev = x;
        }
    }
    
    false
}

pub fn find_tree_step(robots: &[Robot], width: u32, height: u32, min_length: u32) -> (u32, Vec<(u32, u32)>) {
    for step in 0.. {
        let positions: Vec<_> = move_robots(robots, width, height, step).collect();
        if has_horizontal_line(&positions, min_length) {
            return (step, positions);
        }
    }
    panic!("No solution found")
}

pub fn print_grid(positions: &[(u32, u32)], width: u32, height: u32) {
    for y in 0..height {
        for x in 0..width {
            if positions.contains(&(x, y)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
}