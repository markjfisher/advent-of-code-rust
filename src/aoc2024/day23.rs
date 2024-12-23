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