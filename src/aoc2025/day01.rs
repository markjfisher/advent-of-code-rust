pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    do_rotations(input, 50) 
}

pub fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    input.0.clone().into_iter().filter(|&x| x == 0).count() as u32
}

pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    input.1.clone().into_iter().sum::<u32>()
}

pub fn do_rotations(input: &str, initial: i32) -> (Vec<u32>, Vec<u32>) {
    let mut current = initial;
    input.lines().map(|line| {
        let dir = line.as_bytes()[0] as char;
        let count = line[1..].parse::<i32>().unwrap();
        let delta = if dir == 'L' { -count } else { count };
        let raw = current + delta;
        
        // Normalize position to 0-99 range
        let new_current = ((raw % 100) + 100) % 100;
        
        // Count clicks: number of times we cross 0
        let clicks = if raw < 0 {
            // Wrapping from negative: count wraps, plus one if we end at 0
            let wraps = (-raw + 99) / 100;
            let extra = if new_current == 0 { 1 } else { 0 };
            let adjust = if current == 0 && dir == 'L' { 1 } else { 0 };
            (wraps + extra).saturating_sub(adjust)
        } else if raw == 0 && dir == 'L' {
            // Going left and ending exactly at 0: crossed 0 once
            1
        } else {
            // Wrapping from positive: count how many times we cross 100
            raw / 100
        } as u32;
        
        current = new_current;
        (current as u32, clicks)
    }).collect()
}