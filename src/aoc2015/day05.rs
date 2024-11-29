use itertools::Itertools;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    input.lines().filter(|l| is_nice_p1(l.trim())).count()
}

pub fn is_nice_p1(s: &str) -> bool {
    has_at_least_3_vowels(s) && contains_duplicate_letters(s) && does_not_contain_forbidden_sequence(s)
}

pub fn part2(input: &str) -> usize {
    input.lines().filter(|l| is_nice_p2(l.trim())).count()
}

pub fn is_nice_p2(s: &str) -> bool {
    contains_two_pairs(s) && exactly_one_char_between(s)
}

pub fn has_at_least_3_vowels(s: &str) -> bool {
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

pub fn contains_duplicate_letters(s: &str) -> bool {
    s.chars().tuple_windows().find(|(a, b)| a == b).is_some()
}

pub fn does_not_contain_forbidden_sequence(s: &str) -> bool {
    s.chars().tuple_windows().all(|(a, b)| match (a, b) {
        ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => false,
        _ => true
    })
}

// my first recursive function!
pub fn contains_two_pairs(s: &str) -> bool {
    if s.len() < 4 { return false; }
    let pair = &s[0..2];
    let rest = &s[2..];
    rest.contains(pair) || contains_two_pairs(&s[1..])
}

// my second recursive function!
pub fn exactly_one_char_between(s: &str) -> bool {
    if s.len() < 3 { return false; }
    let c1 = &s[0..1];
    let c3 = &s[2..3];
    c1 == c3 || exactly_one_char_between(&s[1..])
}