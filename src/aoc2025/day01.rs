pub fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| {
        let dir = line.as_bytes()[0] as char;
        let mut count = line[1..].parse::<i32>().unwrap();
        if dir == 'L' {
            count = 100 - (count % 100);
        } else {
            count = count % 100;
        }
        count as u32
    }).collect()
}

pub fn part1(input: &Vec<u32>) -> u32 {
    input.iter()
        .scan(50u32, |acc, x| {
            *acc = ((*acc as u64 + *x as u64) % 100) as u32;
            Some(*acc)
        })
        .filter(|&x| x == 0)
        .count() as u32
}

pub fn part2(_input: &Vec<u32>) -> u32 {
    0
}

