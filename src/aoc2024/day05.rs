use crate::util::iter::*;
use crate::util::parse::*;
use petgraph::Graph;
use petgraph::algo::toposort;

pub fn parse(input: &str) -> (usize, usize) {
    let (rules_data, checks_data) = input.split_once("\n\n").unwrap();

    // Create the graph and add edges from rules_data
    let mut graph = Graph::<usize, ()>::new();
    let mut node_map = std::collections::HashMap::new();

    // Add edges: if a|b, then a->b means "a must come before b"
    rules_data
        .iter_unsigned::<usize>()
        .chunk::<2>()
        .for_each(|[before, after]| {
            let from_node = *node_map.entry(before).or_insert_with(|| graph.add_node(before));
            let to_node = *node_map.entry(after).or_insert_with(|| graph.add_node(after));
            graph.add_edge(from_node, to_node, ());
        });

    checks_data.lines().fold(
        (0, 0),
        |(sum_valid_middles, sum_invalid_middles), test_line| {
            let values: Vec<usize> = test_line.iter_unsigned().collect();
            
            // Create a subgraph just for these values
            let mut sequence_graph = Graph::<usize, ()>::new();
            let mut seq_node_map = std::collections::HashMap::new();
            
            // Add nodes and edges for this sequence
            for &value in &values {
                seq_node_map.entry(value).or_insert_with(|| sequence_graph.add_node(value));
            }
            
            // Add edges from the main graph that apply to our sequence
            for &value in &values {
                for &next in &values {
                    if value != next {
                        let from = node_map[&value];
                        let to = node_map[&next];
                        if graph.contains_edge(from, to) {
                            let seq_from = seq_node_map[&value];
                            let seq_to = seq_node_map[&next];
                            sequence_graph.add_edge(seq_from, seq_to, ());
                        }
                    }
                }
            }
            
            // Try to topologically sort this sequence
            let sorted = toposort(&sequence_graph, None).unwrap_or_default();
            let result: Vec<_> = sorted.iter().map(|&n| sequence_graph[n]).collect();
            
            let is_correct = values == result;
            let middle_value = result[values.len() / 2];
            
            if is_correct {
                (sum_valid_middles + middle_value, sum_invalid_middles)
            } else {
                (sum_valid_middles, sum_invalid_middles + middle_value)
            }
        },
    )
}

pub fn part1(input: &(usize, usize)) -> usize {
    input.0
}

pub fn part2(input: &(usize, usize)) -> usize {
    input.1
}