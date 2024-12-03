pub fn parse(input: &str) -> Vec<(u32, u32, bool)> {
    extract_muls(input)
}

pub fn part1(input: &[(u32, u32, bool)]) -> u32 {
    input.iter().map(|(x, y, _)| x * y).sum()
}

pub fn part2(input: &[(u32, u32, bool)]) -> u32 {
    input.iter().filter(|(_, _, on)| *on).map(|(x, y, _)| x * y).sum()
}

pub fn extract_muls(s: &str) -> Vec<(u32, u32, bool)> {
    let mul_regex = regex::Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = Vec::new();
    let mut is_on = true;
    let mut pos = 0;

    while pos < s.len() {
        if s[pos..].starts_with("don't()") {
            is_on = false;
            pos += "don't()".len();
        } else if s[pos..].starts_with("do()") {
            is_on = true;
            pos += "do()".len();
        } else if let Some(cap) = mul_regex.captures(&s[pos..]) {
            let x = cap[1].parse::<u32>().unwrap();
            let y = cap[2].parse::<u32>().unwrap();
            result.push((x, y, is_on));
            pos += cap.get(0).unwrap().as_str().len();
        } else {
            pos += 1;
        }
    }

    result
}