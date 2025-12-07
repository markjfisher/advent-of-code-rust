use crate::util::grid::Grid;
use crate::util::point::Point;

pub fn parse(input: &str) -> (u32, u32) {
    let grid = Grid::parse(input);
    let start = grid.points().find(|&p| grid[p] == b'S').unwrap();
    // create a beam from the start point going down
    // for each beam, follow it until it hits a splitter or the edge of the grid
    // if it hits a splitter, split the beam into two new beams, whose positions are either side of the splitter
    // if it hits the edge of the grid, stop the beam
    // if it hits a splitter, add the new beams to the list of beams to follow
    // repeat until there are no more beams to follow
    // count the number of splitters that have been hit by a beam

    // Beam struct to hold position and direction
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct Beam {
        pos: Point,
        dir: Point,
    }

    use std::collections::{HashSet, VecDeque};

    let mut beams = VecDeque::new();
    let mut visited: HashSet<(Point, Point)> = HashSet::new();
    let mut hit_splitters: HashSet<Point> = HashSet::new();

    // Initial beam: start from S, heading down
    let down = Point::new(0, 1);
    beams.push_back(Beam { pos: start + down, dir: down });

    while let Some(mut beam) = beams.pop_front() {
        // dbg!("-------------", beam.pos);
        loop {
            // Move in the current direction
            beam.pos = beam.pos + beam.dir;
            // dbg!(beam.pos);

            // Stop if out of grid bounds
            if !grid.contains(beam.pos) {
                // dbg!("breaking, not in grid");
                break;
            }
            
            // Check this beam+pos only processes once
            if !visited.insert((beam.pos, beam.dir)) {
                // dbg!("breaking, already visited");
                break;
            }

            let ch = grid[beam.pos];
            // dbg!(ch);
            if ch == b'^' {
                // Record splitter hits
                hit_splitters.insert(beam.pos);
                // split into 2 beams either side of the splitter
                let left = Point::new(beam.pos.x - 1, beam.pos.y);
                let right = Point::new(beam.pos.x + 1, beam.pos.y);

                let left_beam = Beam { pos: left, dir: down };
                let right_beam = Beam { pos: right, dir: down };
                // dbg!("new beams:", left_beam, right_beam);
                if !beams.contains(&left_beam) {
                    // dbg!("pushing left");
                    beams.push_back(left_beam);
                }
                if !beams.contains(&right_beam) {
                    // dbg!("pushing right");
                    beams.push_back(right_beam);
                }

                break;
            }
        }
    }

    let part1 = hit_splitters.len() as u32;

    // For part 2, nothing yet, just dummy
    let part2 = 0;

    (part1, part2)
}

pub fn part1(input: &(u32, u32)) -> u32 {
    input.0
}

pub fn part2(input: &(u32, u32)) -> u32 {
    input.1
}
