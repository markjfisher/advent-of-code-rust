use aoc::aoc2024::day12::*;
use aoc::util::point::*;
use aoc::util::grid::*;

const EXAMPLE1: &str = "\
AAAA
BBCD
BBCC
EEEC";

const EXAMPLE2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const EXAMPLE3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

const EXAMPLE4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

const EXAMPLE5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE1)), 140);
    assert_eq!(part1(&parse(EXAMPLE2)), 772);
    assert_eq!(part1(&parse(EXAMPLE3)), 1930);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(EXAMPLE1)), 80);
    assert_eq!(part2(&parse(EXAMPLE2)), 436);
    assert_eq!(part2(&parse(EXAMPLE4)), 236);
    assert_eq!(part2(&parse(EXAMPLE5)), 368);
}

#[test]
fn test_find_borders() {
    let grid = Grid::parse(EXAMPLE1);
    let borders = find_borders(&grid);

    // Helper function to check if a border exists
    let has_border = |p1: Point, p2: Point| {
        borders.contains(&(p1, p2)) || borders.contains(&(p2, p1))
    };


    assert_eq!(borders.len(), 28);

    // edge borders
    assert!(has_border(Point::new(0, 0), Point::new(-1, 0)));
    assert!(has_border(Point::new(0, 0), Point::new(0, -1)));
    assert!(has_border(Point::new(1, 0), Point::new(1, -1)));
    assert!(has_border(Point::new(2, 0), Point::new(2, -1)));
    assert!(has_border(Point::new(3, 0), Point::new(3, -1)));
    assert!(has_border(Point::new(3, 0), Point::new(4, 0)));
    assert!(has_border(Point::new(0, 1), Point::new(-1, 1)));
    assert!(has_border(Point::new(3, 1), Point::new(4, 1)));
    assert!(has_border(Point::new(0, 2), Point::new(-1, 2)));
    assert!(has_border(Point::new(3, 2), Point::new(4, 2)));
    assert!(has_border(Point::new(0, 3), Point::new(-1, 3)));
    assert!(has_border(Point::new(3, 3), Point::new(4, 3)));
    assert!(has_border(Point::new(0, 3), Point::new(0, 4)));
    assert!(has_border(Point::new(1, 3), Point::new(1, 4)));
    assert!(has_border(Point::new(2, 3), Point::new(2, 4)));
    assert!(has_border(Point::new(3, 3), Point::new(3, 4)));

    // inner borders
    assert!(has_border(Point::new(0, 0), Point::new(0, 1)));
    assert!(has_border(Point::new(1, 0), Point::new(1, 1)));
    assert!(has_border(Point::new(2, 0), Point::new(2, 1)));
    assert!(has_border(Point::new(3, 0), Point::new(3, 1)));
    assert!(has_border(Point::new(1, 1), Point::new(2, 1)));
    assert!(has_border(Point::new(2, 1), Point::new(3, 1)));
    assert!(has_border(Point::new(3, 1), Point::new(3, 2)));
    assert!(has_border(Point::new(1, 2), Point::new(2, 2)));
    assert!(has_border(Point::new(0, 2), Point::new(0, 3)));
    assert!(has_border(Point::new(1, 2), Point::new(1, 3)));
    assert!(has_border(Point::new(2, 2), Point::new(2, 3)));
    assert!(has_border(Point::new(2, 3), Point::new(3, 3)));
}

#[test]
fn test_count_connected_regions() {
    let grid = parse(EXAMPLE1);
    let regions = count_connected_regions(&grid);
    
    // Test A region (size 4)
    assert_eq!(regions.get(&Point::new(0, 0)), Some(&4));
    assert_eq!(regions.get(&Point::new(3, 0)), Some(&4));
    
    // Test B region (size 4)
    assert_eq!(regions.get(&Point::new(0, 1)), Some(&4));
    assert_eq!(regions.get(&Point::new(1, 2)), Some(&4));
    
    // Test C region (size 4)
    assert_eq!(regions.get(&Point::new(2, 1)), Some(&4));
    assert_eq!(regions.get(&Point::new(3, 2)), Some(&4));
    
    // Test D region (size 1)
    assert_eq!(regions.get(&Point::new(3, 1)), Some(&1));
    
    // Test E region (size 3)
    assert_eq!(regions.get(&Point::new(0, 3)), Some(&3));
    
    // Verify all points are mapped
    assert_eq!(regions.len(), (grid.width * grid.height) as usize);
}

#[test]
fn test_score_borders() {
    let grid = parse(EXAMPLE1);
    let score = score_borders(&grid);
    
    // Top edge: A(4) four times
    // Right edge: A(4), D(1), C(3), C(3)
    // Bottom edge: E(3), E(3), E(3), C(3)
    // Left edge: A(4), B(4), B(4), E(3)
    // Internal borders:
    // A-B: 4+4 four times
    // B-C: 4+3 twice
    // C-D: 3+1 once
    // B-C: 4+3 once
    // C-E: 3+3 once
    
    assert_eq!(score, 140);
}

#[test]
fn test_count_region_sides() {
    let grid = parse(EXAMPLE1);
    let regions = create_regions(&grid);
    
    let a_region = regions.iter()
        .find(|r| r.plant_type == b'A')
        .unwrap();
    
    assert_eq!(a_region.points.len(), 4);
    assert_eq!(a_region.side_count, 4);
    
    let b_region = regions.iter()
        .find(|r| r.plant_type == b'B')
        .unwrap();
    
    assert_eq!(b_region.points.len(), 4);
    assert_eq!(b_region.side_count, 4);
    
    let c_region = regions.iter()
        .find(|r| r.plant_type == b'C')
        .unwrap();
    
    // C region is a snake shape:
    //   C
    //   CC
    //    C
    assert_eq!(c_region.points.len(), 4);
    assert_eq!(c_region.side_count, 8); // Has 8 sides due to its shape
    
    let d_region = regions.iter()
        .find(|r| r.plant_type == b'D')
        .unwrap();
    
    assert_eq!(d_region.points.len(), 1);
    assert_eq!(d_region.side_count, 4);
    
    let e_region = regions.iter()
        .find(|r| r.plant_type == b'E')
        .unwrap();
    
    assert_eq!(e_region.points.len(), 3);
    assert_eq!(e_region.side_count, 4);
    
    assert_eq!(regions.len(), 5); // A, B, C, D, E regions
}

#[test]
fn test_score_regions() {
    let grid = parse(EXAMPLE1);
    let score = score_regions(&grid);
    
    // A: 4 points * 4 sides = 16
    // B: 4 points * 4 sides = 16
    // C: 4 points * 8 sides = 32
    // D: 1 point  * 4 sides = 4
    // E: 3 points * 4 sides = 12
    // Total = 16 + 16 + 32 + 4 + 12 = 80
    
    assert_eq!(score, 80);
}

