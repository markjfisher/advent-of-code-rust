use crate::util::{grid::Grid, point::{Point, DIAGONAL}, hash::*};
use once_cell::sync::Lazy;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u32 {
    find_removable(input).len() as u32
}

use std::collections::VecDeque;

pub fn part2(input: &Grid<u8>) -> u32 {
    let mut grid = input.clone();
    let mut neighbour_counts: Grid<u8> = Grid::same_size_with(&grid, 0);
    let mut queue = VecDeque::new();
    let mut removed_count = 0;

    // 1. Initial neighbour counts
    for p in grid.points() {
        if grid[p] == b'@' {
            let mut count = 0;
            for &dir in DIAGONAL.iter() {
                let n = p + dir;
                if grid.contains(n) && grid[n] == b'@' {
                    count += 1;
                }
            }
            neighbour_counts[p] = count;

            if count < 4 {
                queue.push_back(p);
            }
        }
    }

    // println!("queue size: {}", queue.len());
    // println!("initial neighbour_counts:\n{}", neighbour_counts.to_grid_string_with_map(Some(&DIGIT_MAP)));
    // println!("initial grid:\n{}\n", grid.to_grid_string());

    // 2. Process frontier
    while let Some(p) = queue.pop_front() {
        // Might have been removed already as a consequence of a neighbour
        if grid[p] != b'@' {
            continue;
        }

        // Remove this point
        grid[p] = b'.';
        removed_count += 1;

        // Update neighbours
        for &dir in DIAGONAL.iter() {
            let n = p + dir;
            if !grid.contains(n) {
                continue;
            }
            if grid[n] != b'@' {
                continue;
            }

            let cnt = &mut neighbour_counts[n];
            if *cnt > 0 {
                *cnt -= 1;
                // Important: only enqueue when it crosses the 4 -> 3 boundary
                if *cnt == 3 {
                    queue.push_back(n);
                }
            }
        }

        // println!("queue size: {}", queue.len());
        // println!("grid:\n{}\n", grid.to_grid_string());
    
    }

    removed_count
}


pub fn part2_old(input: &Grid<u8>) -> u32 {
    // keep removing until no more to remove
    let mut grid = input.clone();
    let mut removed_count = 0;
    // let mut reductions = 0;

    loop {
        let removable = find_removable(&grid);
        removed_count += removable.len();
        if removable.is_empty() {
            break; // "until no more to remove"
        }
        // reductions += 1;
        remove(&mut grid, &removable);
    }

    // println!("reductions: {}", reductions);

    removed_count as u32
}

fn has_at_least_4_neighbours(grid: &Grid<u8>, p: Point) -> bool {
    let mut count = 0;

    for &direction in DIAGONAL.iter() {
        let x = p + direction;
        if grid.contains(x) && grid[x] == b'@' {
            count += 1;
            if count == 4 {
                return true; // early exit
            }
        }
    }

    false
}

pub fn find_removable(input: &Grid<u8>) -> Vec<Point> {
    input
        .points()
        .filter(|&p| input[p] == b'@')
        .filter(|&p| !has_at_least_4_neighbours(input, p))
        .collect()
}

pub fn remove(input: &mut Grid<u8>, removable: &[Point]) {
    for &p in removable {
        input[p] = b'.';
    }
}

pub static DIGIT_MAP: Lazy<FastMap<u8, &'static str>> = Lazy::new(|| {
    let mut m = FastMap::new();

    for i in 1u8..=8 {
        let s: &'static mut str = Box::leak(i.to_string().into_boxed_str());
        let s: &'static str = s; // coerce &mut str -> &str
        m.insert(i, s);
    }

    m.insert(0, "·");
    m
});