//! Comprehensive 2 dimensional point implementation. This class is designed to work together
//! with the [`Grid`] class.
//!
//! A common theme in Advent of Code is operations in 2 dimensions. This module provides a
//! [`Point`] struct along with implementations of several of the [`std::ops`] traits to support
//! operator overloading, that allows shorthand expressions such as:
//!
//! ```
//!   # use aoc::util::point::Point;
//!
//!   let a = Point::new(1, 2);
//!   let b = Point::new(3, 4);
//!   let k = 2;
//!
//!   assert_eq!(a + b, Point::new(4, 6));
//!   assert_eq!(a - b, Point::new(-2, -2));
//!   assert_eq!(a * k, Point::new(2, 4));
//! ```
//!
//! Additionally there are [`clockwise`] and [`counter_clockwise`] functions for 90 degree rotations
//! and a [`manhattan`] function for the
//! [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between 2 points.
//!
//! [`clockwise`]: Point::clockwise
//! [`counter_clockwise`]: Point::counter_clockwise
//! [`manhattan`]: Point::manhattan
//! [`Grid`]: crate::util::grid
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const UP_LEFT: Point = Point::new(-1, -1);
pub const UP_RIGHT: Point = Point::new(1, -1);
pub const DOWN_LEFT: Point = Point::new(-1, 1);
pub const DOWN_RIGHT: Point = Point::new(1, 1);
pub const JUST_DIAGONALS: [Point; 4] = [UP_LEFT, UP_RIGHT, DOWN_RIGHT, DOWN_LEFT];
// Left to right and top to bottom.
pub const DIAGONAL: [Point; 8] = [
    UP_LEFT,
    UP,
    UP_RIGHT,
    LEFT,
    RIGHT,
    DOWN_LEFT,
    DOWN,
    DOWN_RIGHT,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }

    #[inline]
    #[must_use]
    pub fn manhattan(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    #[must_use]
    pub fn signum(self, other: Self) -> Self {
        Point::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }

    pub fn adjacent_with_diagonals(&self) -> Vec<Point> {
        DIAGONAL.iter().map(|&p| *self + p).collect()
    }

    pub fn to_index(&self) -> usize {
        match *self {
            UP => 0,
            RIGHT => 1,
            DOWN => 2,
            LEFT => 3,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Point {
    #[inline]
    #[must_use]
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => UP,
            b'v' | b'D' => DOWN,
            b'<' | b'L' => LEFT,
            b'>' | b'R' => RIGHT,
            _ => unreachable!(),
        }
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_with_diagonals() {
        let point = Point::new(0, 0);
        let adjacent = point.adjacent_with_diagonals();
        
        assert_eq!(adjacent.len(), 8);
        assert!(adjacent.contains(&Point::new(-1, -1)));
        assert!(adjacent.contains(&Point::new( 0, -1)));
        assert!(adjacent.contains(&Point::new( 1, -1)));
        assert!(adjacent.contains(&Point::new(-1,  0)));
        assert!(adjacent.contains(&Point::new( 1,  0)));
        assert!(adjacent.contains(&Point::new(-1,  1)));
        assert!(adjacent.contains(&Point::new( 0,  1)));
        assert!(adjacent.contains(&Point::new( 1,  1)));
    }
}