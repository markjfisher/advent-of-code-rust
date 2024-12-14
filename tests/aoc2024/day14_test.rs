use aoc::aoc2024::day14::*;

const EXAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(score_p1(&input, 11, 7, 100), 12);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 456);
}

#[test]
fn move_robots_test() {
    let robots = parse(EXAMPLE);
    let width = 11;
    let height = 7;
    
    // Test initial movement
    let positions: Vec<_> = move_robots(&robots, width, height, 1).collect();
    assert_eq!(positions.len(), robots.len());
    // First robot should move from (0,4) by velocity (3,-3) to (3,1)
    assert!(positions.contains(&(3, 1)));
    
    // Test wrapping
    let positions: Vec<_> = move_robots(&robots, width, height, 3).collect();
    assert_eq!(positions.len(), robots.len());
    // First robot after 3 steps: (0,4) + 3*(3,-3) = (9,-5) wrapped to (9,2)
    assert!(positions.contains(&(9, 2)));
}

#[test]
fn quadrant_counts_test() {
    let robots = parse(EXAMPLE);
    let width = 11;
    let height = 7;
    
    // Test initial positions (t=0)
    let counts = quadrant_counts(robots.iter().map(|r| (r.position.x as u32, r.position.y as u32)), width, height);
    assert_eq!(counts, vec![4, 0, 2, 2]);
    
    // Test after movement
    let counts = quadrant_counts(move_robots(&robots, width, height, 1), width, height);
    assert_eq!(counts, vec![2, 1, 4, 2]);
}

#[test]
fn has_horizontal_line_test() {
    // Test case with a horizontal line of length 5
    let positions = vec![
        (2,0), (3,0), (1,0), (4,0), (5,0),  // continuous line of 5
        (1,1), (3,1), (5,1),                 // scattered points
        (7,2), (8,2), (9,2),                 // short line of 3
    ];
    assert!(has_horizontal_line(&positions, 5));
    
    // Test case with no long enough horizontal line
    let positions = vec![
        (1,0), (2,0), (3,0), (5,0),         // broken line
        (1,1), (2,1), (3,1),                 // short line
        (7,2), (8,2), (9,2),                 // short line
    ];
    assert!(!has_horizontal_line(&positions, 5));
    
    // Test case with multiple gaps
    let positions = vec![
        (5,0), (2,0), (4,0), (1,0), (6,0),  // line with gap
        (1,1), (3,1), (5,1),                 // scattered points
    ];
    assert!(!has_horizontal_line(&positions, 5));
}
