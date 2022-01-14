pub trait Circular<T> {
    fn state(&self) -> T;
    fn inc(&mut self);
    fn is_zero(&self) -> bool;
    fn has_next(&self) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub struct Ring {
    pub size: i32,
    pub offset: i32,
    state: i32,
}

impl Ring {
    fn new(size: i32, offset: i32) -> Ring { Ring { size, offset, state: 0 } }
}

impl Circular<i32> for Ring {
    fn state(&self) -> i32 { self.state + self.offset }
    fn inc(&mut self) { self.state = (1 + self.state) % self.size; }
    fn is_zero(&self) -> bool { self.state == 0 }
    fn has_next(&self) -> bool { self.state != self.size - 1 }
}

impl IntoIterator for Ring {
    type Item = i32;
    type IntoIter = RingIntoIterator;
    fn into_iter(self) -> Self::IntoIter { RingIntoIterator { ring: self, started: false, ended: false } }
}

pub struct RingIntoIterator {
    ring: Ring,
    started: bool,
    ended: bool,
}

impl Iterator for RingIntoIterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.ended { return None; };
        if self.started { self.ring.inc(); } else { self.started = true; }
        if !self.ring.has_next() { self.ended = true; }
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

impl Circular<Vec<Ring>> for IntCombinations {
    fn state(&self) -> Vec<Ring> {
        self.state.clone()
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


impl IntoIterator for IntCombinations {
    type Item = Vec<i32>;
    type IntoIter = IntCombinationsIntoIterator;
    fn into_iter(self) -> Self::IntoIter { IntCombinationsIntoIterator { rings: self, started: false, ended: false } }
}


pub struct IntCombinationsIntoIterator {
    rings: IntCombinations,
    started: bool,
    ended: bool,
}

impl Iterator for IntCombinationsIntoIterator {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Vec<i32>> {
        if self.ended { return None; };
        if self.started { self.rings.inc(); } else { self.started = true; }
        if !self.rings.has_next() { self.ended = true; }
        // convert the Vec<Ring> to Vec<i32> for their current state, must use r.state() to add the offset
        let mut ring_states: Vec<_> = self.rings.state().iter().map(|r| r.state()).collect();
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
    fn can_create_d6() {
        let two_d3 = IntCombinations::new(2, 3, 1);
        let vs: Vec<Vec<_>> = two_d3.into_iter().collect();
        assert_eq!(vs, [[1, 1], [1, 2], [1, 3], [2, 1], [2, 2], [2, 3], [3, 1], [3, 2], [3, 3]]);
    }

    #[test]
    fn can_create_around_space() {
        let around_3d_point = IntCombinations::new(3, 3, -1);
        let vs: Vec<Vec<_>> = around_3d_point.into_iter().collect();
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
}