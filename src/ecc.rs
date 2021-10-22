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

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Point {
    pub(crate) x: Option<isize>,
    pub(crate) y: Option<isize>,
    pub(crate) a: isize,
    pub(crate) b: isize,
}

fn find_slope(p1: Point, p2: Point) -> Option<isize> {
    match (p1.x, p1.y, p2.x, p2.y) {
        (Some(x1), Some(y1), Some(x2), Some(y2)) =>
            Some((y2-y1)/(x2-x1)),
        _ => None,
    }
}

fn find_sum_of_points(p1: Point, p2: Point) -> Option<Point> {
    match find_slope(p1, p2) {
        None => None,
        Some(s) => {
            let x3 = s.pow(2) - p1.x.unwrap() - p2.x.unwrap();
            let y3 = s * (p1.x.unwrap() - x3) - p1.y.unwrap();
            let Point{a, b, ..} = p1;
            Some(Point {
                x: Some(x3),
                y: Some(y3),
                a,
                b,
            })
        }
    }
}

impl Point {
    pub fn new(x: Option<isize>, y: Option<isize>, a: isize, b: isize) -> Result<Self, String> {
        let points_on_curve = |x: isize, y: isize| {
            y.pow(2) == x.pow(3) + a * x + b
        };

        match (x, y) {
            (None, None) => Ok(Point { x, y, a, b }),
            (Some(x),Some(y)) if points_on_curve(x, y) =>
                Ok(Self {
                    x: Some(x),
                    y: Some(y),
                    a,
                    b,
                }),
            (Some(x), Some(y)) if !points_on_curve(x, y) =>
                Err(format!("({}, {}) is not on the curve", x, y)),
            (x, y) =>
                Err(format!("({:?}, {:?}) is not on the curve", x, y)),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.a != other.a && self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        match (self.x, other.x) {
            (None, _) => other,
            (_, None) => self,
            _ => {
                match find_sum_of_points(self, other) {
                    None => panic!("Points {:?}, {:?} are not on the same curve", self, other),
                    Some(p) => p,
                }
            }
        }
    }
}