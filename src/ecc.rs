use std::{convert::TryInto, ops::{Add, Sub, Mul, Div}};

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct FieldElement(pub(crate) usize, pub(crate) usize);

impl FieldElement {
    pub fn new(num: usize, prime: usize) -> Result<Self, String> {
        if num >= prime {
            Err(format!("Num {} not in field range O to {}", num, prime - 1))
        } else {
            Ok(Self(num, prime))
        }
    }

    pub fn num(&self) -> usize {
        self.0
    }

    pub fn prime(&self) -> usize {
        self.1
    }

    pub fn pow(self, exp: u32) -> Self {
        let p = self.1;
        let base = self.0 as u128;
        let a = base.pow(exp).rem_euclid(p as u128);
        FieldElement(a.try_into().unwrap(), p)
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.1 != other.1 {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.1, self.1,
                self.1, other.1,
            );
        }
        Self((self.0 + other.0).rem_euclid(self.1), self.1)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.1 != other.1 {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.1, self.1,
                self.1, other.1,
            );
        }
        let diff = (self.0 - other.0) as isize;
        Self (
            diff.rem_euclid(self.1 as isize) as usize,
            self.1,
        )
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.1 != other.1 {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.1, self.1,
                self.1, other.1,
            );
        }
        Self(
            (self.0 * other.0).rem_euclid(self.1),
            self.1,
        )
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.1 != other.1 {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.1, self.1,
                self.1, other.1,
            );
        }
        let num = (self.0 * other.0.pow(other.1 as u32 - 2)) % other.1;
        Self(num, self.1)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Point {
    pub(crate) x: isize,
    pub(crate) y: isize,
    pub(crate) a: isize,
    pub(crate) b: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, a: isize, b: isize) -> Result<Self, String> {
        if y.pow(2) != x.pow(3) + a * x + b {
            Err(format!("({}, {}) is not on the curve", x, y))
        } else {
            Ok(Self { a, b, x, y })
        }
    }
}