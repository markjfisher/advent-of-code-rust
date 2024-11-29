pub fn parse(input: &str) -> Vec<i32> {
    fn h(b: u8) -> i32 {
        match b {
            b'(' => 1,
            b')' => -1,
            _ => unreachable!(),
        }
    }
    input.bytes().map(h).collect()
}


pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> usize {
    let mut floor: i32 = 0;

    for (i, c) in input.iter().enumerate() {
        floor += c;
        if floor < 0 {
            return i + 1;
        }
    }
    unreachable!()
}