mod integer_tools {
    use num::Integer;

    #[allow(dead_code)]

    pub fn divisor(n: i64) -> Vec<i64> {
        assert!(n >= 1);
        let mut retval: Vec<i64> = Vec::new();
        for i in 1..=num_integer::sqrt(n) {
            if n.is_multiple_of(&i) {
                retval.push(i);
                if i * i != n {
                    retval.push(n / i);
                }
            }
        }

        retval
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_divisor() {
        use super::integer_tools::*;
        let test = |n: i64, ans: &[i64]| {
            let mut divisor = divisor(n);
            divisor.sort_unstable();
            assert_eq!(divisor, ans);
        };
        test(1, &[1]);
        test(2, &[1, 2]);
        test(16, &[1, 2, 4, 8, 16]);
        test(12, &[1, 2, 3, 4, 6, 12]);
    }
}
