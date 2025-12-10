use pathfinding::prelude::bfs;
use crate::util::frac::Frac;

#[derive(Debug, Clone)]
pub struct Machine {
    pub num_lights: u8,
    pub target: u64,            // bits: 1 = ON, 0 = OFF
    pub button_masks: Vec<u64>, // one mask per button
    pub joltages: Vec<u32>,     // same length as num_lights
}

use regex::Regex;

pub fn parse_machine(line: &str) -> Machine {
    let re = Regex::new(
        r"^\s*\[([.#]+)\]\s*((?:\([\d,]+\)\s*)+)\{([\d,\s]+)\}\s*$"
    ).unwrap();

    let caps = re
        .captures(line)
        .unwrap_or_else(|| panic!("Invalid machine line: {line}"));

    let lights_str = caps.get(1).unwrap().as_str();
    let num_lights = lights_str.len() as u8;

    let mut target: u64 = 0;
    for (i, ch) in lights_str.chars().enumerate() {
        match ch {
            '#' => target |= 1 << i,
            '.' => {}                // off
            _   => panic!("Unexpected char '{ch}' in lights pattern"),
        }
    }

    let buttons_block = caps.get(2).unwrap().as_str();
    let b_re = Regex::new(r"\(([\d,]+)\)").unwrap();

    let mut button_masks = Vec::new();
    for bcap in b_re.captures_iter(buttons_block) {
        let inner = bcap.get(1).unwrap().as_str();
        let mut mask = 0u64;

        for num in inner.split(',') {
            let idx: usize = num.parse().unwrap();
            mask |= 1u64 << idx;
        }
        button_masks.push(mask);
    }

    let jolts_str = caps.get(3).unwrap().as_str();
    let joltages: Vec<u32> = jolts_str
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();

    if joltages.len() != num_lights as usize {
        panic!(
            "Joltage count {} does not match number of lights {}",
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

// use pathfinding::prelude::bfs to search for the shortest path
// This was quite simple, using XOR as a simple state change on button press was nice
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

// Linear algebra solution for part 2, see below for full explanation
pub fn min_joltage_presses(machine: &Machine) -> Option<i64> {
    let (a_full, target_full) = build_matrix(machine);
    let m = a_full[0].len();

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


// -----------------------------------------------------------------------------
// Part 2: Minimum Joltage Presses – Summary of the Algorithm
// -----------------------------------------------------------------------------
//
// Each machine gives:
//   - n lights
//   - m buttons
//   - Each button increments some subset of lights by +1
//   - A target joltage t[i] for each light i
//
// The goal is:
//      Find non-negative integers x[0..m-1] (press counts)
//      with minimal sum(x) such that:
//
//          A * x = t
//
// Where A is an n×m matrix:
//      A[i][j] = 1 if button j affects light i, otherwise 0
//
// This is an Integer Linear System with small dimensions
// and non-negativity constraints.
//
// A naive BFS would explode (targets up to hundreds), so we solve it
// using exact rational linear algebra + a small search over free variables.
//
// -----------------------------------------------------------------------------
// Main idea
// -----------------------------------------------------------------------------
//
// 1) Build A (n×m) and target vector t.
//
// 2) Perform Gaussian elimination on A (over exact rationals) to determine:
//        - pivot columns (basis variables)
//        - free columns   (variables not constrained by rank)
//    This gives us the rank r of A.
//
// 3) Reorder the system conceptually so it looks like:
//
//        B * x_basis  +  A_free * x_free  =  t'
//
//    Where B is an r×r full-rank submatrix corresponding to pivot columns,
//    and x_free are the free variables we can choose.
//
// 4) For any choice of non-negative integer values for x_free,
//    the right-hand side for the pivot equation becomes:
//
//        rhs_r = t' − A_free * x_free
//
//    If rhs_r has any negative entry, this choice cannot work.
//
// 5) Solve the small r×r linear system:
//
//        B * x_basis = rhs_r
//
//    using exact rational Gauss–Jordan.
//    We only accept solutions where:
//        - all x_basis are integers,
//        - all x_basis >= 0.
//
// 6) Combine x_basis and x_free into the full vector x.
//    Verify A * x == t (safety check).
//
// 7) Among all valid integer solutions, take the one with
//        minimal total presses = sum(x[j]).
//
// -----------------------------------------------------------------------------
// Why this is efficient
// -----------------------------------------------------------------------------
//
// - n and m are tiny (<= 8–12), so Gaussian elimination is trivial.
// - Most machines have few free variables (often 0 or 1).
// - Free variables have tight bounds: pressing a button too many times
//   immediately overshoots a target, so the DFS search is tiny.
// - All arithmetic is exact (using a small rational type Frac),
//   so we never lose integer precision.
//
// This method is effectively a specialised Integer Linear Programming solver,
// but tailored for 0/1 matrices and small problem sizes.
// It is fast for all AoC inputs and does not rely on floating point or
// external solvers.
//
// -----------------------------------------------------------------------------

fn build_matrix(machine: &Machine) -> (Vec<Vec<i64>>, Vec<i64>) {
    let n = machine.num_lights as usize;
    let m = machine.button_masks.len();

    let mut a_matrix = vec![vec![0i64; m]; n];

    for (j, &mask) in machine.button_masks.iter().enumerate() {
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                a_matrix[i][j] = 1;
            }
        }
    }

    let t: Vec<i64> = machine.joltages.iter().map(|&v| v as i64).collect();

    (a_matrix, t)
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
