//! # Advent of Code solutions in Rust
//!
//! [![badge]][link]
//!
//! [badge]: https://img.shields.io/badge/github-blue?style=for-the-badge&logo=github&labelColor=grey
//! [link]: https://github.com/maneatingape/advent-of-code-rust

// A great thank you to @maneatingape for this startup code and project structure.
// See https://github.com/maneatingape/advent-of-code-rust/blob/main/src/lib.rs

// Portable SIMD API is enabled by "simd" feature.
#![cfg_attr(feature = "simd", allow(unstable_features), feature(portable_simd))]

// Configure rustdoc.
#![doc(html_logo_url = "https://maneatingape.github.io/advent-of-code-rust/logo.png")]

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    ansi, grid, hash, heap, integer, iter, md5, parse, point, thread, bronkerbosch, tomita
);

library!(aoc2015 "AOC 2015"
    day01, day02, day03, day04, day05, day06, day07, day08, day09
);

library!(aoc2016 "AOC 2016"
    day01, day02, day03, day04
);

library!(aoc2017 "AOC 2017"
    day01, day02, day03, day04
);

library!(aoc2024 "AOC 2024"
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25, comp
);

library!(aoc2025 "AOC 2025"
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12
);

library!(vis2024 "Visualisations for AOC 2024"
    day16
);

library!(vis2025 "Visualisations for AOC 2025"
    day04
);
