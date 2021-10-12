use std::{ops::{Add, Sub, Mul, Div}};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct FieldElement(isize, usize);

impl FieldElement {
    fn new(num: isize, prime: usize) -> Result<Self, String> {
        if num >= prime as isize {
            Err(format!("Num {} not in field range O to {}", num, prime - 1))
        } else {
            Ok(Self(num, prime))
        }
    }

    fn num(&self) -> isize {
        self.0
    }

    fn prime(&self) -> usize {
        self.1
    }

    fn pow(self, exp: u32) -> Self {
        let mut total = self.0;
        for _ in 0..exp {
            total = (total * self.0).rem_euclid(self.1 as isize);
        }
        FieldElement(total, self.1)
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
        Self((self.0 + other.0).rem_euclid(self.1 as isize), self.1)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.1 != other.1 {
            panic!(
                "Expect {} == {}, found {} != {}",
                self.1, self.1,
                self.1, other.1,
            );
        }
        Self ((self.0 - other.0).rem_euclid(self.1 as isize), self.1)
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
        Self((self.0 * other.0).rem_euclid(self.1 as isize), self.1)
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
        let num = (self.0 * other.0.pow(other.1 as u32 - 2)) % other.1 as isize;
        Self(num, self.1)
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn create_field_element() {
        let (num, prime) = (13, 13);
        match FieldElement::new(num, prime) {
            Err(s) => assert_eq!(s, format!("Num {} not in field range O to {}", num, prime - 1)),
            _ => assert!(false),
        }

        let (num, prime) = (7, 13);
        match FieldElement::new(num, prime) {
            Ok(fe) => assert_eq!(fe, FieldElement(num, prime)),
            _ => assert!(false),
        }
    }

    #[test]
    fn add_field_elements() -> Result<(), String> {
        let (a, b) = (
            FieldElement::new(7, 17)?,
            FieldElement::new(8, 17)?,
        );

        assert_eq!(a + b, FieldElement::new(15, 17)?);

        Ok(())
    }

    #[test]
    fn sub_field_elements() -> Result<(), String> {
        let prime = 17;
        let (a, b) = (
            FieldElement::new(7, prime)?,
            FieldElement::new(8, prime)?,
        );
        assert_eq!(a - b, FieldElement::new(16, prime)?);

        Ok(())
    }

    #[test]
    fn mul_field_elements() -> Result<(), String> {
        let prime = 19;
        let (a, b) = (
            FieldElement::new(5, prime)?,
            FieldElement::new(3, prime)?,
        );
        assert_eq!(a * b, FieldElement::new(15, prime)?);

        Ok(())
    }

    #[test]
    fn exp_field_elements() -> Result<(), String> {
        let primes: Vec<usize> = vec![7, 11, 17, 31];
        for prime in &primes {
            let mut set = vec![];
            for num in 1..*prime as isize {
                let a = FieldElement::new(num, *prime)?;
                let exp = (*prime as u32) - 1;
                let b = a.pow(exp);
                set.push(b);
            }

            let mut result = set.iter()
                .map(|f| f.0)
                .collect::<Vec<isize>>();
            result.sort_unstable();
            let expected = (1..*prime).map(|n| { n as isize }).collect::<Vec<isize>>();

            assert_eq!(set.len(), *prime-1, "Order of the set must equal to p-1");
            assert_eq!(result, expected);
        }

        Ok(())
    }

    #[test]
    fn div_field_elements() {
        let prime: usize = 19;
        let a = FieldElement(2, prime);
        let b = FieldElement(7, prime);
        let c = FieldElement(5, prime);
        assert_eq!(a / b, FieldElement(3, prime));
        assert_eq!(b / c, FieldElement(9, prime));
    }


}
