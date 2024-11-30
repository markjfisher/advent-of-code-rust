use crate::util::hash::{FastSet, FastSetBuilder};

pub fn parse(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(|s| s.split_whitespace().collect()).collect()
}

pub fn part1(input: &[Vec<&str>]) -> u32 {
    input.iter().filter(|s| is_valid_passphrase(s)).count() as u32
}

pub fn part2(input: &[Vec<&str>]) -> u32 {
    input.iter().filter(|s| is_valid_passphrase_no_anagrams(s)).count() as u32
}

fn is_valid_passphrase(passphrase: &[&str]) -> bool {
    let mut seen = FastSet::with_capacity(1000);
    passphrase.iter().all(|word| seen.insert(word))
}

fn is_valid_passphrase_no_anagrams(passphrase: &[&str]) -> bool {
    let mut seen = FastSet::with_capacity(1000);
    passphrase.iter().all(|word| seen.insert(sorted_word(word)))
}

fn sorted_word(word: &str) -> String {
    let mut chars = word.chars().collect::<Vec<_>>();
    chars.sort_unstable();
    chars.into_iter().collect()
}
