pub fn parse(input: &str) -> Vec<u64> {
    let mut parts = input.split("\n\n");
    
    let sequences: Vec<&str> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .collect();
    
    // compute combinations on the input strings once, part 1 is a count of those that are not 0
    parts
        .next()
        .unwrap()
        .lines()
        .map(|input| count_valid_combinations(input, &sequences))
        .collect()
}

fn count_valid_combinations(input: &str, valid_sequences: &[&str]) -> u64 {
    // ways_from_index[i] represents the number of ways to make the substring from i to end
    let mut ways_from_index = vec![0u64; input.len() + 1];
    ways_from_index[input.len()] = 1;  // empty string has one way to make it
    
    for i in (0..input.len()).rev() {
        for &seq in valid_sequences {
            if input[i..].starts_with(seq) {
                ways_from_index[i] = ways_from_index[i].saturating_add(ways_from_index[i + seq.len()]);
            }
        }
    }
    
    ways_from_index[0]
}

pub fn part1(input: &[u64]) -> u32 {
    input.iter()
        .filter(|&&count| count > 0)
        .count() as u32
}

pub fn part2(input: &[u64]) -> u64 {
    input.iter().sum()
}