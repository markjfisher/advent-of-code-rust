# Advent Of Code in Rust

![rust](https://img.shields.io/badge/language-rust-0b7261?style=flat-square&logo=rust)

My [rust][rust] solutions to [Advent of Code][aoc] puzzles.

I previously used [cargo-aoc][cargo-aoc], but switched to a solution that combines all years instead, as cargo-aoc does not support workspaces.

Edit files (src/aoc{year}/day{day}.rs) to add solutions for new days/years.
Then add references to the new files in `src/lib.rs`, `src/main.rs`, and `tests/test.rs`.

## Running Solutions

```bash
cargo build
cargo run
cargo run aoc2024
cargo run aoc2024::day01
```

To run a release version add `--release` to the above arguments

To run tests:

```bash
cargo test
cargo test aoc2024::day01
```

---

[aoc]: https://adventofcode.com/
[rust]: https://rust-lang.org
[cargo-aoc]: https://github.com/gobanos/cargo-aoc
