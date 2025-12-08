use std::collections::HashMap;

#[derive(Debug)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<u64>,
}

// Disjoint Set Union (DSU) with path compression and union by size
impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return; // already in same circuit
        }

        // union by size: attach smaller to larger
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

// Parsed representation: just the list of junction box coordinates.
pub fn parse(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect()
}

/// Core solver, parameterised by the number of shortest pairs to use.
pub fn solve(coordinates: &[(i64, i64, i64)], limit: usize) -> u64 {
    let n = coordinates.len();
    let mut pairs: Vec<(usize, usize, u64)> = Vec::new();

    // 1) Build all unique pairs with squared distance
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1, z1) = coordinates[i];
            let (x2, y2, z2) = coordinates[j];

            let dx = x1 - x2;
            let dy = y1 - y2;
            let dz = z1 - z2;

            let dist2 = (dx * dx + dy * dy + dz * dz) as u64;
            pairs.push((i, j, dist2));
        }
    }

    // 2) Sort by squared distance ascending
    pairs.sort_unstable_by_key(|&(_, _, d)| d);

    // 3) DSU: connect the first `limit` pairs in order.
    let mut dsu = DSU::new(n);
    let mut used_pairs = 0;

    for &(i, j, _) in pairs.iter() {
        if used_pairs >= limit {
            break;
        }
        used_pairs += 1;     // this pair *always* counts toward the limit
        dsu.union(i, j);     // might or might not change circuits
    }

    // 4) Count circuit sizes (group by DSU root)
    let mut circuit_sizes: HashMap<usize, u64> = HashMap::new();
    for i in 0..n {
        let root = dsu.find(i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }

    // 5) Take the 3 largest and multiply
    let mut sizes: Vec<u64> = circuit_sizes.into_values().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // descending
    sizes.iter().take(3).product()
}

pub fn part1(coords: &Vec<(i64, i64, i64)>) -> u64 {
    solve(coords, 1000)
}

pub fn part2(coords: &Vec<(i64, i64, i64)>) -> u64 {
    solve(coords, 1000)
}
