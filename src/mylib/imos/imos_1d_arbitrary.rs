use crate::ab_group::ab_group::AbGroup;
use cargo_snippet::snippet;

#[snippet(prefix = "use imos_1d_arbitrary::*;", include = "ab_group")]
#[allow(clippy::module_inception)]
pub mod imos_1d_arbitrary {
    use super::AbGroup;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Imos1DArbitrary<G: AbGroup> {
        raw: Vec<G::S>,
        begin: i64,
        end: i64,
    }

    impl<G: AbGroup> Imos1DArbitrary<G> {
        pub fn new(begin: i64, end: i64) -> Self {
            debug_assert!(begin < end);
            let len = (end - begin) as usize;
            let mut raw = Vec::with_capacity(len);
            for _ in 0..len {
                raw.push(G::zero());
            }
            Self { raw, begin, end }
        }

        fn is_within(&self, i: i64) -> bool {
            (self.begin..self.end).contains(&i)
        }

        pub fn get(&self, i: i64) -> G::S {
            if cfg!(debug_assertions) && !self.is_within(i) {
                panic!(
                    "index out of bounds: the range is [{}, {}) but the index is {}",
                    self.begin, self.end, i
                );
            }
            self.raw[(i - self.begin) as usize].clone()
        }

        pub fn add(&mut self, i: i64, val: G::S) {
            if cfg!(debug_assertions) && !self.is_within(i) {
                panic!(
                    "index out of bounds: the range is [{}, {}) but the index is {}",
                    self.begin, self.end, i
                );
            }
            let idx = (i - self.begin) as usize;
            self.raw[idx] = G::add(&self.raw[idx], &val);
        }

        pub fn summation(&mut self) {
            for i in 1..self.raw.len() {
                self.raw[i] = G::add(&self.raw[i - 1], &self.raw[i]);
            }
        }

        pub fn difference(&mut self) {
            for i in (1..self.raw.len()).rev() {
                self.raw[i] = G::sub(&self.raw[i], &self.raw[i - 1]);
            }
        }
    }
}

#[cfg(test)]
mod tests_imos_1d_arbitrary {
    use super::imos_1d_arbitrary::*;
    use crate::ab_group::ab_group::AdditiveAbGroup;
    use itertools::Itertools;

    #[test]
    fn test_imos_1d_arbitrary_const_func() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = Imos1DArbitrary::<G>::new(-2, 5 + 1);

        imos.add(-2, 1);
        imos.add(0, -1);
        imos.add(3, 2);
        imos.add(5, -2);
        imos.add(1, 4);
        imos.add(3, -4);

        imos.summation();

        let actual = (-2..5).map(|x| imos.get(x)).collect_vec();
        let expected = vec![1, 1, 0, 4, 4, 2, 2];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_1d_arbitrary_affine() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = Imos1DArbitrary::<G>::new(0, 9 + 2);

        let f = |x: i64| x + 3;
        imos.add(3, f(3));
        imos.add(4, f(4) - 2 * f(3));
        imos.add(7, -f(7));
        imos.add(8, -(f(8) - 2 * f(7)));

        let g = |x: i64| 2 * x + 1;
        imos.add(5, g(5));
        imos.add(6, g(6) - 2 * g(5));
        imos.add(9, -g(9));
        imos.add(10, -(g(10) - 2 * g(9)));

        imos.summation();
        imos.summation();

        let actual = (0..11).map(|x| imos.get(x)).collect_vec();
        let expected = vec![0, 0, 0, 6, 7, 19, 22, 15, 17, 0, 0];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_1d_arbitrary_quadratic() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = Imos1DArbitrary::<G>::new(0, 5 + 3);

        let f = |x: i64| if x >= 1 { x * x } else { 0 };
        for t in 0..3 {
            let begin = 1;
            imos.add(
                begin + t,
                f(begin + t) - 3 * f(begin + t - 1) + 3 * f(begin + t - 2),
            )
        }
        let f = |x: i64| if x >= 5 { -x * x } else { 0 };
        for t in 0..3 {
            let end = 5;
            imos.add(
                end + t,
                f(end + t) - 3 * f(end + t - 1) + 3 * f(end + t - 2),
            )
        }

        imos.summation();
        imos.summation();
        imos.summation();

        let actual = (0..5).map(|x| imos.get(x)).collect_vec();
        let expected = vec![0, 1, 4, 9, 16];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_1d_arbitrary_difference() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = Imos1DArbitrary::<G>::new(0, 5);
        for i in 0..5 {
            imos.add(i, i * i);
        }
        imos.difference();

        let actual = (0..5).map(|x| imos.get(x)).collect_vec();
        let expected = vec![0, 1, 3, 5, 7];
        assert_eq!(actual, expected)
    }

    #[test]
    #[should_panic]
    fn test_imos_1d_arbitrary_get_out_of_bounds_low() {
        type G = AdditiveAbGroup<i64>;
        let imos = Imos1DArbitrary::<G>::new(0, 5);
        imos.get(-1);
    }

    #[test]
    #[should_panic]
    fn test_imos_1d_arbitrary_get_out_of_bounds_high() {
        type G = AdditiveAbGroup<i64>;
        let imos = Imos1DArbitrary::<G>::new(0, 5);
        imos.get(5);
    }

    #[test]
    #[should_panic]
    fn test_imos_1d_arbitrary_add_out_of_bounds_low() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = Imos1DArbitrary::<G>::new(0, 5);
        imos.add(-1, 1);
    }

    #[test]
    #[should_panic]
    fn test_imos_1d_arbitrary_add_out_of_bounds_high() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = Imos1DArbitrary::<G>::new(0, 5);
        imos.add(5, 1);
    }
}
