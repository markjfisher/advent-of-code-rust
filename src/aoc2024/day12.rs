use crate::util::point::*;
use crate::util::grid::Grid;
use std::collections::{HashMap, HashSet};

pub type BorderPair = (Point, Point);

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u32 {
    score_borders(input)
}

pub fn part2(input: &Grid<u8>) -> u32 {
    score_regions(input)
}

pub fn score_borders(grid: &Grid<u8>) -> u32 {
    // the price is found by multiplying that region's area by its perimeter.
    // however, instead if we find all the borders for each point, we can just add the region area.
    // borders are every connected point in the grid, plus those outside of it by 1 step.
    // it made sense at 7am :D
    let borders = find_borders(grid);
    let region_sizes = count_connected_regions(grid);

    borders.iter()
        .map(|(p1, p2)| {
            if grid.contains(*p1) && grid.contains(*p2) {
                (region_sizes[p1] + region_sizes[p2]) as u32
            } else {
                let inside_point = if grid.contains(*p1) { p1 } else { p2 };
                region_sizes[inside_point] as u32
            }
        })
        .sum()
}

pub fn score_regions(grid: &Grid<u8>) -> u32 {
    create_regions(grid).iter()
        .map(|region| (region.points.len() * region.side_count) as u32)
        .sum()
}

pub fn find_borders(grid: &Grid<u8>) -> Vec<BorderPair> {
    let mut borders = Vec::new();
    for point in grid.points() {
        let current = grid[point];
        for dir in ORTHOGONAL {
            let neighbour = point + dir;
            if !grid.contains(neighbour) {
                borders.push((point, neighbour));
            }
            else if grid[neighbour] != current {
                // only add each border once by enforcing an ordering
                if point < neighbour {
                    borders.push((point, neighbour));
                }
            }
        }
    }    
    borders
}

// part 1 function to find the size of each region each point is in
// uses a flood fill to find the size of each region
pub fn count_connected_regions(grid: &Grid<u8>) -> HashMap<Point, usize> {
    let mut region_sizes: HashMap<Point, usize> = HashMap::new();
    
    // do flood fill from each point that hasn't been counted yet
    for start in grid.points() {
        if region_sizes.contains_key(&start) {
            continue;
        }
        
        let plant_type = grid[start];
        let mut visited = HashSet::new();
        let mut to_visit = vec![start];
        
        while let Some(point) = to_visit.pop() {
            if !visited.insert(point) {
                continue;
            }
            
            for dir in ORTHOGONAL {
                let neighbour = point + dir;
                if grid.contains(neighbour) 
                    && grid[neighbour] == plant_type 
                    && !visited.contains(&neighbour) 
                {
                    to_visit.push(neighbour);
                }
            }
        }
        
        // store the region size for all points in this region
        let region_size = visited.len();
        for point in visited {
            region_sizes.insert(point, region_size);
        }
    }
    
    region_sizes
}

// I tried refactoring this to use the flood fill from part 1, but it was slower for some reason :shrug:
pub fn create_regions(grid: &Grid<u8>) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut processed = HashSet::new();
    
    for start in grid.points() {
        if processed.contains(&start) {
            continue;
        }
        
        let plant_type = grid[start];
        let mut region_points = HashSet::new();
        let mut to_visit = vec![start];
        
        while let Some(point) = to_visit.pop() {
            if !region_points.insert(point) {
                continue;
            }
            
            for dir in ORTHOGONAL {
                let neighbour = point + dir;
                if grid.contains(neighbour) 
                    && grid[neighbour] == plant_type 
                    && !region_points.contains(&neighbour) 
                {
                    to_visit.push(neighbour);
                }
            }
        }
        
        // Find sides - this is messy but it works and I'm done with the day
        // It calculates sides by creating ranges by checking points either side
        let mut sides = HashSet::new();
        
        // For each point in the region
        for &point in &region_points {
            // Check vertical sides (left and right)
            for &dir in &[LEFT, RIGHT] {
                let neighbour = point + dir;
                if !grid.contains(neighbour) || grid[neighbour] != plant_type {
                    // Look for continuous vertical line
                    let mut y_min = point.y;
                    let mut y_max = point.y;
                    
                    // Check above for same vertical line
                    let mut check = point + UP;
                    while region_points.contains(&check) {
                        let side_check = check + dir;
                        if !grid.contains(side_check) || grid[side_check] != plant_type {
                            y_min = check.y;
                            check = check + UP;
                        } else {
                            break;
                        }
                    }
                    
                    // Check below for same vertical line
                    check = point + DOWN;
                    while region_points.contains(&check) {
                        let side_check = check + dir;
                        if !grid.contains(side_check) || grid[side_check] != plant_type {
                            y_max = check.y;
                            check = check + DOWN;
                        } else {
                            break;
                        }
                    }
                    
                    sides.insert(Side::Vertical {
                        x: point.x,
                        y_range: (y_min, y_max),
                        direction: dir.x
                    });
                }
            }
            
            // Check horizontal sides (up and down)
            for &dir in &[UP, DOWN] {
                let neighbour = point + dir;
                if !grid.contains(neighbour) || grid[neighbour] != plant_type {
                    // Look for continuous horizontal line
                    let mut x_min = point.x;
                    let mut x_max = point.x;
                    
                    // Check left for same horizontal line
                    let mut check = point + LEFT;
                    while region_points.contains(&check) {
                        let side_check = check + dir;
                        if !grid.contains(side_check) || grid[side_check] != plant_type {
                            x_min = check.x;
                            check = check + LEFT;
                        } else {
                            break;
                        }
                    }
                    
                    // Check right for same horizontal line
                    check = point + RIGHT;
                    while region_points.contains(&check) {
                        let side_check = check + dir;
                        if !grid.contains(side_check) || grid[side_check] != plant_type {
                            x_max = check.x;
                            check = check + RIGHT;
                        } else {
                            break;
                        }
                    }
                    
                    sides.insert(Side::Horizontal {
                        y: point.y,
                        x_range: (x_min, x_max),
                        direction: dir.y
                    });
                }
            }
        }
        
        regions.push(Region {
            points: region_points.clone(),
            plant_type,
            side_count: sides.len(),
        });
        
        processed.extend(region_points);
    }
    
    regions
}

#[derive(Debug)]
pub struct Region {
    pub points: HashSet<Point>,
    pub plant_type: u8,
    pub side_count: usize
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Side {
    Vertical { x: i32, y_range: (i32, i32), direction: i32 },
    Horizontal { y: i32, x_range: (i32, i32), direction: i32 },
}
