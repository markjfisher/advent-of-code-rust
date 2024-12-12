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

pub fn create_regions(grid: &Grid<u8>) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut processed: HashSet<Point> = HashSet::new();
    
    for start in grid.points() {
        if processed.contains(&start) {
            continue;
        }
        
        let plant_type = grid[start];
        let mut region_points = HashSet::new();
        let mut corners = Vec::new();
        let mut to_visit = vec![start];
        
        // First find all points in the region
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
        
        // Then find corners for each point
        for &point in &region_points {
            // A fence exists where we either hit the grid boundary or a different plant
            let fences = [
                !region_points.contains(&(point + UP)),
                !region_points.contains(&(point + RIGHT)),
                !region_points.contains(&(point + DOWN)),
                !region_points.contains(&(point + LEFT))
            ];
            
            // println!("\nAnalyzing point {:?}", point);
            // println!("Fences [UP, RIGHT, DOWN, LEFT]: {:?}", fences);
            
            // Check convex corners (where two fences meet)
            if fences[0] { // top fence
                if fences[3] { corners.push(point); } // top-left
                if fences[1] { corners.push(point); } // top-right
            }
            if fences[2] { // bottom fence
                if fences[3] { corners.push(point); } // bottom-left
                if fences[1] { corners.push(point); } // bottom-right
            }
            
            // Check concave corners (where two non-fences have an outside diagonal)
            if !fences[0] { // no top fence
                if !fences[3] { // no left fence
                    let diagonal = point + UP + LEFT;
                    if !region_points.contains(&diagonal) {
                        corners.push(point);
                    }
                }
                if !fences[1] { // no right fence
                    let diagonal = point + UP + RIGHT;
                    if !region_points.contains(&diagonal) {
                        corners.push(point);
                    }
                }
            }
            if !fences[2] { // no bottom fence
                if !fences[3] { // no left fence
                    let diagonal = point + DOWN + LEFT;
                    if !region_points.contains(&diagonal) {
                        corners.push(point);
                    }
                }
                if !fences[1] { // no right fence
                    let diagonal = point + DOWN + RIGHT;
                    if !region_points.contains(&diagonal) {
                        corners.push(point);
                    }
                }
            }
        }
        
        processed.extend(&region_points);
        
        regions.push(Region {
            points: region_points,
            plant_type,
            side_count: corners.len(),
            corners,
        });
    }
    
    regions
}

#[derive(Debug)]
pub struct Region {
    pub points: HashSet<Point>,
    pub plant_type: u8,
    pub side_count: usize,
    pub corners: Vec<Point>
}

