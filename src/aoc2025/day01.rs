pub fn parse(input: &str) -> (u32, u32) {
    do_rotations(input, 50)
}

pub fn part1(input: &(u32, u32)) -> u32 {
    input.0
}

pub fn part2(input: &(u32, u32)) -> u32 {
    input.1
}

pub fn do_rotations(input: &str, initial: i32) -> (u32, u32) {
    let result = input.lines().fold((0, 0, initial), |(exact_zeros, total_crossings, current), line| {
        let dir = line.as_bytes()[0] as char;
        let count = line[1..].parse::<i32>().unwrap();
        let delta = if dir == 'L' { -count } else { count };
        let raw = current + delta;
        
        // Normalize position to 0-99 range
        let new_current = ((raw % 100) + 100) % 100;
        
        // Count exact zeros (part1)
        let new_exact_zeros = exact_zeros + if new_current == 0 { 1 } else { 0 };
        
        // Count crossings (part2): number of times we cross 0
        let crossings = if raw < 0 {
            // Going left into negative: count wraps needed, plus one if we end at 0
            let wraps = (-raw + 99) / 100;
            let extra = if new_current == 0 { 1 } else { 0 };
            // Special case: starting at 0 and going left doesn't count the first wrap
            let adjust = if current == 0 && dir == 'L' { 1 } else { 0 };
            (wraps + extra).saturating_sub(adjust)
        } else if raw == 0 && dir == 'L' {
            // Going left and ending exactly at 0: crossed once
            1
        } else {
            // Going right: number of times we passed 100 (which wraps to 0)
            raw / 100
        };
        
        (new_exact_zeros, total_crossings + crossings as u32, new_current)
    });
    let (exact_zeros, total_crossings, _) = result;
    (exact_zeros, total_crossings)
}