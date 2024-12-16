use crate::util::grid::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn viz(input: &Grid<u8>) {
    println!("viz: w: {}, h: {}", input.width, input.height);
}
