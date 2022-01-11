use std::collections::HashSet;
use itertools::Itertools;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    let mut v = str_to_vec(input);
    find_next_password(&mut v, 0)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    let mut v = str_to_vec(input);
    find_next_password(&mut v, 1)
}

fn str_to_vec(s: &str) -> Vec<u8> {
    s.bytes().collect_vec().iter().map(|b| *b - 97).collect_vec()
}

fn find_next_password(current_password: &mut Vec<u8>, skip: u8) -> String {
    let pl = current_password.len();
    let mut valid_password = false;
    let mut skipped: u8 = 0;
    while !valid_password && skipped <= skip {
        increment_password(current_password, pl);
        valid_password = is_valid(current_password);
        if valid_password {
            skipped += 1;
            valid_password = false;
        }
    }

    current_password.iter().map(|b| (*b + 97) as char).join("")
}

fn is_valid(password: &mut Vec<u8>) -> bool {
    let has_sequence_of_3_increasing = password.windows(3).any(|ws| {
        ws[1] == ws[0] + 1 && ws[2] == ws[0] + 2
    });
    let includes_o_i_l = password.iter().any(|b| *b == 8 || *b == 11 || *b == 14);
    let pairs = password.windows(2).filter(|ws| {
        ws[0] == ws[1]
    }).map(|pair| pair.iter().join("")).collect::<HashSet<String>>();

    has_sequence_of_3_increasing && !includes_o_i_l && pairs.len() >= 2
}

fn increment_password(password: &mut Vec<u8>, pl: usize) {
    let mut inc_next = true;

    for i in 1..=pl {
        inc_next = increment_digit(password, pl - i, &mut inc_next);
        if !inc_next { break; }
    }
}

fn increment_digit(ds: &mut Vec<u8>, digit: usize, inc_next: &mut bool) -> bool {
    if *inc_next {
        *inc_next = false;
        ds[digit] = ds[digit] + 1;
        if ds[digit] > 25 {
            ds[digit] = 0;
            *inc_next = true;
        }
    }
    *inc_next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let next_password = part1("abcdefgh");
        assert_eq!(next_password, "abcdffaa".to_string());
    }

    #[test]
    fn can_convert_string_to_vec() {
        assert_eq!(str_to_vec("aaaaaaaa"), vec![0u8, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(str_to_vec("hepxcrrq"), vec![7u8, 4, 15, 23, 2, 17, 17, 16]);
    }

    #[test]
    fn valid_password_tests() {
        assert!(is_valid(&mut str_to_vec("abcdffaa")));
        assert!(is_valid(&mut str_to_vec("ghjaabcc")));
        assert!(!is_valid(&mut str_to_vec("hijklmmn")));
        assert!(!is_valid(&mut str_to_vec("abbceffg")));
        assert!(!is_valid(&mut str_to_vec("abbcegjk")));
    }
}