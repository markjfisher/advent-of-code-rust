use crate::util::point::*;
use crate::util::grid::*;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> String {
    let mut pos = Point::new(1, 1);
    let mut code = String::new();
    let digits = Grid::parse("123\n456\n789");

    for line in input {
        for d in line.bytes() {
            let next_pos = pos + Point::from(d);
            if next_pos.x >= 0 && next_pos.x < 3 && next_pos.y >= 0 && next_pos.y < 3 {
                pos = next_pos;
            }
        }
        code.push(digits[pos] as char);
    }
    code
}

pub fn part2(input: &[&str]) -> String {
    // Mark the empty positions with a # so we can't move into them
    let pad = Grid::parse("##1##\n#234#\n56789\n#ABC#\n##D##");
    let mut pos = Point::new(0, 2);
    let mut code = String::new();

    for line in input {
        for d in line.bytes() {
            let next_pos = pos + Point::from(d);
            // Check bounds FIRST! and that the next position is not a wall.
            if next_pos.x >= 0 && next_pos.x < 5 && next_pos.y >= 0 && next_pos.y < 5 && pad[next_pos] != b'#' {
                pos = next_pos;
            }
        }
        code.push(pad[pos] as char);
    }
    code
}
