use pathfinding::prelude::bfs;

#[derive(Debug, Clone)]
pub struct Machine {
    pub num_lights: u8,
    pub target: u64,          // bits: 1 = ON, 0 = OFF
    pub button_masks: Vec<u64>, // one mask per button
    pub joltages: Vec<u32>,   // same length as num_lights
}

pub fn parse_machine(line: &str) -> Machine {
    // Split off the joltages part: "... } {3,5,4,7}"
    let mut outer_parts = line.split('{');
    let left = outer_parts
        .next()
        .expect("line should contain a '{' for joltages")
        .trim();
    let jolts_part = outer_parts
        .next()
        .expect("joltages part missing")
        .trim();

    // Strip trailing '}' from joltages and parse numbers
    let jolts_str = jolts_part
        .trim_end_matches('}')
        .trim();

    let joltages: Vec<u32> = if jolts_str.is_empty() {
        Vec::new()
    } else {
        jolts_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().expect("invalid joltage"))
            .collect()
    };

    // Extract lights pattern from [...] on the left side
    let start_bracket = left.find('[').expect("missing '['");
    let end_bracket = left[start_bracket + 1..]
        .find(']')
        .expect("missing ']'")
        + start_bracket
        + 1;

    let lights_str = &left[start_bracket + 1..end_bracket];
    let num_lights = lights_str.len() as u8;

    // Build target bitmask: bit i = 1 if lights_str[i] == '#'
    let mut target: u64 = 0;
    for (i, ch) in lights_str.chars().enumerate() {
        if ch == '#' {
            target |= 1u64 << i;
        } else if ch != '.' {
            panic!("unexpected char in lights pattern: {ch}");
        }
    }

    // Everything after the ']' are the button specs: "(3) (1,3) ..."
    let buttons_part = &left[end_bracket + 1..];

    let mut button_masks = Vec::new();

    for token in buttons_part.split_whitespace() {
        if !token.starts_with('(') {
            continue;
        }
        // Remove surrounding parentheses
        let inner = token
            .trim_start_matches('(')
            .trim_end_matches(')');

        if inner.is_empty() {
            continue;
        }

        let mut mask: u64 = 0;
        for idx_str in inner.split(',') {
            let idx: usize = idx_str.trim().parse().expect("invalid index");
            mask |= 1u64 << idx;
        }
        button_masks.push(mask);
    }

    // Sanity check: joltages (if present) should match number of lights
    if !joltages.is_empty() && joltages.len() != num_lights as usize {
        panic!(
            "joltages length {} does not match number of lights {}",
            joltages.len(),
            num_lights
        );
    }

    Machine {
        num_lights,
        target,
        button_masks,
        joltages,
    }
}

pub fn parse(input: &str) -> (u32, u32) {
    let machines: Vec<Machine> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_machine)
        .collect();
    let part1 = machines.iter().map(min_light_presses).sum::<Option<usize>>().unwrap();
    let part2 = machines.iter().map(min_joltage_presses).sum::<Option<i64>>().unwrap();
    // let part2 = 0;
    (part1 as u32, part2 as u32)
}

pub fn min_light_presses(machine: &Machine) -> Option<usize> {
    let start: u64 = 0;
    let goal: u64 = machine.target;

    if start == goal {
        return Some(0);
    }

    let path = bfs(
        &start,
        |state| {
            // From a given state, you can press any button:
            // successor = state XOR button_mask
            machine
                .button_masks
                .iter()
                .map(|&mask| state ^ mask)
                .collect::<Vec<u64>>()
        },
        |state| *state == goal,
    )?;

    // Path includes the start state, so presses = path length - 1
    Some(path.len() - 1)
}

