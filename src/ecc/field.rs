use anyhow::{anyhow, Result};
use num::{BigUint, ToPrimitive};
use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(PartialOrd, Ord, PartialEq, Clone, Eq, Debug, Hash)]
pub struct FieldElement<T> {
    pub num: T,
    pub prime: T,
}

impl FieldElement<BigUint> {
    pub fn new(num: BigUint, prime: BigUint) -> Result<Self> {
        if num >= prime {
            Err(anyhow!(
                "Num {} not in field range O to {}",
                num,
                prime - BigUint::from(1_u32)
            ))
        } else {
            Ok(Self { num, prime })
        }
    }

    pub fn pow(self, exp: BigUint) -> Self {
        let base = self.num;
        let p = self.prime;
        let one = BigUint::from(1_u32);
        let n = exp.modpow(&one.clone(), &(p.clone() - one));
        FieldElement {
            num: base.modpow(&n, &p),
            prime: p,
        }
    }
}

impl AddAssign for FieldElement<BigUint> {
    fn add_assign(&mut self, other: Self) {
        if self.prime != other.prime {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.prime, self.prime, self.prime, other.prime,
            );
        }

        *self = Self {
            num: (&self.num + other.num).modpow(&BigUint::from(1 as u32), &self.prime),
            prime: other.prime,
        }
    }
}

impl Add for FieldElement<BigUint> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.prime, self.prime, self.prime, other.prime,
            );
        }
        Self {
            num: (self.num + other.num).modpow(&BigUint::from(1 as u32), &self.prime),
            prime: self.prime,
        }
    }
}

impl SubAssign for FieldElement<BigUint> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            num: (&self.num - other.num).modpow(&BigUint::from(1 as u32), &self.prime),
            prime: other.prime,
        };
    }
}

impl Sub for FieldElement<BigUint> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.prime, self.prime, self.prime, other.prime,
            );
        }

        Self {
            num: (self.num - other.num).modpow(&BigUint::from(1 as u32), &self.prime),
            prime: self.prime,
        }
    }
}

impl MulAssign for FieldElement<BigUint> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            num: (&self.num * other.num).modpow(&BigUint::from(1 as u32), &self.prime),
            prime: other.prime,
        };
    }
}

impl Mul for FieldElement<BigUint> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.prime, self.prime, self.prime, other.prime,
            );
        }

        Self {
            num: (self.num * other.num).modpow(&BigUint::from(1 as u32), &self.prime),
            prime: self.prime,
        }
    }
}

impl DivAssign for FieldElement<BigUint> {
    fn div_assign(&mut self, other: Self) {
        let p = other.prime.clone();
        if self.prime != other.prime {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.prime, self.prime, self.prime, other.prime,
            );
        }
        let order = other.prime.to_u32().expect("fail to cast to u32");
        let num = (&self.num * other.num.pow(order - 2)).modpow(&BigUint::from(1_u32), &p);
        *self = Self {
            num,
            prime: other.prime,
        }
    }
}

impl Div for FieldElement<BigUint> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let p = other.prime.clone();
        if self.prime != other.prime {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.prime, self.prime, self.prime, other.prime,
            );
        }
        let order = other.prime.to_u32().expect("fail to cast to u32");
        let num = (self.num * other.num.pow(order - 2)).modpow(&BigUint::from(1_u32), &p);
        Self {
            num,
            prime: self.prime,
        }
    }
}
