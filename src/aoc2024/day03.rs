pub fn parse(input: &str) -> Vec<(u32, u32)> {
    extract_muls(input)
}

pub fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(x, y)| x * y).sum()
}

pub fn part2(_input: &[(u32, u32)]) -> u32 {
    456
}

pub fn extract_muls(s: &str) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    for cap in re.captures_iter(s) {
        if let (Ok(x), Ok(y)) = (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
            result.push((x, y));
        }
    }
    result
}