use aoc::aoc2015::day03::*;

#[test]
// > delivers presents to 2 houses: one at the starting location, and one to the east.
fn example1() {
    assert_eq!(part1(&parse(">")), 2);
}

#[test]
// ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
fn example2() {
    assert_eq!(part1(&parse("^>v<")), 4);
}

#[test]
// ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
fn example3() {
    assert_eq!(part1(&parse("^v^v^v^v^v")), 2);
}

#[test]
// ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
fn example4() {
    assert_eq!(part2(&parse("^v")), 3);
}

#[test]
// ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
fn example5() {
    assert_eq!(part2(&parse("^>v<")), 3);
}

#[test]
// ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
fn example6() {
    assert_eq!(part2(&parse("^v^v^v^v^v")), 11);
}