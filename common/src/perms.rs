/// A trait for types that represent circular/cyclic sequences with a finite size.
/// The sequence maintains a current state and can be incremented through its values.
/// 
/// Type Parameters:
/// - 'a: lifetime parameter for references
/// - T: the type of value produced by the circular sequence
pub trait Circular<'a, T: 'a> {
    /// Returns the current state/value in the sequence
    /// For Ring, this returns the current position plus any offset
    fn state(&'a self) -> T;

    /// Moves to the next value in the sequence
    /// When reaching the end, wraps back to the start (modulo behavior)
    fn inc(&'a mut self);

    /// Checks if the current state is at the starting position (0)
    fn is_zero(&self) -> bool;

    /// Checks if there are more values before wrapping back to start
    /// Returns false when at the last value in the sequence
    fn has_next(&self) -> bool;
}

/// A Ring represents a circular counter that cycles through values 0 to (size-1)
/// with an optional offset applied to the output values.
/// 
/// Fields:
/// - size: The number of distinct values in the ring (e.g., size 3 gives values 0,1,2)
/// - offset: A value added to the internal state when getting the current value
/// - state: The current position in the ring (internal counter before offset)
///
/// Example:
/// ```
/// let ring = Ring { size: 3, offset: 1, state: 0 };
/// // ring.state() would return 1 (0 + offset)
/// // After inc(): 2 (1 + offset)
/// // After inc(): 3 (2 + offset)
/// // After inc(): 1 (0 + offset) // wraps around
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Ring {
    pub size: i32,
    pub offset: i32,
    state: i32,
}

impl Ring {
    fn new(size: i32, offset: i32) -> Ring { Ring { size, offset, state: 0 } }
}

impl<'a> Circular<'a, i32> for Ring {
    fn state(&self) -> i32 { self.state + self.offset }
    fn inc(&mut self) { self.state = (1 + self.state) % self.size; }
    fn is_zero(&self) -> bool { self.state == 0 }
    fn has_next(&self) -> bool { self.state != self.size - 1 }
}

/// Iterator implementation for Ring that produces a sequence of values from 0 to size-1 (plus offset)
/// 
/// Fields:
/// - ring: The Ring being iterated over
/// - started: Tracks if we've started iteration (to handle first value correctly)
/// - ended: Tracks if we've completed the sequence
pub struct RingIntoIterator {
    ring: Ring,
    started: bool,
    ended: bool,
}

// Implements conversion from Ring into an iterator
impl IntoIterator for Ring {
    type Item = i32;
    type IntoIter = RingIntoIterator;
    
    /// Creates a new iterator from a Ring
    /// The iterator will yield all values from the Ring exactly once
    fn into_iter(self) -> Self::IntoIter { 
        RingIntoIterator { 
            ring: self, 
            started: false, 
            ended: false 
        } 
    }
}

// The actual iterator implementation for Ring
impl Iterator for RingIntoIterator {
    type Item = i32;
    
    /// Produces the next value in the sequence
    /// - First call returns the initial state
    /// - Subsequent calls increment the ring before returning
    /// - Returns None when sequence is complete
    fn next(&mut self) -> Option<i32> {
        if self.ended { 
            return None; 
        }
        
        // If we've already started, increment before returning
        if self.started { 
            self.ring.inc(); 
        } else { 
            self.started = true; 
        }
        
        // Check if this is the last value
        if !self.ring.has_next() { 
            self.ended = true; 
        }
        
        Option::Some(self.ring.state())
    }
}

#[derive(Debug)]
pub struct IntCombinations {
    pub size: i32,
    pub ring_size: i32,
    state: Vec<Ring>
}

impl IntCombinations {
    pub fn new(size: i32, ring_size: i32, offset: i32) -> IntCombinations {
        let rings: Vec<_> = (0..size).into_iter().map(|_| Ring::new(ring_size, offset)).collect();
        IntCombinations { size, ring_size, state: rings }
    }
}

impl<'a> Circular<'a, Vec<&'a Ring>> for IntCombinations {
    fn state(&self) -> Vec<&Ring> {
        let rings: Vec<_> = self.state.iter().collect();
        rings
    }
    fn inc(&mut self) {
        for r in self.state.iter_mut() {
            r.inc();
            if !r.is_zero() { break; }
        }
    }
    fn is_zero(&self) -> bool {
        self.state.iter().all(|r| r.is_zero())
    }
    fn has_next(&self) -> bool {
        self.state.iter().any(|r| r.has_next())
    }
}

/// Iterator for IntCombinations that produces sequences of integers representing
/// all possible combinations of the contained rings
/// 
/// Fields:
/// - rings: The IntCombinations being iterated
/// - started: Tracks if iteration has begun
/// - ended: Tracks if all combinations have been produced
pub struct IntCombinationsIntoIterator {
    rings: IntCombinations,
    started: bool,
    ended: bool,
}

// Implements conversion from IntCombinations into an iterator
impl IntoIterator for IntCombinations {
    type Item = Vec<i32>;
    type IntoIter = IntCombinationsIntoIterator;
    
