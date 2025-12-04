use crate::util::{grid::Grid, point::{Point, DIAGONAL}};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u32 {
    find_removable(input).len() as u32
}

pub fn part2(input: &Grid<u8>) -> u32 {
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