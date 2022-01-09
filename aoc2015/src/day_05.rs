use itertools::Itertools;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().filter(|l| is_nice_p1(l.trim())).count()
}

fn is_nice_p1(s: &str) -> bool {
    has_at_least_3_vowels(s) && contains_duplicate_letters(s) && does_not_contain_forbidden_sequence(s)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|l| is_nice_p2(l.trim())).count()
}

fn is_nice_p2(s: &str) -> bool {
    contains_two_pairs(s) && exactly_one_char_between(s)
}

fn has_at_least_3_vowels(s: &str) -> bool {
    // functional version, but filters all letters before exiting.
    // let m1 = s.chars().filter(|c| match c {
    //     'a' | 'e' | 'i' | 'o' | 'u' => true,
    //     _ => false
    // }).count() >= 3;

    // this version exits early compared to filter, so is faster...?
    let mut vowel_count: u32 = 0;
    let mut s_iter = s.chars().into_iter();
    while let Some(c) = s_iter.next() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => ()
        };
        if vowel_count == 3 { break; }
    }
    vowel_count == 3
}

fn contains_duplicate_letters(s: &str) -> bool {
    s.chars().tuple_windows().find(|(a, b)| a == b).is_some()
}

fn does_not_contain_forbidden_sequence(s: &str) -> bool {
    s.chars().tuple_windows().all(|(a, b)| match (a, b) {
        ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => false,
        _ => true
    })
}

// my first recursive function!
fn contains_two_pairs(s: &str) -> bool {
    if s.len() < 4 { return false; }
    let pair = &s[0..2];
    let rest = &s[2..];
    rest.contains(pair) || contains_two_pairs(&s[1..])
}

// my second recursive function!
fn exactly_one_char_between(s: &str) -> bool {
    if s.len() < 3 { return false; }
    let c1 = &s[0..1];
    let c3 = &s[2..3];
    c1 == c3 || exactly_one_char_between(&s[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

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

}