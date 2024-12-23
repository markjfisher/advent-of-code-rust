use crate::util::hash::*;

type Input<'a> = FastMap<&'a str, FastSet<&'a str>>;

pub fn parse(input: &str) -> Input<'_> {
    input.lines().fold(FastMap::new(), |mut map, line| {
        let (a, b) = line.split_once('-').unwrap();
        map.entry(a).or_insert_with(FastSet::new).insert(b);
        map.entry(b).or_insert_with(FastSet::new).insert(a);
        map
    })
}

pub fn part1(input: &Input<'_>) -> usize {
    let mut count = 0;
    
    // For each node that starts with 't'
    for (&name, connections) in input {
        if !name.starts_with('t') {
            continue;
        }
        
        // For each pair of connections from our 't' node
        for &first in connections {
            for &second in connections {
                // Skip if either:
                // 1. first >= second (avoid duplicate pairs)
                // 2. either connection starts with 't' and comes before our current name
                //    (avoid counting the same triple from multiple 't' perspectives)
                if first >= second || 
                   (first.starts_with('t') && first < name) ||
                   (second.starts_with('t') && second < name) {
                    continue;
                }
                
                // If these two nodes are also connected, we found a triple
                if input.get(first)
                       .map_or(false, |set| set.contains(second)) {
                    println!("{} {} {}", name, first, second);
                    count += 1;
                }
            }
        }
    }
    
    count
}

pub fn part2(_input: &Input<'_>) -> u32 {
    456
}