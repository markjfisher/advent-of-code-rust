// use std::collections::FastMap;
use crate::util::hash::*;

pub fn parse(input: &str) -> FastMap<&str, Vec<&str>> {
    let mut graph: FastMap<&str, Vec<&str>> = FastMap::new();

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let (from, rest) = line
            .split_once(':')
            .expect("Each line should look like `name: child1 child2 ...`");

        let from = from.trim();
        let children: Vec<&str> = rest.split_whitespace().collect();
        graph.insert(from, children);
    }

    graph
}

pub fn dfs_part1<'a>(
    node: &'a str,
    target: &str,
    graph: &'a FastMap<&'a str, Vec<&'a str>>,
    memo: &mut FastMap<&'a str, u64>,
) -> u64 {
    if let Some(&v) = memo.get(node) {
        return v;
    }

    if node == target {
        memo.insert(node, 1);
        return 1;
    }

    let mut total = 0;
    if let Some(neighbours) = graph.get(node) {
        for &next in neighbours {
            total += dfs_part1(next, target, graph, memo);
        }
    }

    memo.insert(node, total);
    total
}

pub fn dfs_part2<'a>(
    node: &'a str,
    target: &str,
    mask: u8,
    graph: &'a FastMap<&'a str, Vec<&'a str>>,
    memo: &mut FastMap<(&'a str, u8), u64>,
    visit1: &str,
    visit2: &str,
) -> u64 {
    // Update the mask based on current node
    let mut new_mask = mask;
    if node == visit1 {
        new_mask |= 1; // set bit 0
    }
    if node == visit2 {
        new_mask |= 2; // set bit 1
    }

    let key = (node, new_mask);
    if let Some(&v) = memo.get(&key) {
        return v;
    }

    if node == target {
        // Only count paths that have seen BOTH visit1 and visit2
        let res = if new_mask == 0b11 { 1 } else { 0 };
        memo.insert(key, res);
        return res;
    }

    let mut total = 0;
    if let Some(neighbours) = graph.get(node) {
        for &next in neighbours {
            total += dfs_part2(next, target, new_mask, graph, memo, visit1, visit2);
        }
    }

    memo.insert(key, total);
    total

}


pub fn part1(input: &FastMap<&str, Vec<&str>>) -> u64 {
    let mut memo_p1 = FastMap::new();
    dfs_part1("you", "out", input, &mut memo_p1)
}

pub fn part2(input: &FastMap<&str, Vec<&str>>) -> u64 {
    let mut memo_p2 = FastMap::new();
    dfs_part2("svr", "out", 0, input, &mut memo_p2, "fft", "dac")
}
