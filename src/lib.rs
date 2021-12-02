pub mod ecc;
pub use num::{BigInt, BigUint};

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ecc::*;
    use anyhow::Result;
    use std::collections::HashSet;

    #[test]
    fn create_field_element() {
        let (num, prime): (u32, u32) = (13, 13);
        match FieldElement::new(BigUint::from(num), BigUint::from(prime)) {
            Err(_) => assert!(true, "Should return an error"),
            _ => assert!(false),
        }
    }

    #[test]
    fn add_field_elements() -> Result<()> {
        let (mut a, b) = (
            FieldElement::new(BigUint::from(7_u32), BigUint::from(17_u32))?,
            FieldElement::new(BigUint::from(8_u32), BigUint::from(17_u32))?,
        );
        let expect = FieldElement::new(BigUint::from(15_u32), BigUint::from(17_u32))?;
        assert_eq!(a.clone() + b.clone(), expect);
        a += b;
        assert_eq!(a, expect);
        Ok(())
    }

    #[test]
    fn sub_field_elements() -> Result<()> {
        let p = 17;
        let (a, mut b) = (
            FieldElement::new(BigUint::from(7_u32), BigUint::from(p as u32))?,
            FieldElement::new(BigUint::from(8_u32), BigUint::from(p as u32))?,
        );
        let expect = FieldElement::new(BigUint::from(1_u32), BigUint::from(p as u32))?;
        assert_eq!(b.clone() - a.clone(), expect);
        b -= a;
        assert_eq!(b, expect);
        Ok(())
    }

    #[test]
    fn mul_field_elements() -> Result<()> {
        let prime = 19;
        let (mut a, b) = (
            FieldElement::new(BigUint::from(5_u32), BigUint::from(prime as u32))?,
            FieldElement::new(BigUint::from(3_u32), BigUint::from(prime as u32))?,
        );
        let expect = FieldElement::new(BigUint::from(15_u32), BigUint::from(prime as u32))?;
        assert_eq!(a.clone() * b.clone(), expect);
        a *= b;
        assert_eq!(a, expect);
        Ok(())
    }

    #[test]
    fn fermat_theorem() -> Result<()> {
        let k = [1, 3, 7, 13, 18];
        let p = 19;
        let mut result_set = HashSet::new();
        let ns: Vec<BigUint> = (0..p).map(|n| BigUint::from(n as u32)).collect();

        for k in k {
            let mut res = vec![];
            let kf = FieldElement::new(BigUint::from(k as u32), BigUint::from(p as u32))?;
            for n in &ns {
                let nf = FieldElement::new(n.clone(), BigUint::from(p as u32))?;
                let a = kf.clone() * nf;
                res.push(a);
            }
            res.sort();
            result_set.insert(res);
        }

        assert_eq!(result_set.len(), 1);

        let r = result_set.into_iter().next().unwrap();

        let test: Vec<BigUint> = r.iter().map(|n| n.num.clone()).collect();

        assert_eq!(test, ns);

        Ok(())
    }

    #[test]
    fn test_exponential() -> Result<()> {
        let p: usize = 31;
        let e: u32 = 2;
        let base: usize = 10;
        let result = base.pow(e as u32).rem_euclid(p);
        let expected = FieldElement::new(BigUint::from(result), BigUint::from(p))?;
        let got = FieldElement::new(BigUint::from(base), BigUint::from(p))?.pow(BigUint::from(e));

        assert_eq!(got, expected);

        Ok(())
    }

    #[test]
    fn exp_field_elements() -> Result<()> {
        let ps: Vec<usize> = vec![7, 11, 17];

        for p in &ps {
            let mut set = vec![];
            for num in 1..*p {
                let a = FieldElement::new(BigUint::from(num as u32), BigUint::from(*p as u32))?;
                let exp = (*p - 1) as u32;
                let b = a.pow(BigUint::from(exp));
                set.push(b);
            }

            let got: Vec<BigUint> = set.iter().map(|f| f.num.clone()).collect();

            assert_eq!(set.len(), *p - 1, "Order of the set must equal to p-1");

            let mut ones = vec![];
            for _ in 0..*p - 1 {
                ones.push(BigUint::from(1_u32));
            }

            assert_eq!(got, ones);
        }

        Ok(())
    }

    #[test]
    fn div_field_elements() -> Result<()> {
        let p = BigUint::from(19_u32);
        let mut a = FieldElement::new(BigUint::from(2_u32), p.clone())?;
        let mut b = FieldElement::new(BigUint::from(7_u32), p.clone())?;
        let c = FieldElement::new(BigUint::from(5_u32), p.clone())?;

        let mut expect = FieldElement::new(BigUint::from(3_u32), p.clone())?;
        assert_eq!(a.clone() / b.clone(), expect, "f2 / f7 must equal to f3",);

        a /= b.clone();
        assert_eq!(a, expect);

        expect = FieldElement::new(BigUint::from(9_u32), p.clone())?;
        assert_eq!(b.clone() / c.clone(), expect, "f7 / f5 must equal to f9",);

        b /= c;
        assert_eq!(b, expect);

        Ok(())
    }

    #[test]
    fn create_point() {
        let (x, y, a, b) = (
            BigInt::from(-1),
            BigInt::from(-1),
            BigInt::from(5),
            BigInt::from(7),
        );
        let p = Point::new(x.clone(), y.clone(), a.clone(), b.clone());
        assert_eq!(p, Point::OnCurve(x, y, a, b));
    }

    #[test]
    fn point_addition_with_identity() {
        let (a, b) = (BigInt::from(5), BigInt::from(7));
        let p1 = Point::new(BigInt::from(-1), BigInt::from(-1), a.clone(), b.clone());
        let id = Point::identity(a, b);
        let s1 = p1.clone() + id.clone();
        let s2 = id + p1.clone();
        assert_eq!(s1, p1);
        assert_eq!(s2, p1);
    }

    #[test]
    fn point_additive_inverse() {
        let (a, b) = (BigInt::from(5), BigInt::from(7));
        let p1 = Point::new(BigInt::from(-1), BigInt::from(-1), a.clone(), b.clone());
        let p2 = Point::new(BigInt::from(-1), BigInt::from(1), a.clone(), b.clone());

        let s = p1.clone() + p2.clone();
        assert_eq!(s, Point::identity(a, b));
    }

    #[test]
    fn test_point_addition() {
        // curve y^2 = x^3 + 5*x + 7
        // a = 5, b = 7
        let (a, b) = (5, 7);
        let p = Point::new(
            BigInt::from(-1),
            BigInt::from(-1),
            BigInt::from(a),
            BigInt::from(b),
        );

        let mut res = p.clone() + p.clone();
        assert_eq!(res, Point::Infinity(BigInt::from(a), BigInt::from(b)));
        let p1 = Point::new(
            BigInt::from(2),
            BigInt::from(5),
            BigInt::from(a),
            BigInt::from(b),
        );
        res = p1 + p;
        assert_eq!(res, Point::Infinity(BigInt::from(a), BigInt::from(b)));
    }
}
