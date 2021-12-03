# Advent Of Code in Rust

![rust](https://img.shields.io/badge/language-rust-0b7261?style=flat-square&logo=rust)

My rust solutions to [Advent of Code][aoc] puzzles.

This project uses [`aoc-runner`][aoc-runner] and [`cargo-aoc`][cargo-aoc]

## Running Solutions

To run, cd into the appropriate year and type

    cargo aoc

To benchmark code run:

    cargo aoc bench

To run a release version (much quicker):

    cargo build --release
    cargo run --release

To run tests:

    cargo test

## New days

cargo-aoc can be used to get your input data, you need to set your aoc web session id with

    cargo aoc credentials -s {token}  # get this from Application tab in chrome tools for aoc website

then you can just run

    cargo aoc input [ -d {day} -y {year} ] # defaults to today's date

## Generators

To parse the input quickly, use `#[aoc_generator(dayX}]` and implement a function that will convert the
data input file to types required. e.g. for day 01, a list of strings that need to be converted to integers:

```rust
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    common::input_vec(input)
}
```

Other examples are in the cargo_aoc docs and in the worked solutions, e.g. [src/day_03.rs](src/day_03.rs)

---

[aoc]: https://adventofcode.com/
[rust]: https://rust-lang.org
[cargo-aoc]: https://github.com/gobanos/cargo-aoc
[aoc-runner]: https://github.com/gobanos/aoc-runner