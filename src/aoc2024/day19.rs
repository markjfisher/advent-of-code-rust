pub fn parse(input: &str) -> (Vec<&str>, Vec<String>) {
    let mut parts = input.split("\n\n");
    
    // Parse valid sequences (first line)
    let sequences = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .collect();
    
    // Parse input strings (remaining lines)
    let inputs = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    
    (sequences, inputs)
}

pub fn part1(input: &(Vec<&str>, Vec<String>)) -> u32 {
    let (sequences, inputs) = input;
    
    // No need for conversion anymore
    inputs.iter()
        .filter(|input| !find_sequence_combinations(input, sequences).is_empty())
        .count() as u32
}

pub fn part2(_input: &(Vec<&str>, Vec<String>)) -> u32 {
    456
}

pub fn find_sequence_combinations(input: &str, valid_sequences: &[&str]) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    
    fn recurse(
        remaining: &str,
        valid_seqs: &[&str],
        current_sequence: Vec<String>,
        all_results: &mut Vec<Vec<String>>
    ) {
        if remaining.is_empty() {
            if !all_results.contains(&current_sequence) {
                all_results.push(current_sequence);
            }
            return;
        }
        
        for seq in valid_seqs {
            if remaining.starts_with(seq) {
                let mut new_sequence = current_sequence.clone();
                new_sequence.push(seq.to_string());
                
                recurse(
                    &remaining[seq.len()..],
                    valid_seqs,
                    new_sequence,
                    all_results
                );
            }
        }
    }
    
    recurse(input, valid_sequences, Vec::new(), &mut results);
    results
}