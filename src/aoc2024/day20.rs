use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

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
    let mut improvements_p1 = 0;
    let mut improvements_p2 = 0;
    let mut step_time = 0;

    // get all the path points from the grid so we can iterate over them
    let path_points: Vec<_> = grid_bfs_time.points()
        .filter(|&p| grid_bfs_time[p] > 0)
        .collect();

    while step_time < path_points.len() {
        let current = path_points[step_time];
        // I tried checking if we were close enough to the end to ignore the point, but it didn't work. CBA to work out why

        // look around the current point to see if we connect with somewhere else that's better than our improvement minimum
        let current_step_time = grid_bfs_time[current];
        for dy in -max_range..=max_range {
            for dx in (-max_range + dy.abs())..=(max_range - dy.abs()) {
                let connection_distance = (dx.abs() + dy.abs()) as usize;
                let next = current + Point::new(dx, dy);
                
                // check if the next point is on the path
                let next_step_time = if grid_bfs_time.contains(next) { grid_bfs_time[next] } else { 0 };
                // do we make enough of an improvement if we can get to the next point?
                // note: saturating_sub is used to avoid having to do "if next_step_time > current_step_time" and returns 0 for that case, as the types are usize we can't simply subtract
                if next_step_time.saturating_sub(current_step_time) >= connection_distance + min_improvement {
                    if connection_distance == 2 {
                        improvements_p1 += 1;
                    }
                    improvements_p2 += 1;
                }
            }
        }
        
        step_time += 1;
    }

    (improvements_p1, improvements_p2)
}

pub fn part1(solution: &(u32, u32)) -> u32 {
    solution.0
}

pub fn part2(solution: &(u32, u32)) -> u32 {
    solution.1
}
