use pathfinding::prelude::bfs;
// use crate::util::frac::Frac;
use z3::{ast::Int, Optimize, SatResult};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Machine {
    pub num_lights: u8,
    pub target: u64,            // bits: 1 = ON, 0 = OFF
    pub button_masks: Vec<u64>, // one mask per button
    pub joltages: Vec<u32>,     // same length as num_lights
}

pub fn part1(input: &(u32, u32)) -> u32 {
    input.0
}

pub fn part2(input: &(u32, u32)) -> u32 {
    input.1
}

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
    let part2 = machines.iter().map(min_joltage_presses_z3).sum::<Option<i64>>().unwrap();
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


// Classic case of using Z3 for this, although I added this later after a linear algebra solution
// so that I'd have a z3 solution for future puzzles.

// Use Z3 to solve the system of equations
pub fn min_joltage_presses_z3(machine: &Machine) -> Option<i64> {
    let n = machine.num_lights as usize;
    let m = machine.button_masks.len();

    // Build A (n × m) and t (targets)
    let mut a_matrix = vec![vec![0i64; m]; n];
    for (j, &mask) in machine.button_masks.iter().enumerate() {
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                a_matrix[i][j] = 1;
            }
        }
    }

    let t: Vec<i64> = machine.joltages.iter().map(|&v| v as i64).collect();

    let opt = Optimize::new();

    // Int vars x_j for each button
    let x_vars: Vec<Int> = (0..m)
        .map(|j| Int::new_const(format!("x_{j}")))
        .collect();

    // x_j >= 0
    for x in &x_vars {
        opt.assert(&x.ge(Int::from_i64(0)));
    }

    // For each light i: sum_j A[i][j] * x_j == t[i]
    for i in 0..n {
        let mut terms: Vec<Int> = Vec::new();
        for j in 0..m {
            if a_matrix[i][j] != 0 {
                let coeff = Int::from_i64(a_matrix[i][j]);
                terms.push(coeff * &x_vars[j]);
            }
        }

        let lhs = if terms.is_empty() {
            Int::from_i64(0)
        } else {
            // fold instead of Int::add to avoid fiddly lifetimes
            let mut sum = terms[0].clone();
            for t2 in terms.iter().skip(1) {
                sum = &sum + t2;
            }
            sum
        };

        let rhs = Int::from_i64(t[i]);
        opt.assert(&lhs.eq(&rhs));
    }

    // Objective: minimize total presses = sum_j x_j
    let mut total = Int::from_i64(0);
    for x in &x_vars {
        total = &total + x;
    }
    opt.minimize(&total);

    // Solve
    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().expect("expected model from Optimize");
            let mut total_val = 0i64;

            for x in &x_vars {
                if let Some(v) = model.eval(x, true) {
                    let as_i64 = v
                        .as_i64()
                        .expect("Z3 returned non-integer for Int var");
                    if as_i64 < 0 {
                        // shouldn't happen due to x >= 0, but be safe
                        return None;
                    }
                    total_val += as_i64;
                } else {
                    // model_completion=true should prevent this, but bail if it happens
                    return None;
                }
            }

            Some(total_val)
        }
        _ => None, // Unsat or Unknown
    }
}
