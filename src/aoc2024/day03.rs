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
    let mut result = Vec::new();
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut on_offs: Vec<(usize, bool)> = Vec::new();
    let mut pos = 0;
    while pos < s.len() {
        if s[pos..].starts_with("don't()") {
            on_offs.push((pos, false));
            pos += "don't()".len();
        } else if s[pos..].starts_with("do()") {
            on_offs.push((pos, true));
            pos += "do()".len();
        } else {
            pos += 1;
        }
    }

    for cap in re.captures_iter(s) {
        if let (Ok(x), Ok(y)) = (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
            let match_pos = cap.get(0).unwrap().start();
            let is_on = on_offs.iter()
                .filter(|&&(pos, _)| pos < match_pos)
                .last()
                .map_or(true, |&(_, state)| state);
            
            result.push((x, y, is_on));
        }
    }
    result
}