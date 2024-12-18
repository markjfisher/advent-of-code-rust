use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use pathfinding::prelude::bfs_reach;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Grid<u16> {
    let mut grid = Grid::new(71, 71, u16::MAX);
    for (i, [x, y]) in input.iter_signed().chunk::<2>().enumerate() {
        grid[Point::new(x, y)] = i as u16;
    }
    grid
}

pub fn part1(input: &Grid<u16>) -> u32 {
    fast_bfs(input, 1024).unwrap()
}

pub fn part2(grid: &Grid<u16>) -> String {
    let non_max_count = grid.points().filter(|&v| grid[v] != u16::MAX).count() as u16;
    let mut low = 1024;
    let mut high = non_max_count;

    while low < high {
        let mid = low + (high - low) / 2;
        match fast_bfs(grid, mid) {
            Some(_) => low = mid + 1,  // Path found, try higher time
            None => high = mid,        // No path, try lower time
        }
    }

    // x marks the spot
    let x = grid.points().find(|&v| grid[v] == low - 1).unwrap();
    format!("{},{}", x.x, x.y)
}

// An attempt to use library bfs solution, to see if it's easier to write.
// This is using the pathfinding crate, but takes 49ms compared to 60us for fast_bfs on the same input
// using bfs_reach dropped it to 38ms, so still very slow. also uses side effect to capture value which is horrid.
pub fn do_bfs(grid: &Grid<u16>, time: u16) -> Option<u32> {
    let start = (ORIGIN, 0u32);
    let end = Point::new(grid.width - 1, grid.height - 1);
    let grid = grid.clone();
    
    let mut min_cost = None;
    let solution = bfs_reach(
        start,
        |&(pos, cost)| {
            // Return no successors if:
            // 1. We've found a solution and current cost >= that solution, or
            // 2. We've explored too far (beyond grid size squared) because there is no solution
            if (min_cost.is_some() && cost >= min_cost.unwrap()) || 
               (cost > (grid.width * grid.height) as u32) {
                return vec![];
            }
            ORTHOGONAL.iter()
                .filter(|&offset| {
                    let next_pos = pos + *offset;
                    grid.contains(next_pos) && time <= grid[next_pos]
                })
                .map(|&offset| {
                    let next_pos = pos + offset;
                    if next_pos == end {
                        min_cost = Some(cost + 1);
                    }
                    (next_pos, cost + 1)
                })
                .collect::<Vec<_>>()
        });

    solution.count(); // eugh. side effect to capture the min_cost - hideous
    min_cost
}

// using hand crafted bfs instead, based on maneatingpie solution
pub fn fast_bfs(grid: &Grid<u16>, time: u16) -> Option<u32> {
    let mut todo = VecDeque::new();
    let mut seen = grid.clone();
    let end = Point::new(grid.width - 1, grid.height - 1);

    todo.push_back((ORIGIN, 0));
    seen[ORIGIN] = 0;
    while let Some((position, cost)) = todo.pop_front() {
        if position == end {
            return Some(cost);
        }
        for next in ORTHOGONAL.map(|o| position + o) {
            if grid.contains(next) && time <= seen[next] {
                todo.push_back((next, cost + 1));
                seen[next] = 0;
            }
        }
    }
    None
}