use crate::util::hash::*;
use petgraph::graphmap::GraphMap;
use petgraph::Undirected;

use crate::util::tomita::Tomita;
// use crate::util::bronkerbosch::BronKerbosch;

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
    // this version reduces original implementation complexity from 62ms to 0.5ms by not double looping the connections
    input.iter()
        // only starting with 't'
        .filter(|(&name, _)| name.starts_with('t'))
        .flat_map(|(&n1, edges)| {
            edges.iter().flat_map(move |&n2| {
                input[n2]
                    .intersection(edges)
                    .map(move |&n3| {
                        let mut triple = [n1, n2, n3];
                        triple.sort_unstable();
                        triple
                    })
            })
        })
        .collect::<FastSet<_>>()
        .len()
}

pub fn part2(nodes: &Input<'_>) -> String {
    let mut visited = FastSet::new();
    let mut largest_group = FastSet::new();

    for &start in nodes.keys() {
        if visited.contains(start) {
            continue;
        }

        let mut current_group = FastSet::new();
        let mut stack = vec![start];

        // another dfs! find potential group members
        while let Some(node) = stack.pop() {
            // do we connect to all current members?
            if current_group.iter().all(|&member| nodes[node].contains(member)) {
                current_group.insert(node);
                visited.insert(node);

                // only add neighbors that connect to all current members
                stack.extend(
                    nodes[node]
                        .iter()
                        .filter(|&&n| !current_group.contains(n) && 
                               current_group.iter().all(|&member| nodes[n].contains(member)))
                );
            }
        }

        if current_group.len() > largest_group.len() {
            largest_group = current_group;
        }
    }

    let mut result: Vec<_> = largest_group.iter().copied().collect();
    result.sort_unstable();
    result.join(",")
}

// Use various graph algorithms to find cliques
// Direct:         1.2ms
// Tomita:        11.8ms
// BronKerbosch: 168.0ms
pub fn _part2_algos(input: &Input<'_>) -> String {
    let mut graph = GraphMap::<&str, (), Undirected>::new();
    
    // Add all edges (in both directions since it's undirected)
    for (&from, edges) in input {
        for &to in edges {
            graph.add_edge(from, to, ());
        }
    }
    
    // Find all cliques using Tomita's algorithm
    let mut clique_alg = Tomita::new(graph);
    
    // Find all the cliques using BronKerbosch - SLOWEST ~170ms
    // let mut clique_alg = BronKerbosch::new(graph);

    clique_alg.compute();
    
    // Get all cliques and sort them by size (largest first)
    let mut cliques = clique_alg.cliques().to_vec();
    cliques.sort_by_key(|clique| -(clique.len() as i32));
    
    // print all cliques with their sizes
    for (i, clique) in cliques.iter().enumerate() {
        println!("Clique {}: size={} members={:?}", 
                i, 
                clique.len(), 
                clique.iter().collect::<Vec<_>>());
    }
    
    // Take the largest clique, sort its members, and join with commas
    let largest = &cliques[0];
    let mut members: Vec<_> = largest.iter().copied().collect();
    members.sort_unstable();
    members.join(",")
}


