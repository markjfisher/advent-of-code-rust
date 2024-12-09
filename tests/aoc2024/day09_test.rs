use aoc::aoc2024::day09::*;

#[test]
fn part1_test() {
    let input = parse("2333133121414131402");
    // let input = parse("12345");
    assert_eq!(part1(&input), 1928);
}

#[test]
fn part2_test() {
    let input = parse("2333133121414131402");
    assert_eq!(part2(&input), 2858);
}

#[test]
fn test_move_block() {
    let mut blocks = Vec::new();
    blocks.push(Block::new(0, 2)); // "00"
    blocks.push(Block::new(-1, 3)); // "..."
    blocks.push(Block::new(1, 2)); // "11"
    blocks.push(Block::new(-1, 1)); // "."
    blocks.push(Block::new(2, 2)); // "22"

    assert_eq!(format_blocks(&blocks), "00...11.22");

    // Try to move the last block (22) into the first gap
    assert!(try_move_block(4, &mut blocks));
    assert_eq!(format_blocks(&blocks), "0022.11...");
    // we don't merge gaps anymore, so this is 6 instead of 5
    assert_eq!(blocks.len(), 6);
}

#[test]
fn test_format_blocks() {
    let mut blocks = Vec::new();
    blocks.push(Block::new(0, 2)); // "00"
    blocks.push(Block::new(-1, 3)); // "..."
    blocks.push(Block::new(1, 3)); // "111"
    blocks.push(Block::new(-1, 2)); // ".."
    blocks.push(Block::new(2, 1)); // "2"

    assert_eq!(format_blocks(&blocks), "00...111..2");
    assert_eq!(blocks.len(), 5);
}

#[test]
fn test_merge_gaps() {
    // Test 1: Adjacent gaps at start
    let mut blocks = Vec::new();
    blocks.push(Block::new(-1, 2)); // ".."
    blocks.push(Block::new(-1, 3)); // "..."
    blocks.push(Block::new(1, 1)); // "1"

    merge_gaps(&mut blocks);
    assert_eq!(format_blocks(&blocks), ".....1");
    assert_eq!(blocks.len(), 2); // Should merge into one gap + one data

    // Test 2: Adjacent gaps in middle
    let mut blocks = Vec::new();
    blocks.push(Block::new(0, 2)); // "00"
    blocks.push(Block::new(-1, 2)); // ".."
    blocks.push(Block::new(-1, 3)); // "..."
    blocks.push(Block::new(1, 1)); // "1"

    merge_gaps(&mut blocks);
    assert_eq!(format_blocks(&blocks), "00.....1");
    assert_eq!(blocks.len(), 3); // Should be: data + merged gap + data

    // Test 3: Multiple gaps to merge
    let mut blocks = Vec::new();
    blocks.push(Block::new(0, 1)); // "0"
    blocks.push(Block::new(-1, 2)); // ".."
    blocks.push(Block::new(-1, 3)); // "..."
    blocks.push(Block::new(-1, 1)); // "."
    blocks.push(Block::new(1, 1)); // "1"

    merge_gaps(&mut blocks);
    assert_eq!(format_blocks(&blocks), "0......1");
    assert_eq!(blocks.len(), 3); // Should merge all gaps into one
}
