pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> i32 {
    input.lines().map(|l| diff_decoded(l.trim()) ).sum()
}

pub fn part2(input: &str) -> i32 {
    input.lines().map(|l| diff_encoded(l.trim()) ).sum()
}

pub fn diff_decoded(s: &str) -> i32 {
    let mut size = 0;
    let mut is_backslash = false;
    let mut skip_chars = 0;
    // skip the enclosing quotes entirely
    for c in s[1..s.len() - 1].chars() {
        if skip_chars > 0 { skip_chars -= 1 }
        else if is_backslash {
            if c == 'x' { skip_chars = 2; }
            size += 1;
            is_backslash = false;
        }
        else if c == '\\' { is_backslash = true; }
        else { size += 1; }
    }
    (s.len() - size) as i32
}

pub fn diff_encoded(s: &str) -> i32 {
    // if we see a \\ or " add 2, else add 1
    let mut size = 0;
    for c in s.chars() {
        match c {
            '\\' | '"' => size += 2,
            _ => size += 1,
        }
    }
    // each line has 2 new quotes at start and end
    (size + 2 - s.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_test_input_p1() {
        assert_eq!(part1(create_test_string().as_str()), 12);
    }

    #[test]
    fn can_test_input_p2() {
        assert_eq!(part2(create_test_string().as_str()), 19);
    }

    fn create_test_string() -> String {
        String::from(r#"""
            "abc"
            "aaa\"aaa"
            "\x27""#)
    }
}