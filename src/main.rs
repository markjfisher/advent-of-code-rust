use aoc::util::ansi::*;
use aoc::util::parse::*;
use aoc::*;
use std::env::args;
use std::fs::read_to_string;
use std::iter::empty;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use color_eyre::{eyre::Context, Result};

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

    // first check for visualisations
    if args().nth(1).map_or(false, |arg| arg.starts_with("vis")) {
        do_vis(year, day);
    } else {
        do_aoc(year, day);
    }
}

fn do_aoc(year: Option<u32>, day: Option<u32>) {
    // Filter solutions
    let solutions = empty()
        .chain(aoc2015())
        .chain(aoc2016())
        .chain(aoc2017())
        // .chain(aoc2021())
        // .chain(aoc2023())
        .chain(aoc2024())
        .chain(aoc2025())
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

            solved += 2;
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

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
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
            },)*]
        }
    }
}

run!(aoc2015
    day01, day02, day03, day04, day05, day06, day07, day08, day09
);

run!(aoc2016
    day01, day02, day03, day04
);

run!(aoc2017
    day01, day02, day03, day04
);

run!(aoc2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

run!(aoc2025
    day01
);

struct Visualisation {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> Result<()>,
}

fn do_vis(year: Option<u32>, day: Option<u32>) {
    let visualisations = empty()
        .chain(vis2024())
        .filter(|visualisation| year.is_none_or(|y: u32| y == visualisation.year))
        .filter(|visualisation| day.is_none_or(|d: u32| d == visualisation.day));

    for Visualisation { year, day, path, wrapper } in visualisations {
        if let Ok(data) = read_to_string(&path) {
            // just run the visualisation via the wrapper
            let _result = wrapper(data).context("msg");
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!("    Place input file in {BOLD}{WHITE}{}{RESET}", path.display());
        }
    }
}

macro_rules! viz {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Visualisation> {
            vec![$({
                let year = stringify!($year);
                let day = stringify!($day);
                let path = Path::new("input").join(year).join(day).with_extension("txt");

                let wrapper = |data: String| {
                    use $year::$day::*;

                    let input = parse(&data);
                    viz(input)
                };

                Visualisation { year: year.unsigned(), day: day.unsigned(), path, wrapper }
            },)*]
        }
    }
}

viz!(vis2024
    day16
);
