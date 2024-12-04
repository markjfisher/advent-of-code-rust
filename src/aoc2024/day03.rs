use tailcall::tailcall;

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
    let re = regex::Regex::new(r"(do(?:n't)?\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut result = Vec::new();
    let mut is_on = true;

    for cap in re.captures_iter(s) {
        let matched_str = cap.get(0).unwrap().as_str();
        match matched_str {
            "don't()" => is_on = false,
            "do()" => is_on = true,
            _ => {
                if let (Some(x), Some(y)) = (cap.get(2), cap.get(3)) {
                    let x = x.as_str().parse::<u32>().unwrap();
                    let y = y.as_str().parse::<u32>().unwrap();
                    result.push((x, y, is_on));
                }
            }
        }
    }

    result
}

// This is a non mutating version of extract_muls. Has almost identical run time
pub fn extract_muls_tr(s: &str) -> Vec<(u32, u32, bool)> {
    let re = regex::Regex::new(r"(do(?:n't)?\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    
    #[tailcall]
    fn do_extract<'a>(
        mut captures: regex::CaptureMatches<'a, 'a>,
        mut result: Vec<(u32, u32, bool)>,
        is_on: bool,
    ) -> Vec<(u32, u32, bool)> {
        match captures.next() {
            None => result,
            Some(cap) => {
                let matched_str = cap.get(0).unwrap().as_str();
                match matched_str {
                    "don't()" => do_extract(captures, result, false),
                    "do()" => do_extract(captures, result, true),
                    _ => {
                        if let (Some(x), Some(y)) = (cap.get(2), cap.get(3)) {
                            let x = x.as_str().parse::<u32>().unwrap();
                            let y = y.as_str().parse::<u32>().unwrap();
                            result.push((x, y, is_on));
                        }
                        do_extract(captures, result, is_on)
                    }
                }
            }
        }
    }

    do_extract(re.captures_iter(s), Vec::new(), true)
}