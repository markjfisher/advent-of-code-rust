use pathfinding::prelude::count_paths;

const USE_PATHFINDING: bool = false;

pub fn parse(input: &str) -> Vec<u64> {
    let mut parts = input.split("\n\n");
    
    let sequences: Vec<&str> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .collect();
    
    let count_fn = if USE_PATHFINDING {
        count_valid_combinations_pathfinding
    } else {
        count_valid_combinations_dp
    };
    
    parts
        .next()
        .unwrap()
        .lines()
        .map(|input| count_fn(input, &sequences))
        .collect()
}

pub fn part1(input: &[u64]) -> u32 {
    input.iter()
        .filter(|&&count| count > 0)
        .count() as u32
}

pub fn part2(input: &[u64]) -> u64 {
    input.iter().sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    index: usize,
    max_length: usize,
}

impl Pos {
    fn new(valid_sequences: &[&str]) -> Self {
        let max_length = valid_sequences.iter()
            .map(|x| x.len())
            .max()
            .unwrap();
        
        Self {
            index: 0,
            max_length,
        }
    }

    fn successors(&self, input: &str, valid_sequences: &[&str]) -> Vec<Pos> {
        let mut result = Vec::new();
        for i in 1..=self.max_length {
            if let Some(next) = input.get(self.index..self.index + i) {
                for &part in valid_sequences {
                    if next == part {
                        result.push(Pos { 
                            index: self.index + i,
                            max_length: self.max_length,
                        });
                    }
                }
            }
        }
        result
    }
}

fn count_valid_combinations_pathfinding(input: &str, valid_sequences: &[&str]) -> u64 {
    count_paths(
        Pos::new(valid_sequences),
        |p| p.successors(input, valid_sequences),
        |p| p.index == input.len()
    ) as u64
}

fn count_valid_combinations_dp(input: &str, valid_sequences: &[&str]) -> u64 {
    // Original dynamic programming implementation
    let mut ways_from_index = vec![0u64; input.len() + 1];
    ways_from_index[input.len()] = 1;
    
    for i in (0..input.len()).rev() {
        for &seq in valid_sequences {
            if input[i..].starts_with(seq) {
                ways_from_index[i] = ways_from_index[i].saturating_add(ways_from_index[i + seq.len()]);
            }
        }
    }
    
    ways_from_index[0]
}
