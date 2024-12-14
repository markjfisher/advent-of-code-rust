use crate::util::parse::ParseOps;
use crate::util::point::*;
use crate::util::hash::*;
use rayon::prelude::*;

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
    let steps = part2_crt(input);
    // let positions = move_robots(input, 101, 103, steps).collect::<Vec<_>>();
    // print_grid(&positions, 101, 103);
    steps
}

pub fn score_p1(input: &[Robot], width: u32, height: u32, steps: u32) -> u32 {
    let positions = move_robots(input, width, height, steps);
    let counts = quadrant_counts(positions, width, height);
    score_quadrant_counts(&counts)
}

pub fn move_robots<'a>(robots: &'a [Robot], width: u32, height: u32, steps: u32) -> impl Iterator<Item = (u32, u32)> + 'a {
    // precompute steps % width and steps % height
    let steps_x = steps % width;
    let steps_y = steps % height;
    
    robots.iter().map(move |robot| {
        let new_x = (robot.position.x as i64 + steps_x as i64 * robot.velocity.x as i64).rem_euclid(width as i64) as u32;
        let new_y = (robot.position.y as i64 + steps_y as i64 * robot.velocity.y as i64).rem_euclid(height as i64) as u32;
        (new_x, new_y)
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
    for xs in by_row.values().filter(|xs| xs.len() >= min_length as usize) {
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
    (0..7000).into_par_iter()
        .find_map_first(|step| {
            let positions: Vec<_> = move_robots(robots, width, height, step).collect();
            if has_horizontal_line(&positions, min_length) {
                Some((step, positions))
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("No solution found"))
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

fn calculate_variance(positions: &[(u32, u32)]) -> f64 {
    let mean = positions.iter().map(|&(x, _)| x as f64).sum::<f64>() / positions.len() as f64;
    let variance = positions.iter()
        .map(|&(x, _)| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / positions.len() as f64;
    variance
}

fn find_best_offset(robots: &[Robot], modulo: u32, use_x: bool) -> u32 {
    let mut best_variance = f64::MAX;
    let mut best_offset = 0;
    
    for offset in 0..modulo {
        let positions: Vec<_> = move_robots(robots, modulo, modulo, offset).collect();
        let variance = if use_x {
            calculate_variance(&positions)
        } else {
            // For y coordinates, swap x and y in the variance calculation
            calculate_variance(&positions.iter().map(|&(x, y)| (y, x)).collect::<Vec<_>>())
        };
        if variance < best_variance {
            best_variance = variance;
            best_offset = offset;
        }
    }
    best_offset
}

// Using Chinese Remainder Theorem to solve.
// For the x-coordinates, we want to find the offset that minimizes the variance of the x-coordinates.
// Similarly for the y-coordinates.
// We then use the Chinese Remainder Theorem to combine the two solutions.
// Additionally we use the precomputed inverse of WIDTH mod HEIGHT to speed up the CRT.
pub fn part2_crt(input: &[Robot]) -> u32 {
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;
    const INV_W: i64 = 51;  // Precomputed inverse of WIDTH mod HEIGHT
    
    let bx = find_best_offset(input, WIDTH as u32, true) as i64;
    let by = find_best_offset(input, HEIGHT as u32, false) as i64;
    
    let t = bx + INV_W * (by - bx) * WIDTH;
    t.rem_euclid(WIDTH * HEIGHT) as u32
}