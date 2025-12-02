use aoc::aoc2025::day02::*;

const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 1227775554);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 4174379265);
}

#[test]
fn test_parsing_data() {
    assert_eq!(parse("1-31"), (33, 33)); // 11+22 for both
    assert_eq!(parse("95-115"), (99, 210)); // just 99 for p1, 99+111 for p2.
    assert_eq!(parse("8000-20000"), (180790, 191901)); // 8080+8181+..+9999, but we have 1 more in p2 range: 111111
    assert_eq!(parse("100000-222333"), (19822803, 21590478)); // lots of numbers! checking dedupe works
}

// sanity check the solutions in low range. Takes 280ms to run over first 1000 numbers
#[test]
fn test_slow_vs_fast_correctness() {
    // I tried 50000 too, took 10s to run. To keep this usable, lowering to 1000
    for lower in 1..1000 {
        for upper in lower..lower+50 {
            let fast = sum_repeated_in_range(lower, upper);
            let slow = (lower..=upper)
                .filter(|&n| is_repeated_block(n))
                .sum::<u64>();

            assert_eq!(fast, slow, "range {lower}-{upper}");
        }
    }
}

#[test]
fn test_min_max() {
    assert_eq!(10.min(20), 10);
    assert_eq!(20.min(10), 10);
    assert_eq!(10.max(20), 20);
    assert_eq!(20.max(10), 20);

}