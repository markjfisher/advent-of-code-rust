use crate::util::point::Point;

// This day was extremely difficult.
// I initially implemented modelling different types of Robot and their movements, calculating the strings for each next key press.
// However it became difficult to work out the minimal solution between iterations.
// I also suspected it wouldn't scale for part 2, as it was escalating on the strings being used. I was right, Eric hit us with a huge depth of 25.

// This solution came from realising that there are only 12 different patterns to moving around,
// and that v<<A is just v<A with an extra keypress, because we are optimising the order of the keypresses so the next robot moves minimally.
// So we could count and recurse, and use memoization to capture previous calculations.

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> i64 {
    input.iter()
        .map(|line| {
            let multiplier = line[..line.len()-1].parse::<i64>().unwrap();
            multiplier * type_code(line, 2)
        })
        .sum()
}

pub fn part2(input: &[&str]) -> i64 {
    input.iter()
        .map(|line| {
            let multiplier = line[..line.len()-1].parse::<i64>().unwrap();
            multiplier * type_code(line, 25)
        })
        .sum()
}

// 2 directions per diagonal + each cardinal direction gives 12 different patterns
// we don't need to double count here on key presses, that will happen later
#[derive(Debug, Clone, Copy)]
pub enum ButtonPattern {
    LUA, // Left-Up-A
    ULA, // Up-Left-A
    LA,  // Left-A
    LDA, // Left-Down-A
    DLA, // Down-Left-A
    UA,  // Up-A
    DA,  // Down-A
    URA, // Up-Right-A
    RUA, // Right-Up-A
    RA,  // Right-A
    DRA, // Down-Right-A
    RDA, // Right-Down-A
}

// This is a recursive function that counts the number of button presses for a given pattern at a given depth
// It works on a base pattern (e.g. <^A (LUA)) and works out what the next robot would do to achieve that pattern.
// An example is:
//                  <    to    ^    to    A
//   requires key presses:
//   <^A (LUA) -> v<<A (DLA), >^A (RUA), >A (RA)
pub fn count_pattern_presses(pattern: ButtonPattern, depth: u32, pattern_cache: &mut Vec<Vec<i64>>) -> i64 {
    use ButtonPattern::*;

    if depth == 0 {
        // base case, 2 presses for each cardinal direction, 3 for diagonals
        return match pattern {
            LA | UA | DA | RA => 2,
            _ => 3,
        };
    }

    // memoization - the key to performance!!
    // is this pattern cached?
    if pattern_cache[pattern as usize][depth as usize] != 0 {
        return pattern_cache[pattern as usize][depth as usize];
    }

    // break down each pattern to the next version down the chain
    let result = match pattern {
        LUA => { // becomes v<<A >^A >A 
            count_pattern_presses(DLA, depth - 1, pattern_cache) +
            count_pattern_presses(RUA, depth - 1, pattern_cache) +
            count_pattern_presses(RA, depth - 1, pattern_cache) + 1 // +1 is for repeated '<'
        }
        ULA => { // becomes <A v<A >>^A
            count_pattern_presses(LA, depth - 1, pattern_cache) +
            count_pattern_presses(DLA, depth - 1, pattern_cache) +
            count_pattern_presses(RUA, depth - 1, pattern_cache) + 1 // +1 is for repeated '>'
        }
        LA => { // becomes v<<A >>^A
            count_pattern_presses(DLA, depth - 1, pattern_cache) +
            count_pattern_presses(RUA, depth - 1, pattern_cache) + 2 // +2 is for repeated '<' and '>'
        }
        LDA => { // becomes v<<A >A ^>A
            count_pattern_presses(DLA, depth - 1, pattern_cache) +
            count_pattern_presses(RA, depth - 1, pattern_cache) +
            count_pattern_presses(URA, depth - 1, pattern_cache) + 1 // +1 is for repeated '<'
        }
        DLA => { // becomes <vA <A >>^A
            count_pattern_presses(LDA, depth - 1, pattern_cache) +
            count_pattern_presses(LA, depth - 1, pattern_cache) +
            count_pattern_presses(RUA, depth - 1, pattern_cache) + 1 // +1 is for repeated '>'
        }
        UA => { // becomes <A >A
            count_pattern_presses(LA, depth - 1, pattern_cache) +
            count_pattern_presses(RA, depth - 1, pattern_cache)
        }
        DA => { // becomes <vA ^>A
            count_pattern_presses(LDA, depth - 1, pattern_cache) +
            count_pattern_presses(URA, depth - 1, pattern_cache)
        }
        URA => { // becomes <A v>A ^A
            count_pattern_presses(LA, depth - 1, pattern_cache) +
            count_pattern_presses(DRA, depth - 1, pattern_cache) +
            count_pattern_presses(UA, depth - 1, pattern_cache)
        }
        RUA => { // becomes vA <^A >A
            count_pattern_presses(DA, depth - 1, pattern_cache) +
            count_pattern_presses(LUA, depth - 1, pattern_cache) +
            count_pattern_presses(RA, depth - 1, pattern_cache)
        }
        RA => { // becomes vA ^A
            count_pattern_presses(DA, depth - 1, pattern_cache) +
            count_pattern_presses(UA, depth - 1, pattern_cache)
        }
        DRA => { // becomes <vA >A ^A
            count_pattern_presses(LDA, depth - 1, pattern_cache) +
            count_pattern_presses(RA, depth - 1, pattern_cache) +
            count_pattern_presses(UA, depth - 1, pattern_cache)
        }
        RDA => { // becomes vA <A ^>A
            count_pattern_presses(DA, depth - 1, pattern_cache) +
            count_pattern_presses(LA, depth - 1, pattern_cache) +
            count_pattern_presses(URA, depth - 1, pattern_cache)
        }
    };

    pattern_cache[pattern as usize][depth as usize] = result;
    result
}

