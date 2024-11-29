use aoc::aoc2015::day01::*;

// (()) and ()() both result in floor 0.
#[test]
fn sample1() {
    assert_eq!(part1(&parse("(())")), 0);
    assert_eq!(part1(&parse("()()")), 0);
}

// ((( and (()(()( both result in floor 3.
#[test]
fn sample2() {
    assert_eq!(part1(&parse("(((")), 3);
    assert_eq!(part1(&parse("(()(()(")), 3);
}

// ))((((( also results in floor 3.
#[test]
fn sample3() {
    assert_eq!(part1(&parse("))(((((")), 3);
}

// ()) and ))( both result in floor -1 (the first basement level).
#[test]
fn sample4() {
    assert_eq!(part1(&parse("())")), -1);
    assert_eq!(part1(&parse("))(")), -1);
}

// ))) and )())()) both result in floor -3.
#[test]
fn sample5() {
    assert_eq!(part1(&parse(")))")), -3);
    assert_eq!(part1(&parse(")())())")), -3);
}

// ) causes him to enter the basement at character position 1.
#[test]
fn sample6() {
    assert_eq!(part2(&parse(")")), 1);
}

// ()()) causes him to enter the basement at character position 5.
#[test]
fn sample7() {
    assert_eq!(part2(&parse("()())")), 5);
}


