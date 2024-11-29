use aoc::util::ansi::*;
use aoc::util::parse::*;
use aoc::*;
use std::env::args;
use std::fs::read_to_string;
use std::iter::empty;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main() {
    // Parse command line options
    let (year, day) = match args().nth(1) {
        Some(arg) => {
            let str = arg.as_str();
            let mut iter = str.iter_unsigned();
            (iter.next(), iter.next())
        }
        None => (None, None),
    };

    // Filter solutions
    let solutions = empty()
        .chain(aoc2015())
        // .chain(aoc2021())
        // .chain(aoc2023())
        .filter(|solution| year.is_none_or(|y: u32| y == solution.year))
        .filter(|solution| day.is_none_or(|d: u32| d == solution.day));

    // Pretty print output and timing for each solution
    let mut solved = 0;
    let mut duration = Duration::ZERO;

    for Solution { year, day, path, wrapper } in solutions {
        if let Ok(data) = read_to_string(&path) {
            let instant = Instant::now();
            let (part1, part2) = wrapper(data);
            let elapsed = instant.elapsed();

            solved += 1;
            duration += elapsed;

            println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
            println!("    Part 1: {part1}");
            println!("    Part 2: {part2}");
            println!("    Elapsed: {} μs", elapsed.as_micros());
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!("    Place input file in {BOLD}{WHITE}{}{RESET}", path.display());
        }
    }

    // Print totals
    println!("{BOLD}{RED}Solved: {solved}{RESET}");
    println!("{BOLD}{GREEN}Duration: {} ms{RESET}", duration.as_millis());
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (String, String),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let year = stringify!($year);
        let day = stringify!($day);
        let path = Path::new("input").join(year).join(day).with_extension("txt");

        let wrapper = |data: String| {
            use $year::$day::*;

            let input = parse(&data);
            let part1 = part1(&input);
            let part2 = part2(&input);

            (part1.to_string(), part2.to_string())
        };

        Solution { year: year.unsigned(), day: day.unsigned(), path, wrapper }
    }};
}

fn aoc2015() -> Vec<Solution> {
    vec![
        solution!(aoc2015, day01),
        solution!(aoc2015, day02),
        solution!(aoc2015, day03),
        solution!(aoc2015, day04),
        // solution!(aoc2015, day05),
        // solution!(aoc2015, day06),
        // solution!(aoc2015, day07),
        // solution!(aoc2015, day08),
        // solution!(aoc2015, day09),
        // solution!(aoc2015, day10),
        // solution!(aoc2015, day11),
        // solution!(aoc2015, day12),
        // solution!(aoc2015, day13),
        // solution!(aoc2015, day14),
        // solution!(aoc2015, day15),
    ]
}

// fn aoc2021() -> Vec<Solution> {
//     vec![
//         // solution!(aoc2021, day01),
//         // solution!(aoc2021, day02),
//         // solution!(aoc2021, day03),
//     ]
// }

// fn aoc2023() -> Vec<Solution> {
//     vec![
//         // solution!(aoc2023, day01),
//     ]
// }