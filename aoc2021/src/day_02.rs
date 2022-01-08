pub struct SubMove {
    h: i32,
    d: i32,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<SubMove> {
    input.lines().map(|line| {
        // split the line into instruction and value
        let mut s = line.split_whitespace();
        let instruction = s.next().unwrap();
        let val: i32 = s.next().unwrap().parse().unwrap();
        // convert to [x,y] pair for forward/depth values from instruction
        match instruction {
            "forward" => SubMove { h: val, d: 0i32 },
            "up" => SubMove { h: 0i32, d: -val },
            "down" => SubMove { h: 0i32, d: val },
            _ => unreachable!()
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part_01(input: &[SubMove]) -> i32 {
    let output = input.iter().fold((0, 0), |(horizon, depth), mv| {
        match mv {
            SubMove { h, d: 0 } => (horizon + h, depth),
            SubMove { h: 0, d } => (horizon, depth + d),
            _ => unreachable!()
        }
    });
    output.0 * output.1
}

#[aoc(day2, part2)]
pub fn solve_part_02(input: &[SubMove]) -> i32 {
    let output = input.iter().fold((0, 0, 0), |(horizon, depth, aim), mv| {
        match mv {
            SubMove { h, d: 0 } => (horizon + h, depth + aim * h, aim),
            SubMove { h: 0, d } => (horizon, depth, aim + d),
            _ => unreachable!()
        }
    });
    output.0 * output.1
}