fn type_code(code: &str, depth: u32) -> i64 {
    use ButtonPattern::*;

    let mut pos = Point::new(2, 3); // Start at 'A'
    let mut count = 0;
    let mut pattern_cache = vec![vec![0; 26]; 12]; // 12 patterns x 26 depths

    for c in code.chars() {
        let target = match c {
            'A' => Point::new(2, 3),
            '0' => Point::new(1, 3),
            '1'..='9' => {
                let v = c as i32 - '1' as i32;
                Point::new(v % 3, 2 - (v / 3))
            }
            _ => continue,
        };

        if target == pos {
            count += 1;
            continue;
        }

        // This is the logic for moving between two points ensuring we don't cut the blank spaces, and choosing an optimal path to reduce presses.
        // Nice use of match to setup the 3 conditions; is it vertical/horizontal/right
        let pattern = match (target.x == pos.x, target.y == pos.y, target.x > pos.x) {
            // Vertically aligned
            (true, false, _) => {
                if target.y > pos.y { DA } else { UA }
            },
            // Horizontally aligned
            (false, true, right) => {
                if right { RA } else { LA }
            },
            // Moving right, diagonally - avoid the blank space
            (false, false, true) => {
                if target.y > pos.y {
                    // Can't start vertically if it would move us into (3, 0)
                    if target.y == 3 && pos.x == 0 { RDA } else { DRA }
                } else {
                    URA
                }
            },
            // Moving left, diagonally
            (false, false, false) => {
                if target.y > pos.y {
                    LDA
                } else {
                    // If we would move into (3, 0) going horizontally, start by going up first
                    if target.x == 0 && pos.y == 3 { ULA } else { LUA }
                }
            },
            _ => unreachable!(),
        };

        // Add extra presses for moves beyond the first in each direction
        let extra = std::cmp::max(0, (target.y - pos.y).abs() - 1) + 
                   std::cmp::max(0, (target.x - pos.x).abs() - 1);

        count += count_pattern_presses(pattern, depth, &mut pattern_cache) + extra as i64;
        pos = target;
    }

    count
}

