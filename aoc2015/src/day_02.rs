type Gift = (u32, u32, u32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input.lines()
        .map(|line| {
            // order them smallest to longest so l and w are the two shortest sides
            let mut g = line.trim().split('x').map(|d| d.parse().unwrap());
            let l = g.next().unwrap();
            let w = g.next().unwrap();
            let h = g.next().unwrap();
            let mut v = vec![l, w, h];
            v.sort();
            (v[0], v[1], v[2])
        }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Gift]) -> u32 {
    // we already have ordered lengths, so can take l and w as shortest sides
    input.iter().map(|&(l,w,h)| {
        2 * l * w + 2 * w * h + 2 * h * l + l * w
    }).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Gift]) -> u32 {
    // we already have ordered lengths, so can take l and w as shortest sides
    input.iter().map(|&(l,w,h)| {
        2 * (l + w) + l * w * h
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    fn example1() {
        assert_eq!(solve_part1(&input_generator("2x3x4")), 58);
    }

    #[test]
    // A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
    fn example2() {
        assert_eq!(solve_part1(&input_generator("1x1x10")), 43);
    }

    #[test]
    // A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    fn example3() {
        assert_eq!(solve_part2(&input_generator("2x3x4")), 34);
    }

    #[test]
    // A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.
    fn example4() {
        assert_eq!(solve_part2(&input_generator("1x1x10")), 14);
    }
}