use anyhow::{anyhow, Result};
use num::{bigint::Sign, BigInt, BigUint, FromPrimitive, ToPrimitive};
use std::convert::TryInto;
use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::process::Output;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Point<T> {
    pub x: Option<T>,
    pub y: Option<T>,
    pub a: T,
    pub b: T,
}

impl Point<BigInt> {
    pub fn is_on_curve(x: BigInt, y: BigInt, a: BigInt, b: BigInt) -> bool {
        y.pow(2) == x.pow(3) + a * x + b
    }

    pub fn identity(a: BigInt, b: BigInt) -> Self {
        Self {
            x: None,
            y: None,
            a,
            b,
        }
    }

    pub fn new(x: Option<BigInt>, y: Option<BigInt>, a: BigInt, b: BigInt) -> Option<Self> {
        match (x.clone(), y.clone()) {
            (None, None) => {
                println!("Point at Infinity");
                None
            }
            (Some(x), Some(y)) if Self::is_on_curve(x.clone(), y.clone(), a.clone(), b.clone()) => {
                Some(Self {
                    x: Some(x),
                    y: Some(y),
                    a,
                    b,
                })
            }
            (Some(x), Some(y)) if !Self::is_on_curve(x.clone(), y.clone(), a, b) => {
                panic!("({}, {}) is not on the curve", x, y)
            }
            (x, y) => panic!("({:?}, {:?}) is not on the curve", x, y),
        }
    }

    fn find_slope(p1: Point<BigInt>, p2: Point<BigInt>) -> Option<isize> {
        match (p1.x, p1.y, p2.x, p2.y) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                let d = x2 - x1;
                if d > BigInt::from(0 as u32) {
                    let s = (y2 - y1) / d;
                    Some(s.to_isize().unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Add for Point<BigInt> {
    type Output = Self;
    fn add(self, other: Point<BigInt>) -> Self {
        if self.a != other.a && self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        if self.x == other.x && self.y != other.y {
            return Self::identity(self.a, self.b);
        }

        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }
        other
        // match Self::find_slope(self.clone(), other.clone()) {
        //     None => {
        //         Self {
        //             x: None,
        //             y: None,
        //             a: self.a,
        //             b: self.b,
        //         }
        //     }
        //     Some(s) => {
        //         let big_s = BigInt::from_isize(s).unwrap();
        //         let x3 = big_s.pow(2)
        //             - self.x.clone().unwrap()
        //             - other.x.unwrap();
        //         let x_ = self.x.unwrap();
        //         let y_ = self.y.unwrap();
        //         let y3 = s * (x_ - x3.clone()) - y_;
        //         let Point{a, b, ..} = self;
        //         Point {
        //             x: Some(x3.try_into().unwrap()),
        //             y: Some(y3.try_into().unwrap()),
        //             a,
        //             b,
        //         }
        //     }
        // }
    }
}

/*
impl Add for Point<BigUint> {
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
*/
