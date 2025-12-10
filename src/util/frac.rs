
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct Frac {
    num: i64,
    den: i64,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

impl Frac {
    pub fn new(num: i64, den: i64) -> Self {
        assert!(den != 0);
        if num == 0 {
            return Frac { num: 0, den: 1 };
        }
        let mut num = num;
        let mut den = den;
        if den < 0 {
            num = -num;
            den = -den;
        }
        let g = gcd(num.abs(), den);
        Frac {
            num: num / g,
            den: den / g,
        }
    }

    pub fn from_i64(n: i64) -> Self {
        Frac { num: n, den: 1 }
    }

    pub fn zero() -> Self {
        Frac { num: 0, den: 1 }
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub fn is_integer(&self) -> bool {
        self.den == 1
    }

    pub fn to_i64(&self) -> i64 {
        debug_assert!(self.is_integer());
        self.num
    }
}


impl Add for Frac {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Frac::new(self.num * other.den + other.num * self.den, self.den * other.den)
    }
}

impl Sub for Frac {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Frac::new(self.num * other.den - other.num * self.den, self.den * other.den)
    }
}

impl Mul for Frac {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Frac::new(self.num * other.num, self.den * other.den)
    }
}

impl Div for Frac {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        assert!(!other.is_zero());
        Frac::new(self.num * other.den, self.den * other.num)
    }
}