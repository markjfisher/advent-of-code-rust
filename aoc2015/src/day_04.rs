use crypto::digest::Digest;
use crypto::md5::Md5;

#[aoc(day4, part1)]
pub fn part1(secret: &str) -> u32 {
    part1_solve(secret, 0)
}

fn part1_solve(secret: &str, start_at: u32) -> u32 {
    solver(secret, start_at, |hash| hash[0..2] == [0; 2] && (hash[2] & 0xF0) == 0)
}

#[aoc(day4, part2)]
pub fn part2(secret: &str) -> u32 {
    // use the value from part1 as starting position to reduce time a bit. from 140ms to 105ms in release
    part2_solve(secret, 254575)
}

fn part2_solve(secret: &str, start_at: u32) -> u32 {
    solver(secret, start_at, |hash| hash[0..3] == [0; 3])
}


fn solver(secret: &str, start_at: u32, is_valid: impl Fn(&[u8; 16]) -> bool) -> u32 {
    let mut hash = [0; 16];

    let mut hasher = Md5::new();
    hasher.input_str(secret);

    (start_at..=u32::MAX)
        .find(|i| {
            let mut hasher = hasher;
            hasher.input_str(&i.to_string());
            hasher.result(&mut hash);
            is_valid(&hash)
        }).unwrap()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043
    // starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    fn example1() {
        assert_eq!(part1_solve("abcdef", 609000), 609043);
    }

    #[test]
    // If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash
    // starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like
    // 000006136ef....
    fn example2() {
        assert_eq!(part1_solve("pqrstuv", 1048900), 1048970);
    }
}