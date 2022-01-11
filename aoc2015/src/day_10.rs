use itertools::Itertools;

#[aoc(day10, part1)]
pub fn part1(input: &[u8]) -> usize {
    play_look_and_say3(input.to_vec(), 40).len()
}

#[aoc(day10, part2)]
pub fn part2(input: &[u8]) -> usize {
    play_look_and_say3(input.to_vec(), 50).len()
}

// this is naive implementation, takes 2s to do 40 iterations, doesn't return for 50.
#[allow(dead_code)]
fn play_look_and_say(s: &str, n: i32) -> String {
    if n <= 0 { return s.to_string(); }
    let x = split_text(s).iter().fold("".to_string(), |say, v| {
        let count = v.len();
        let c = v.chars().next().unwrap();
        format!("{}{}{}", say, count, c)
    });
    return play_look_and_say(&x, n-1);
}

pub fn split_text(s: &str) -> Vec<String> {
    let mut r = Vec::new();
    for (_, group) in &s.chars().into_iter().group_by(|e| *e) {
        r.push(group.map(|e| e.to_string()).join(""));
    }
    r
}

// another implementation. 30ms, 300ms to do 40, 50 iterations respectively
// compared to 6, 85 for final solution
#[allow(dead_code)]
fn play_look_and_say2(input: &str, iterations: usize) -> usize {
    let mut i: Vec<u8> = input.as_bytes().iter().map(|c| *c).collect();
    for _ in 0..iterations {
        let mut t = Vec::new();
        let mut c = (i[0], 0);
        for j in 0..i.len() {
            if i[j] == c.0 {
                c = (c.0, c.1 + 1)
            } else {
                t.extend_from_slice(c.1.to_string().as_bytes());
                t.push(c.0);
                c = (i[j], 1);
            }
        }
        t.extend_from_slice(c.1.to_string().as_bytes());
        t.push(c.0);

        i.truncate(0);
        i.extend_from_slice(&t);
    }
    return i.len();
}

// this is the fastest solution so far. it does just create big vectors instead of strings
fn play_look_and_say3(in_seq: Vec<u8>, iterations: u8) -> Vec<u8> {
    fn next_sequence(in_seq: Vec<u8>) -> Vec<u8> {
        let mut result = Vec::new();
        let mut current_number = in_seq[0];
        let mut current_runlength = 1;

        for i in &in_seq[1..] {
            if current_number == *i {
                current_runlength += 1;
            } else {
                result.push(current_runlength);
                result.push(current_number);
                current_runlength = 1;
                current_number = *i;
            }
        }
        result.push(current_runlength);
        result.push(current_number);
        result
    }

    let mut seq = in_seq.clone();
    for _ in 0..iterations {
        seq = next_sequence(seq);
    }
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitting_text() {
        assert_eq!(split_text("gHHH5YY++///\\"), vec!["g", "HHH", "5", "YY", "++", "///", "\\"]);
        assert!(split_text("").is_empty());
    }

    #[test]
    fn can_play_look_and_say() {
        assert_eq!(play_look_and_say("1", 1), "11");
        assert_eq!(play_look_and_say("11", 1), "21");
        assert_eq!(play_look_and_say("21", 1), "1211");
        assert_eq!(play_look_and_say("1211", 1), "111221");
        assert_eq!(play_look_and_say("111221", 1), "312211");

        assert_eq!(play_look_and_say("1", 5), "312211");
    }

    #[test]
    fn another_go() {
        let mut seq = vec![1u8];
        seq = play_look_and_say3(seq, 5);
        assert_eq!(seq, vec![3u8, 1, 2, 2, 1, 1])
    }

    #[test]
    fn another_go_full() {
        let mut seq = vec![1u8,1,1,3,1,2,2,1,1,3];
        seq = play_look_and_say3(seq, 10);
        assert_eq!(seq.len(), 130)
    }
}