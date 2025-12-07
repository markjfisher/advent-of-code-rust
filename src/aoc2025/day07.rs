use crate::util::grid::Grid;
use crate::util::point::Point;
use std::collections::HashSet;

pub fn parse(input: &str) -> (u64, u64) {
    let grid = Grid::parse(input);
    let start = grid.points().find(|&p| grid[p] == b'S').unwrap();

    let width = grid.width as usize;
    let height = grid.height as usize;

    // It's a DP problem, so let's use a DP table
    // dp[row][x] = beams just ABOVE row 'row' at column 'x'
    let mut dp = vec![vec![0u64; width]; height + 1];

    let sx = start.x as usize;
    let sy = start.y as usize;

    // The initial beam!
    dp[sy + 1][sx] = 1;

    let mut hit_splitters: HashSet<Point> = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            let cnt = dp[y][x];
            if cnt == 0 {
                continue;
            }

            let p = Point::new(x as i32, y as i32);
            let ch = grid[p];

            if ch == b'^' {
                // Part 1: record splitter hit once
                hit_splitters.insert(p);

                // Split beams: down-left and down-right
                // The shape of the puzzle means we don't need to check for out-of-bounds on left/right
                dp[y + 1][x - 1] += cnt;
                dp[y + 1][x + 1] += cnt;
            } else {
                // Any non-splitter: beams go straight down
                dp[y + 1][x] += cnt;
            }
        }
    }

    let part1 = hit_splitters.len() as u64;

    let total_paths: u64 = dp[height].iter().sum();
    let part2 = total_paths as u64;

    (part1, part2)
}

pub fn part1(input: &(u64, u64)) -> u64 {
    input.0
}

pub fn part2(input: &(u64, u64)) -> u64 {
    input.1
}
