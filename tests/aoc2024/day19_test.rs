use aoc::aoc2024::day19::*;

const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(EXAMPLE)), 6);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 456);
}

#[test]
fn test_sequence_combinations() {
    let valid_sequences = vec!["r", "wr", "b", "a", "ab", "b"];
    
    let result = find_sequence_combinations("x", &valid_sequences);
    assert_eq!(result.len(), 0);
    
    let result = find_sequence_combinations("r", &valid_sequences);
    assert_eq!(result, vec![vec!["r"]]);
    
    let result = find_sequence_combinations("wr", &valid_sequences);
    assert_eq!(result, vec![vec!["wr"]]);
    
    let result = find_sequence_combinations("rb", &valid_sequences);
    assert_eq!(result, vec![vec!["r", "b"]]);

    let result = find_sequence_combinations("abb", &valid_sequences);
    assert_eq!(result, vec![vec!["a", "b", "b"], vec!["ab", "b"]]);
}