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
        let mut clicks: u32 = 0;
        let mut new_current = current;
        let dir = line.as_bytes()[0] as char;
        let count = line[1..].parse::<i32>().unwrap();
        if dir == 'L' {
            new_current = new_current - count;
        } else {
            new_current = new_current + count;
        }
        while new_current < 0 { new_current += 100; clicks += 1; }
        // after being on the left, we may end up on 0, which has to be counted as a click
        if new_current == 0 { clicks += 1; }
        // compensate for starting at 0 and going Left, first click should be ignored
        if current == 0 && dir == 'L' { clicks -= 1; }
        while new_current >= 100 { new_current -= 100; clicks += 1; }
        current = new_current;

        (current as u32, clicks as u32)
    }).collect()}