pub mod ecc;

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::ecc::*;

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
        let p = 17;
        let (a, b) = (
            FieldElement::new(7, p)?,
            FieldElement::new(8, p)?,
        );
        // assert_eq!(a - b, FieldElement::new(16, p)?);
        assert_eq!(b - a, FieldElement::new(1, p)?);

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
    fn fermat_theorem() -> Result<(), String> {
        let k = [1, 3, 7, 13, 18];
        let p = 19;
        let mut result_set = HashSet::new();
        let ns: Vec<usize> = (0..p).collect();

        for k in k {
            let mut res = vec![];
            let kf = FieldElement::new(k, p as usize)?;
            for n in &ns {
                let nf = FieldElement::new(*n, p)?;
                let a = kf * nf;
                res.push(a);
            }
            res.sort();
            result_set.insert(res);
        }

        assert_eq!(result_set.len(), 1);

        let r = result_set
            .into_iter()
            .next()
            .unwrap();

        let test: Vec<usize> = r.iter()
            .map(|n| { n.num() })
            .collect();

        assert_eq!(test, ns);

        Ok(())
    }

    #[test]
    fn test_exponential() -> Result<(), String> {
        let p: usize = 31;
        let e: u32 = 2;
        let base: usize = 10;
        let result = base.pow(e as u32).rem_euclid(p);

        let expected = FieldElement::new(result, p)?;
        let got = FieldElement::new(base, p)?.pow(e);

        assert_eq!(got, expected);

        Ok(())
    }

    #[test]
    fn exp_field_elements() -> Result<(), String> {
        let ps: Vec<usize> = vec![7, 11, 17];

        for p in &ps {
            let mut set = vec![];
            for num in 1..*p {
                let a = FieldElement::new(num, *p)?;
                let exp = (*p - 1) as u32;
                let b = a.pow(exp);
                set.push(b);
            }

            let got = set.iter()
                .map(|f| f.0)
                .collect::<Vec<usize>>();

            assert_eq!(set.len(), *p-1, "Order of the set must equal to p-1");

            let mut ones = vec![];
            for _ in 0..*p-1 {
                ones.push(1);
            }

            assert_eq!(got, ones);
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

    #[test]
    fn create_point() -> Result<(), String> {
        let (x, y, a, b) = (-1, -1, 5, 7);
        match Point::new(x, y, a, b) {
            Ok(p) => assert_eq!(p, Point { a, b, x, y }),
            _ => assert!(false),
        }

        let (x, y, a, b) = (-1, -2, 5, 7);
        match Point::new(x, y, a, b) {
            Err(_) => assert!(true),
            _ => assert!(false),
        }

        Ok(())
    }
}
