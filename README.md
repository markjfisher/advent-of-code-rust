# Advent Of Code in Rust

My rust solutions to [Advent of Code](https://adventofcode.com/) puzzles.

This project uses [aoc_runner](https://github.com/gobanos/aoc-runner) and [cargo_aoc](https://github.com/gobanos/cargo-aoc)

## Running Solutions

To run, cd into the appropriate year and type

    cargo aoc

To benchmark code run:

    cargo aoc bench

To run a release version (much quicker):

    cargo build --release
    cargo run --release

## New days

cargo_aoc can be used to get your input data, you need to set your aoc web session id with

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

Other examples are in the cargo_aoc docs, e.g. parsing lines of "{L}x{W}x{H}":

```rust
pub struct Gift {
    l: u32,
    w: u32,
    h: u32
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
            (
                gift.next().unwrap(),
                gift.next().unwrap(),
                gift.next().unwrap(),
            )
        }).collect()
}
```
