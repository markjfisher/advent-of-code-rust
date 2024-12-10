use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    find_paths(grid, false, false)
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    find_paths(grid, true, false)
}

pub fn find_paths(grid: &Grid<u8>, distinct: bool, debug: bool) -> u32 {
    let mut seen = grid.same_size_with(Point::new(-1, -1));

    grid.points()
        .filter(|p| grid[*p] == b'0')
        .map(|p| {
            if debug {
                dfs_debug(grid, &mut seen, p, p, distinct)
            } else {
                dfs(grid, &mut seen, p, p, distinct)
            }
        })
        .sum()
}

fn dfs(grid: &Grid<u8>, seen: &mut Grid<Point>, start: Point, point: Point, distinct: bool) -> u32 {
    ORTHOGONAL
        .iter()
        .fold(0, |total, &direction| {
            let next = point + direction;
            if grid.contains(next) && grid[next] == grid[point] + 1 && (distinct || seen[next] != start) {
                seen[next] = start;
                total + if grid[next] == b'9' {
                    1
                } else {
                    dfs(grid, seen, start, next, distinct)
                }
            } else {
                total
            }
        })
}

// Same as above but with some debug showing paths
fn dfs_debug(grid: &Grid<u8>, seen: &mut Grid<Point>, start: Point, point: Point, distinct: bool) -> u32 {
    fn dfs_with_path(grid: &Grid<u8>, seen: &mut Grid<Point>, start: Point, point: Point, distinct: bool, path: &mut Vec<Point>) -> u32 {
        ORTHOGONAL
            .iter()
            .fold(0, |total, &direction| {
                let next = point + direction;
                if grid.contains(next) && grid[next] == grid[point] + 1 && (distinct || seen[next] != start) {
                    // mark this as seen from starting at the particular point, only for part 1
                    if !distinct {
                        seen[next] = start;
                    }
                    path.push(next);

                    let result = if grid[next] == b'9' {
                        println!("Path: {}", path.iter()
                            .map(|p| format!("[{},{}]", p.x, p.y))
                            .collect::<Vec<_>>()
                            .join("->"));
                        1
                    } else {
                        dfs_with_path(grid, seen, start, next, distinct, path)
                    };

                    path.pop();
                    total + result
                } else {
                    total
                }
            })
    }

    let mut path = vec![point];
    dfs_with_path(grid, seen, start, point, distinct, &mut path)
}