    /// Creates a new iterator that will produce all possible combinations
    fn into_iter(self) -> Self::IntoIter { 
        IntCombinationsIntoIterator { 
            rings: self, 
            started: false, 
            ended: false 
        } 
    }
}

// The actual iterator implementation for IntCombinations
impl Iterator for IntCombinationsIntoIterator {
    type Item = Vec<i32>;
    
    /// Produces the next combination in the sequence
    /// - First call returns initial states of all rings
    /// - Subsequent calls increment the rings (with carry-over)
    /// - Returns None when all combinations are exhausted
    fn next(&mut self) -> Option<Vec<i32>> {
        if self.ended { 
            return None; 
        }
        
        // If we've already started, increment before returning
        if self.started { 
            self.rings.inc(); 
        } else { 
            self.started = true; 
        }
        
        // Check if this is the last combination
        if !self.rings.has_next() { 
            self.ended = true; 
        }
        
        // Convert the rings' states to a vector of integers
        let mut ring_states: Vec<_> = self.rings.state()
            .iter()
            .map(|r| r.state())
            .collect();
        ring_states.reverse();
        
        Option::Some(ring_states)
    }
}

pub struct AroundSpace;

impl AroundSpace {
    pub fn new(size: i32) -> IntCombinations {
        IntCombinations::new(size, 3, -1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_ring() {
        let r1 = Ring::new(3, 0);
        assert_eq!(r1.state, 0);
        assert_eq!(r1.size, 3);
        assert_eq!(r1.offset, 0);
    }

    #[test]
    fn can_iterate_ring() {
        let r1 = Ring::new(3, 0);
        let vs: Vec<_> = r1.into_iter().collect();
        assert_eq!(vs, [0, 1, 2]);
    }

    #[test]
    fn can_create_d3() {
        let two_d3 = IntCombinations::new(2, 3, 1);
        let vs: Vec<Vec<_>> = two_d3.into_iter().collect();
        assert_eq!(vs, [[1, 1], [1, 2], [1, 3], [2, 1], [2, 2], [2, 3], [3, 1], [3, 2], [3, 3]]);
    }

    #[test]
    fn can_create_around_space() {
        // AroundSpace generates coordinates in 3D space centered at origin with radius 1
        let around_space = AroundSpace::new(3);
        
        // Collect all neighboring coordinates into a vector
        let vs: Vec<Vec<_>> = around_space.into_iter().collect();
        
        // Verify we get all 27 points in 3D space from (-1,-1,-1) to (1,1,1)
        assert_eq!(vs, [
            [-1, -1, -1], [-1, -1, 0], [-1, -1, 1],
            [-1, 0, -1], [-1, 0, 0], [-1, 0, 1],
            [-1, 1, -1], [-1, 1, 0], [-1, 1, 1],

            [0, -1, -1], [0, -1, 0], [0, -1, 1],
            [0, 0, -1], [0, 0, 0], [0, 0, 1],
            [0, 1, -1], [0, 1, 0], [0, 1, 1],

            [1, -1, -1], [1, -1, 0], [1, -1, 1],
            [1, 0, -1], [1, 0, 0], [1, 0, 1],
            [1, 1, -1], [1, 1, 0], [1, 1, 1]]);
    }

    #[test]
    fn can_create_around_6d() {
        // mind blowing... the struct has no data, is just a marker to add functions to, but as we're just using it to get iterator, who cares.
        let vs: Vec<Vec<_>> = AroundSpace::new(6).into_iter().collect();
        assert_eq!(vs.len(), 729);
    }

    #[test]
    fn test_int_combinations_iterator_basics() {
        let combinations = IntCombinations::new(2, 2, 0);
        let mut iterator = combinations.into_iter();
        
        // Test first value before any iteration
        assert_eq!(iterator.next(), Some(vec![0, 0]));
        
        // Test subsequent values
        assert_eq!(iterator.next(), Some(vec![0, 1]));
        assert_eq!(iterator.next(), Some(vec![1, 0]));
        assert_eq!(iterator.next(), Some(vec![1, 1]));
        
        // Test that iterator is exhausted
        assert_eq!(iterator.next(), None);
        assert_eq!(iterator.next(), None); // Should still be None when called again
    }

    #[test]
    fn test_int_combinations_iterator_with_offset() {
        let combinations = IntCombinations::new(2, 2, 1); // offset of 1
        let iterator = combinations.into_iter();
        let all_values: Vec<Vec<i32>> = iterator.collect();
        
        assert_eq!(all_values, vec![
            vec![1, 1],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ]);
    }

    #[test]
    fn test_int_combinations_iterator_empty() {
        let combinations = IntCombinations::new(0, 2, 0);
        let mut iterator = combinations.into_iter();
        
        // Should return empty vector since size is 0
        assert_eq!(iterator.next(), Some(vec![]));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_int_combinations_iterator_single_ring() {
        let combinations = IntCombinations::new(1, 3, 0);
        let all_values: Vec<Vec<i32>> = combinations.into_iter().collect();
        
        assert_eq!(all_values, vec![
            vec![0],
            vec![1],
            vec![2],
        ]);
    }
}