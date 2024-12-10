use aoc::aoc2024::day10::*;

const TEST0: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";


const TEST1: &str = "\
0123
7654
89..";

const TEST2: &str = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

const TEST3: &str = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

const TEST4: &str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

const TEST5: &str = "\
012345
123456
234567
345678
4.6789
56789.";

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = parse(TEST0);
        assert_eq!(part1(&input), 36);
    }

    #[test]
    fn simple_test() {
        assert_eq!(find_paths(&parse(TEST1), false, true), 1);
    }

    #[test]
    fn simple_test2() {
        assert_eq!(find_paths(&parse(TEST2), false, true), 2);
    }
}


#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn part2_test() {
        let input = parse(TEST0);
        assert_eq!(part2(&input), 81);
    }
    #[test]
    fn simple_test() {
        assert_eq!(find_paths(&parse(TEST1), true, true), 1);
    }

    #[test]
    fn simple_test2() {
        assert_eq!(find_paths(&parse(TEST2), true, true), 2);
    }

    #[test]
    fn simple_test3() {
        assert_eq!(find_paths(&parse(TEST3), true, true), 3);
    }

    #[test]
    fn simple_test4() {
        assert_eq!(find_paths(&parse(TEST4), true, false), 13);
    }

    #[test]
    fn simple_test5() {
        assert_eq!(find_paths(&parse(TEST5), true, false), 227);
    }
}