pub fn min_joltage_presses(machine: &Machine) -> Option<i64> {
    let (a_full, target_full) = build_matrix(machine);
    let m = a_full[0].len();

    // Edge case: if target is all zero, no presses needed
    if target_full.iter().all(|&v| v == 0) {
        return Some(0);
    }

    let (pivot_rows, pivot_cols) = find_pivots(&a_full);
    let r = pivot_rows.len();
    if r == 0 {
        // A is all zeros: only solution is target == 0 (handled above)
        return None;
    }

    // Build B (r x r) from pivot rows/cols
    let mut b_matrix = vec![vec![0i64; r]; r];
    for (i, &row_idx) in pivot_rows.iter().enumerate() {
        for (k, &col_idx) in pivot_cols.iter().enumerate() {
            b_matrix[i][k] = a_full[row_idx][col_idx];
        }
    }

    // Column-wise matrix restricted to pivot rows: A_cols_r[j][i]
    let mut a_cols_r = vec![vec![0i64; r]; m];
    for j in 0..m {
        for (i, &row_idx) in pivot_rows.iter().enumerate() {
            a_cols_r[j][i] = a_full[row_idx][j];
        }
    }

    // Target restricted to pivot rows
    let mut rhs_r: Vec<i64> = pivot_rows.iter().map(|&ri| target_full[ri]).collect();

    use std::collections::HashSet;
    let pivot_set: HashSet<usize> = pivot_cols.iter().cloned().collect();
    let free_cols: Vec<usize> = (0..m).filter(|j| !pivot_set.contains(j)).collect();

    let mut best: Option<i64> = None;
    let mut free_press = vec![0i64; free_cols.len()];

    dfs_free(
        0,
        &free_cols,
        &a_cols_r,
        &mut rhs_r,
        &mut free_press,
        0,
        &b_matrix,
        &pivot_cols,
        &a_full,
        &target_full,
        &mut best,
    );

    best
}

pub fn part1(input: &(u32, u32)) -> u32 {
    input.0
}

pub fn part2(input: &(u32, u32)) -> u32 {
    input.1
}


// --------------------------------------------------------------------
// maths

fn build_matrix(machine: &Machine) -> (Vec<Vec<i64>>, Vec<i64>) {
    let n = machine.num_lights as usize;
    let m = machine.button_masks.len();

    let mut a = vec![vec![0i64; m]; n];

    for (j, &mask) in machine.button_masks.iter().enumerate() {
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                a[i][j] = 1;
            }
        }
    }

    let t: Vec<i64> = machine.joltages.iter().map(|&v| v as i64).collect();

    (a, t)
}

#[derive(Clone, Copy, Debug)]
struct Frac {
    num: i64,
    den: i64,
}

impl Frac {
    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0);
        if num == 0 {
            return Frac { num: 0, den: 1 };
        }
        let mut num = num;
        let mut den = den;
        if den < 0 {
            num = -num;
            den = -den;
        }
        let g = gcd(num.abs(), den);
        Frac {
            num: num / g,
            den: den / g,
        }
    }

    fn from_i64(n: i64) -> Self {
        Frac { num: n, den: 1 }
    }

    fn zero() -> Self {
        Frac { num: 0, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn is_integer(&self) -> bool {
        self.den == 1
    }

    fn to_i64(&self) -> i64 {
        debug_assert!(self.is_integer());
        self.num
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

use std::ops::{Add, Sub, Mul, Div};

impl Add for Frac {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Frac::new(self.num * other.den + other.num * self.den, self.den * other.den)
    }
}

impl Sub for Frac {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Frac::new(self.num * other.den - other.num * self.den, self.den * other.den)
    }
}

impl Mul for Frac {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Frac::new(self.num * other.num, self.den * other.den)
    }
}

impl Div for Frac {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        assert!(!other.is_zero());
        Frac::new(self.num * other.den, self.den * other.num)
    }
}

fn solve_square_system(b_matrix: &Vec<Vec<i64>>, b: &Vec<i64>) -> Option<Vec<i64>> {
    let n = b_matrix.len();
    assert_eq!(b.len(), n);

    // augmented [B | b]
    let mut mat = vec![vec![Frac::zero(); n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = Frac::from_i64(b_matrix[i][j]);
        }
        mat[i][n] = Frac::from_i64(b[i]);
    }

    // Gauss-Jordan to RREF
    for col in 0..n {
        // find pivot
        let mut pivot_row = None;
        for row in col..n {
            if !mat[row][col].is_zero() {
                pivot_row = Some(row);
                break;
            }
        }
        let Some(pr) = pivot_row else {
            return None; // singular
        };

        if pr != col {
            mat.swap(pr, col);
        }

        let pivot_val = mat[col][col];
        for j in col..=n {
            mat[col][j] = mat[col][j] / pivot_val;
        }

        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = mat[row][col];
            if factor.is_zero() {
                continue;
            }
            for j in col..=n {
                mat[row][j] = mat[row][j] - factor * mat[col][j];
            }
        }
    }

    let mut x = vec![0i64; n];
    for i in 0..n {
        let v = mat[i][n];
        if !v.is_integer() {
            return None;
        }
        let xi = v.to_i64();
        if xi < 0 {
            return None;
        }
        x[i] = xi;
    }
    Some(x)
}

