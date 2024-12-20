use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;
use rayon::prelude::*;

// find both solutions as we parse
pub fn parse(input: &str) -> (u32, u32) {
    let bfs_times = bfs_times(&Grid::parse(input));
    count_improvements(&bfs_times, 20, 100)
}

// find the time it takes to reach each every point from start
// walls will stay as time 0, which means they aren't on the path
pub fn bfs_times(grid: &Grid<u8>) -> Grid<usize> {
    let start = grid.find(b'S').unwrap();
    let mut grid_bfs_time = grid.same_size_with(0_usize);
    let mut queue = VecDeque::new();
    
    grid_bfs_time[start] = 1;
    queue.push_back(start);
    let mut step_time = 0;

    // simple bfs
    while let Some(current) = queue.pop_front() {
        step_time += 1;
        for &dir in ORTHOGONAL.iter() {
            let next = current + dir;
            if grid.contains(next) && grid[next] != b'#' && grid_bfs_time[next] == 0 {
                queue.push_back(next);
                // we need to offset since start time is 1, so we can check 0 value where not yet visited
                grid_bfs_time[next] = step_time + 1;
            }
        }
    }
    
    grid_bfs_time
}

pub fn count_improvements(
    grid_bfs_time: &Grid<usize>,
    max_range: i32,
    min_improvement: usize,
) -> (u32, u32) {
    // get all the path points from the grid so we can iterate over them
    let path_points: Vec<_> = grid_bfs_time.points()
        .filter(|&p| grid_bfs_time[p] > 0)
        .collect();

    let results: Vec<(u32, u32)> = path_points.par_iter()
        .map(|&current| {
            let mut improvements_p1 = 0;
            let mut improvements_p2 = 0;
            
            // look around the current point to see if we connect with somewhere else that's better than our improvement minimum
            for dy in -max_range..=max_range {
                for dx in (-max_range + dy.abs())..=(max_range - dy.abs()) {
                    let cheat_distance = (dx.abs() + dy.abs()) as usize;
                    let next = current + Point::new(dx, dy);
                    // jump to next point if it's not on the path
                    if !grid_bfs_time.contains(next) {
                        continue;
                    }

                    let next_step_time = grid_bfs_time[next];
                    // do we make enough of an improvement if we can get to the next point?
                    // note: saturating_sub is used to avoid having to do "if next_step_time > current_step_time" and returns 0 for that case, as the types are usize we can't simply subtract
                    if next_step_time.saturating_sub(grid_bfs_time[current]) >= cheat_distance + min_improvement {
                        if cheat_distance == 2 {
                            improvements_p1 += 1;
                        }
                        improvements_p2 += 1;
                    }
                }
            }
            
            (improvements_p1, improvements_p2)
        })
        .collect();

    results.iter()
        .fold((0, 0), |acc, &x| (acc.0 + x.0, acc.1 + x.1))
}

pub fn part1(solution: &(u32, u32)) -> u32 {
    solution.0
}

pub fn part2(solution: &(u32, u32)) -> u32 {
    solution.1
}
