use aoc::aoc2025::day08::*;

const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[test]
fn part1_and_2_test() {
    let coords = parse_coords(EXAMPLE);
    assert_eq!(solve_both(&coords, 10), (40, 25272));
}

fn validate_dsu(dsu: &DSU, expected: &[usize], sizes: &[usize]) {
    assert_eq!(dsu.parent.len(), expected.len());
    for (i, &p) in dsu.parent.iter().enumerate() {
        assert_eq!(p, expected[i]);
    }
    for (i, &s) in dsu.size.iter().enumerate() {
        assert_eq!(s, sizes[i]);
    }
}

#[test]
fn can_build_dsu() {
    // Create a DSU with 5 elements
    let mut dsu = DSU::new(5);

    // NOTE: only root node sizes matter!
    // So in the following tests, only entries from parents that point to themselves have a valid size
    // and although we test the values fully, some of them are just noise

    dsu.union(0, 3);
    dsu.find(0);    // force compression
    dsu.find(3);    // force compression
    validate_dsu(&dsu, &[0, 1, 2, 0, 4], &[2, 1, 1, 1, 1]);
    assert_eq!(dsu.size[0], 2);

    dsu.union(1, 4);
    dsu.find(1);    // force compression
    dsu.find(4);    // force compression
    validate_dsu(&dsu, &[0, 1, 2, 0, 1], &[2, 2, 1, 1, 1]);
    assert_eq!(dsu.size[1], 2);

    // 3 and 4 already point to 0 and 1 respectively, so should compress down to 0 via parents
    dsu.union(3, 4);
    dsu.find(3);    // force compression
    dsu.find(4);    // force compression
    // both sets of 2 are now joined together with 0 as parent
    validate_dsu(&dsu, &[0, 0, 2, 0, 0], &[4, 2, 1, 1, 1]);
    assert_eq!(dsu.size[0], 4);

    dsu.union(2, 4);
    dsu.find(2);    // force compression
    dsu.find(4);    // force compression
    // everything now merged into 0
    validate_dsu(&dsu, &[0, 0, 0, 0, 0], &[5, 2, 1, 1, 1]);
    assert_eq!(dsu.size[0], 5);

}