fn find_pivots(a_matrix: &Vec<Vec<i64>>) -> (Vec<usize>, Vec<usize>) {
    let n = a_matrix.len();
    let m = a_matrix[0].len();

    // Work on a Frac copy of A
    let mut mat = vec![vec![Frac::zero(); m]; n];
    for i in 0..n {
        for j in 0..m {
            mat[i][j] = Frac::from_i64(a_matrix[i][j]);
        }
    }

    // row_map[r] = which original row is currently at mat[r]
    let mut row_map: Vec<usize> = (0..n).collect();

    let mut pivot_rows = Vec::new();
    let mut pivot_cols = Vec::new();

    let mut row = 0;

    for col in 0..m {
        if row == n {
            break;
        }

        // Find pivot row at or below 'row' with non-zero entry in this column
        let mut pivot_r = None;
        for r in row..n {
            if !mat[r][col].is_zero() {
                pivot_r = Some(r);
                break;
            }
        }
        let Some(pr) = pivot_r else {
            continue; // no pivot in this column
        };

        // Bring pivot row up
        if pr != row {
            mat.swap(pr, row);
            row_map.swap(pr, row);
        }

        // Record pivot in terms of ORIGINAL row/column indices
        pivot_rows.push(row_map[row]);
        pivot_cols.push(col);

        // Normalize pivot row
        let pivot_val = mat[row][col];
        for j in col..m {
            mat[row][j] = mat[row][j] / pivot_val;
        }

        // Eliminate this column from all other rows
        for r in 0..n {
            if r == row {
                continue;
            }
            let factor = mat[r][col];
            if factor.is_zero() {
                continue;
            }
            for j in col..m {
                mat[r][j] = mat[r][j] - factor * mat[row][j];
            }
        }

        row += 1;
    }

    (pivot_rows, pivot_cols)
}


fn dfs_free(
    idx: usize,
    free_cols: &[usize],
    a_cols_r: &Vec<Vec<i64>>,
    rhs_r: &mut Vec<i64>,
    free_press: &mut Vec<i64>,
    presses_free_sum: i64,
    b_matrix: &Vec<Vec<i64>>,
    pivot_cols: &[usize],
    a_full: &Vec<Vec<i64>>,
    target_full: &Vec<i64>,
    best: &mut Option<i64>,
) {
    // prune if already worse
    if let Some(b) = *best {
        if presses_free_sum >= b {
            return;
        }
    }

    if idx == free_cols.len() {
        // No more free vars; solve for basis vars
        if let Some(x_basis) = solve_square_system(b_matrix, rhs_r) {
            let m = a_full[0].len();
            let n = a_full.len();

            // build full x vector
            let mut x_full = vec![0i64; m];

            // pivot columns
            for (k, &col) in pivot_cols.iter().enumerate() {
                x_full[col] = x_basis[k];
            }

            // free columns
            for (k, &col) in free_cols.iter().enumerate() {
                x_full[col] = free_press[k];
            }

            // verify full system A * x == target
            for i in 0..n {
                let mut sum = 0i64;
                for j in 0..m {
                    sum += a_full[i][j] * x_full[j];
                }
                if sum != target_full[i] {
                    return; // invalid
                }
            }

            // sum presses
            let total: i64 = x_full.iter().sum();
            if total < 0 {
                return;
            }

            if let Some(b) = *best {
                if total < b {
                    *best = Some(total);
                }
            } else {
                *best = Some(total);
            }
        }
        return;
    }

    let col_idx = free_cols[idx];
    let col = &a_cols_r[col_idx]; // length r

    // compute bound for this free variable: can't overshoot any rhs component it affects
    let mut bound_opt: Option<i64> = None;
    for i in 0..rhs_r.len() {
        if col[i] == 1 {
            if rhs_r[i] < 0 {
                return; // already impossible
            }
            bound_opt = Some(match bound_opt {
                None => rhs_r[i],
                Some(b) => b.min(rhs_r[i]),
            });
        }
    }

    // if this button doesn't affect any pivot row, pressing it is useless (or harmful)
    // safest: only try k = 0
    let max_k = match bound_opt {
        None => 0,
        Some(b) => b,
    };

    let original_rhs = rhs_r.clone();

    for k in 0..=max_k {
        if k > 0 {
            // subtract this column one more time
            for i in 0..rhs_r.len() {
                rhs_r[i] -= col[i];
            }
        }

        free_press[idx] = k;

        let new_sum = presses_free_sum + k;
        if let Some(b) = *best {
            if new_sum >= b {
                break; // further k only increases sum
            }
        }

        dfs_free(
            idx + 1,
            free_cols,
            a_cols_r,
            rhs_r,
            free_press,
            new_sum,
            b_matrix,
            pivot_cols,
            a_full,
            target_full,
            best,
        );
    }

    *rhs_r = original_rhs;
    free_press[idx] = 0;
}
