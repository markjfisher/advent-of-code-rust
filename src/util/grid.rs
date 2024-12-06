//! Fast 2 dimensional Grid backed by a single `vec`. This module is designed to work with [`Point`].
//!
//! The traits [`Index`] and [`IndexMut`] are implemented for [`Point`] to allow usage like:
//!
//! ```
//!   # use aoc::util::grid::Grid;
//!   # use aoc::util::point::Point;
//!
//!   let mut grid = Grid::parse("1");
//!   let point = Point::new(0, 0);
//!
//!   let foo = grid[point];
//!   assert_eq!(foo, b'1');
//!
//!   grid[point] = foo + 1;
//!   assert_eq!(grid[point], b'2');
//! ```
//!
//! A convenience [`parse`] method creates a `Grid` directly from a 2 dimenionsal set of
//! ASCII characters, a common occurence in Advent of Code inputs. The [`default_copy`] function
//! creates a grid of the same size, that can be used for in BFS algorithms for tracking visited
//! location or for tracking cost in Djikstra.
//!
//! [`Point`]: crate::util::point
//! [`parse`]: Grid::parse
//! [`default_copy`]: Grid::default_copy
use crate::util::point::*;
use crate::util::hash::*;
use std::ops::{Index, IndexMut};


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid { width, height, bytes }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn find(&self, needle: T) -> Option<Point> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
        Grid { width, height, bytes: vec![value; (width * height) as usize] }
    }
}

impl<T> Grid<T> {
    pub fn default_copy<U: Default + Copy>(&self) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![U::default(); (self.width * self.height) as usize],
        }
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height).flat_map(move |y| 
            (0..self.width).map(move |x| Point::new(x, y))
        )
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}

#[derive(Clone)]
pub struct SparseGrid<T> {
    pub width: i32,
    pub height: i32,
    pub data: FastMap<Point, T>,
}

impl SparseGrid<u8> {
    pub fn parse<F>(input: &str, predicate: F) -> Self 
    where 
        F: Fn(u8) -> bool,
    {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        
        let mut data = FastMap::new();
        for (y, row) in raw.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if predicate(value) {
                    data.insert(Point::new(x as i32, y as i32), value);
                }
            }
        }

        SparseGrid { width, height, data }
    }

    pub fn parse_all(input: &str) -> Self {
        Self::parse(input, |_| true)
    }
}

impl<T> SparseGrid<T> {
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height).flat_map(move |y| 
            (0..self.width).map(move |x| Point::new(x, y))
        )
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T> Index<Point> for SparseGrid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.data[&point]
    }
}

impl<T> IndexMut<Point> for SparseGrid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        self.data.get_mut(&point).unwrap()
    }
}

impl<T: PartialEq> SparseGrid<T> {
    pub fn find(&self, needle: &T) -> Option<Point> {
        self.data.iter()
            .find(|(_, value)| *value == needle)
            .map(|(&point, _)| point)
    }

    pub fn find_all<'a>(&'a self, needle: &'a T) -> impl Iterator<Item = Point> + 'a {
        self.data.iter()
            .filter(move |(_, value)| *value == needle)
            .map(move |(&point, _)| point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_points_iterator() {
        let grid = Grid::parse("12\n34");
        let points: Vec<Point> = grid.points().collect();
        
        assert_eq!(points, vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ]);
        
        // Verify we can access all values through these points
        assert_eq!(grid[points[0]], b'1');
        assert_eq!(grid[points[1]], b'2');
        assert_eq!(grid[points[2]], b'3');
        assert_eq!(grid[points[3]], b'4');
    }
}
