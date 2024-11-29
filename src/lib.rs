//! # Advent of Code solutions in Rust
//!
//! [![badge]][link]
//!
//! [badge]: https://img.shields.io/badge/github-blue?style=for-the-badge&logo=github&labelColor=grey
//! [link]: https://github.com/maneatingape/advent-of-code-rust

// This was copied from https://github.com/maneatingape/advent-of-code-rust/blob/main/src/lib.rs

// Configure rustdoc.
#![doc(html_logo_url = "https://maneatingape.github.io/advent-of-code-rust/logo.png")]

/// # Utility modules to handle common recurring Advent of Code patterns.
pub mod util {
    pub mod ansi;
    // pub mod bitset;
    pub mod grid;
    pub mod hash;
    // pub mod heap;
    pub mod integer;
    pub mod iter;
    // pub mod math;
    // pub mod md5;
    pub mod parse;
    pub mod point;
    // pub mod slice;
    // pub mod thread;
}

/// # Help Santa by solving puzzles to fix the weather machine's snow function.
pub mod aoc2015 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    // pub mod day_04;
    // pub mod day_05;
    // pub mod day_06;
    // pub mod day_07;
    // pub mod day_08;
    // pub mod day_09;
    // pub mod day_10;
    // pub mod day_11;
    // pub mod day_12;
    // pub mod day_13;
    // pub mod day_14;
    // pub mod day_15;
}

// # Retrieve the keys to Santa's sleigh with an underwater submarine adventure.
// pub mod aoc2021 {
//     pub mod day_01;
//     pub mod day_02;
//     pub mod day_03;
// }

// # Restore global snow production.
// pub mod aoc2023 {
//     pub mod day_01;
// }
