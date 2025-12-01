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
        // For positive moves: crossings = raw / 100 (how many times we passed 100)
        // For negative moves: crossings = ceil(-raw / 100) = (-raw + 99) / 100
        // Special cases: ending at 0 when going left adds 1, starting at 0 going left subtracts 1
        let crossings = if raw > 0 {
            raw / 100
        } else {
            // raw <= 0: going left either to the 0, or beyond it
            let wraps = (-raw + 99) / 100;  // ceil(-raw / 100)
            let ends_at_zero = if new_current == 0 { 1 } else { 0 };
            let starts_at_zero = if current == 0 { 1 } else { 0 };
            (wraps + ends_at_zero).saturating_sub(starts_at_zero)
        };
        
        (new_exact_zeros, total_crossings + crossings as u32, new_current)
    });
    let (exact_zeros, total_crossings, _) = result;
    (exact_zeros, total_crossings)
}