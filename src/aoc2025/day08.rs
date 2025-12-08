use std::cmp::Reverse;

#[derive(Debug)]
pub struct DSU {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
}

// Disjoint Set Union (DSU) with path compression and union by size

// Here's an example of how DSU works:
// Initial DSU:
//   parent = [0, 1, 2, 3, 4]
//   size   = [1, 1, 1, 1, 1]
// Each node is its own root.
// Now let's do some unions. These could be any order, but in the puzzle they are by "nearest coordinate pairs".
// For our example we'll make up the pairs as (0,3), (1, 4), (3, 4), (2, 4).
//
// union(0, 3)
//   find(0) = 0
//   find(3) = 3
// Both have size 1, so we'll just attach 3 to 0.
//   parent[3] = 0
//   size[0] = 2
//   parent = [0, 1, 2, 0, 4]
//   size   = [2, 1, 1, 1, 1]
// Note how entry 3 is now 0, which is its parent. Its size is no longer important.
//
// union(1, 4)
//   find(1) = 1
//   find(4) = 4
// Both have size 1, so we'll just attach 4 to 1.
//   parent[4] = 1
//   size[1] = 2
//   parent = [0, 1, 2, 0, 1]
//   size   = [2, 2, 1, 1, 1]
//
// union(3, 4)
//   find(3) = 0 -> root = 0, size[0] = 2
//   find(4) = 1 -> root = 1, size[1] = 2
//   same size, so we'll attach 1 to 0.
//   parent[1] = 0
//   size[0] = 4
//   parent = [0, 0, 2, 0, 1] - uncompressed until find is called
//   size   = [4, x, 1, x, x] - don't care about the x entries, as they are not roots
//
// union(2, 4)
//   find(2) = 2,             size[2] = 1
//   find(4) = 0 -> root = 0, size[0] = 4
//   so we'll attach 2 to 0 as node 0 has larger size
//   parent[2] = 0
//   size[0] = 5
//   parent = [0, 0, 0, 0, 0]
//   size   = [5, x, x, x, x]
// 
// and this completes the unions, as node 0 has size of number of entries
// For part 2 solution, the 2 items being unioned in the end were (2, 4), so look up those items coordinates as needed

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    // do path compression during find, so we change the entry's parent as we look for it
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) {
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

pub fn parse_coords(input: &str) -> Vec<(i64, i64, i64)> {
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

fn top3_product(dsu: &DSU) -> u64 {
    let n = dsu.parent.len();
    let mut sizes: Vec<usize> = Vec::new();

    // Only roots (parent[i] == i) have valid size[i]
    for i in 0..n {
        if dsu.parent[i] == i {
            sizes.push(dsu.size[i]);
        }
    }

    // Sort descending and take top 3
    sizes.sort_unstable_by_key(|&s| Reverse(s));
    sizes.iter().take(3).product::<usize>() as u64
}

pub fn solve_both(coordinates: &[(i64, i64, i64)], part1_limit: usize) -> (u64, u64) {
    let n = coordinates.len();

    // build all unique pairs with squared distance
    let mut pairs: Vec<(usize, usize, u64)> = Vec::new();
    pairs.reserve(n * (n - 1) / 2);

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

    // sort by squared distance ascending
    pairs.sort_unstable_by_key(|&(_, _, d)| d);

    // walk pairs once, tracking answers for both parts
    let mut dsu = DSU::new(n);

    let mut part1_answer: Option<u64> = None;
    let mut part2_answer: Option<u64> = None;

    for (idx, &(i, j, _)) in pairs.iter().enumerate() {
        // count how many pairs we've gone over for p1
        let pairs_seen = idx + 1;

        // do union for p2. if already in same circuit, this does nothing
        dsu.union(i, j);

        // if we've just hit the LIMIT'th pair, snapshot the DSU state for part 1
        if pairs_seen == part1_limit && part1_answer.is_none() {
            part1_answer = Some(top3_product(&dsu));
        }

        // after this union, check if everything is in a single circuit
        let root = dsu.find(i);
        if dsu.size[root] as usize == n && part2_answer.is_none() {
            let (x1, _, _) = coordinates[i];
            let (x2, _, _) = coordinates[j];
            part2_answer = Some((x1 as u64) * (x2 as u64));
            break; // fully connected, no need to process more pairs
        }
    }

    // fun with options! get the values out and assert there was one.
    (
        part1_answer.expect("Part 1 answer not found"),
        part2_answer.expect("Part 2 answer not found"),
    )
}

pub fn parse(input: &str) -> (u64, u64) {
    let coords = parse_coords(input);
    solve_both(&coords, 1000)
}

pub fn part1(result: &(u64, u64)) -> u64 {
    result.0
}

pub fn part2(result: &(u64, u64)) -> u64 {
    result.1
}
