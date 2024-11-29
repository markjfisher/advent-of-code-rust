use aoc::aoc2015::day05::*;

#[test]
// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
fn filters_3_vowels() {
    assert_eq!(has_at_least_3_vowels("aei"), true);
    assert_eq!(has_at_least_3_vowels("xazegov"), true);
    assert_eq!(has_at_least_3_vowels("aeiouaeiouaeiou"), true);
    assert_eq!(has_at_least_3_vowels("abcdefgh"), false);
}

#[test]
// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
fn filter_duplicate_letter() {
    assert_eq!(contains_duplicate_letters("xx"), true);
    assert_eq!(contains_duplicate_letters("abcdde"), true);
    assert_eq!(contains_duplicate_letters("aabbccdd"), true);
    assert_eq!(contains_duplicate_letters("abcde"), false);
}

#[test]
// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
fn filter_forbidden_sequences() {
    assert_eq!(does_not_contain_forbidden_sequence("ac"), true);
    assert_eq!(does_not_contain_forbidden_sequence("ab"), false);
    assert_eq!(does_not_contain_forbidden_sequence("acd"), false);
    assert_eq!(does_not_contain_forbidden_sequence("apq"), false);
    assert_eq!(does_not_contain_forbidden_sequence("axy"), false);
}

#[test]
fn test_part1() {
    assert_eq!(is_nice_p1("ugknbfddgicrmopn"), true);
    assert_eq!(is_nice_p1("aaa"), true);
    assert_eq!(is_nice_p1("jchzalrnumimnmhp"), false);
    assert_eq!(is_nice_p1("haegwjzuvuyypxyu"), false);
    assert_eq!(is_nice_p1("dvszwmarrgswjxmb"), false);
}

#[test]
fn filters_two_pairs() {
    assert_eq!(contains_two_pairs("xyxy"), true);
    assert_eq!(contains_two_pairs("aabcdefgaa"), true);
    assert_eq!(contains_two_pairs("aaa"), false);
    assert_eq!(contains_two_pairs("xxxaaa"), false);
}

#[test]
fn filters_repeat_char_with_one_between() {
    assert_eq!(exactly_one_char_between("xyx"), true);
    assert_eq!(exactly_one_char_between("abcdefeghi"), true);
    assert_eq!(exactly_one_char_between("aaa"), true);
    assert_eq!(exactly_one_char_between("xyz"), false);
}

#[test]
fn test_part2() {
    assert_eq!(is_nice_p2("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_nice_p2("xxyxx"), true);
    assert_eq!(is_nice_p2("uurcxstgmygtbstg"), false);
    assert_eq!(is_nice_p2("ieodomkazucvgmuy"), false);
} 