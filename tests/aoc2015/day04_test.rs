use aoc2015::day04::*;

#[test]
// If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043
// starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
fn example1() {
    assert_eq!(part1_solve("abcdef", 609000), 609043);
}

#[test]
// If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash
// starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like
// 000006136ef....
fn example2() {
    assert_eq!(part1_solve("pqrstuv", 1048900), 1048970);
} 