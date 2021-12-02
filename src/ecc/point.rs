use num::traits::pow::Pow;
use num::BigInt;
use num::Zero;
use std::cmp::Ord;
use std::marker::Copy;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Point<T: Add + Sub + Mul + Div + Eq + Ord> {
    Infinity(T, T),
    OnCurve(T, T, T, T),
    OffCurve,
}

impl Point<BigInt> {
    pub fn is_on_curve(x: &BigInt, y: &BigInt, a: &BigInt, b: &BigInt) -> bool {
        y.pow(2_u32) == x.pow(3_u32) + a * x + b
    }

    pub fn identity(a: BigInt, b: BigInt) -> Self {
        Self::Infinity(a, b)
    }

    pub fn new(x: BigInt, y: BigInt, a: BigInt, b: BigInt) -> Self {
        if Self::is_on_curve(&x, &y, &a, &b) {
            Self::OnCurve(x, y, a, b)
        } else {
            Self::OffCurve
        }
    }

    fn find_slope(p1: Point<BigInt>, p2: Point<BigInt>) -> Option<BigInt> {
        match (p1, p2) {
            (Self::Infinity(_, _), _) | (_, Self::Infinity(_, _)) => Some(BigInt::zero()),
            (Self::OnCurve(x1, y1, ..), Self::OnCurve(x2, y2, ..)) => {
                let x = x2 - x1;
                if x > BigInt::from(0) {
                    Some((y2 - y1) / x)
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
    fn add(self, other: Self) -> Self::Output {
        if self == Self::OffCurve || other == Self::OffCurve {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        match (self.clone(), other.clone()) {
            (Self::Infinity(_, _), other) => other,
            (me, Self::Infinity(_, _)) => me,
            (Self::OnCurve(x1, y1, a, b), Self::OnCurve(x2, y2, _, _)) => {
                // Handle points on a vertical line.
                if x1 == x2 && y1 != y2 {
                    return Self::Infinity(a, b);
                }

                // Find the slope
                if let Some(s) = Self::find_slope(self.clone(), other.clone()) {
                    // When P1 = P2 and y = 0
                    if self == other && y1 == BigInt::from(0) {
                        return Self::Infinity(a, b);
                    }

                    let x3 = BigInt::from(s.clone().pow(2_u32)) - x1.clone() - x2.clone();
                    let y3 = (s * (x1.clone() - x3.clone())) - y1;
                    return Point::new(x3, y3, a, b);
                } else {
                    Point::Infinity(a, b)
                }
            }
            _ => Self::OffCurve,
        }
    }
}
