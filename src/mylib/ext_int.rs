use cargo_snippet::snippet;

#[snippet(prefix = "use mod_ext_int::ExtInt::{self, *};")]

pub mod mod_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign},
    };

    pub const INF: ExtInt = ExtInt::INF;
    pub fn fin(x: i64) -> ExtInt {
        ExtInt::fin(x)
    }

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ExtInt(i64);
    impl ExtInt {
        pub const INF: Self = Self(i64::MAX);

        pub fn fin(x: i64) -> Self {
            Self(x)
        }
        pub fn get_fin(self) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                panic!("called `ExtInt::get_fin()` on a infinity")
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                default
            }
        }
        #[inline]
        pub fn is_fin(self) -> bool {
            self.0 != i64::MAX
        }
        pub fn is_inf(self) -> bool {
            self.0 == i64::MAX
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() {
                Some(self.0)
            } else {
                None
            }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Self(a),
                None => Self::INF,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Self(0),
                Ordering::Greater => {
                    if self.is_fin() {
                        Self(self.0 * t)
                    } else {
                        Self::INF
                    }
                }
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            if self.is_inf() || rhs.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 + rhs.0)
            }
        }
    }
    impl AddAssign for ExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            if self.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 + rhs)
            }
        }
    }
    impl AddAssign<i64> for ExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl std::iter::Sum for ExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                if x.is_inf() {
                    return Self::INF;
                }
                s += x.0;
            }
            Self::fin(s)
        }
    }
    impl fmt::Display for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_inf() {
                write!(f, "+∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    impl fmt::Debug for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_inf() {
                write!(f, "+∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    pub struct ExtIntAdditive(Infallible);
    impl Monoid for ExtIntAdditive {
        type S = ExtInt;
        fn identity() -> Self::S {
            ExtInt::fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct ExtIntMin(Infallible);
    impl Monoid for ExtIntMin {
        type S = ExtInt;
        fn identity() -> Self::S {
            ExtInt::INF
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.min(b)
        }
    }
}
mod tests {

    use ac_library::Monoid;

    use super::mod_ext_int::*;

    #[allow(clippy::eq_op)]
    #[test]
    fn test_ext_int_ord() {
        let _x: ExtInt = fin(3);

        assert!(INF >= INF);
        assert!(INF >= fin(3));
        assert!(fin(6) >= fin(4));
        assert!(fin(4) >= fin(4));

        assert!(INF <= INF);
        assert!(fin(3) <= INF);
        assert!(fin(4) <= fin(6));
        assert!(fin(4) <= fin(4));

        use std::cmp::min;

        assert_eq!(min(INF, INF), INF);
        assert_eq!(min(INF, fin(3)), fin(3));
        assert_eq!(min(fin(3), INF), fin(3));
        assert_eq!(min(fin(6), fin(4)), fin(4));
        assert_eq!(min(fin(4), fin(4)), fin(4));
    }

    #[test]
    fn test_ext_int_add() {
        assert_eq!(INF + INF, INF);
        assert_eq!(INF + fin(3), INF);
        assert_eq!(fin(3) + INF, INF);
        assert_eq!(fin(3) + fin(4), fin(7));
    }

    #[test]
    fn test_ext_int_add_assign() {
        let mut x = fin(3);
        x += fin(4);
        assert_eq!(x, fin(7));
        x += INF;
        assert_eq!(x, INF);
    }

    #[test]
    fn test_ext_int_add_i64() {
        assert_eq!(INF + 4, INF);
        assert_eq!(fin(3) + 4, fin(7));
    }

    #[test]
    fn test_ext_int_add_assign_i64() {
        let mut x = fin(3);
        x += 4;
        assert_eq!(x, fin(7));

        let mut y = INF;
        y += 4;
        assert_eq!(y, INF);
    }

    #[test]
    fn test_ext_int_sum() {
        let test = |xs: &[ExtInt], expected: ExtInt| {
            assert_eq!(xs.iter().copied().sum::<ExtInt>(), expected);
        };
        test(&[fin(3), fin(4), fin(5)], fin(12));
        test(&[fin(3), INF, fin(5)], INF);
        test(&[fin(3)], fin(3));
        test(&[INF], INF);
        test(&[], fin(0));
    }

    #[test]
    #[should_panic]
    fn test_ext_int_get_fin_panic() {
        INF.get_fin();
    }

    #[test]
    fn test_ext_int_util() {
        assert_eq!(fin(3).get_fin(), 3);

        assert_eq!(fin(3).get_fin_or(0), 3);
        assert_eq!(INF.get_fin_or(0), 0);

        assert!(fin(3).is_fin());
        assert!(!INF.is_fin());

        assert!(!fin(3).is_inf());
        assert!(INF.is_inf());

        assert_eq!(fin(3).to_option(), Some(3));
        assert_eq!(INF.to_option(), None);

        assert_eq!(ExtInt::from_option(Some(3)), fin(3));
        assert_eq!(ExtInt::from_option(None), INF);
    }

    #[test]
    fn test_ext_int_times() {
        assert_eq!(fin(3).times(0), fin(0));
        assert_eq!(fin(3).times(10), fin(30));
        assert_eq!(fin(0).times(0), fin(0));
        assert_eq!(fin(0).times(10), fin(0));
        assert_eq!(fin(-3).times(0), fin(0));
        assert_eq!(fin(-3).times(10), fin(-30));
        assert_eq!(INF.times(0), fin(0)); // INF を 0 回足した場合と考え、足し算の単位元 fin(0) 扱い
        assert_eq!(INF.times(10), INF);
    }

    #[test]
    fn test_ext_int_additive() {
        type M = ExtIntAdditive;
        assert_eq!(M::binary_operation(&fin(3), &fin(4)), fin(7));
        assert_eq!(M::binary_operation(&fin(3), &INF), INF);
        assert_eq!(M::identity(), fin(0));
        assert_eq!(M::binary_operation(&M::identity(), &fin(5)), fin(5));
        assert_eq!(M::binary_operation(&M::identity(), &INF), INF);
    }

    #[test]
    fn test_ext_int_min() {
        type M = ExtIntMin;
        assert_eq!(M::binary_operation(&fin(3), &fin(4)), fin(3));
        assert_eq!(M::binary_operation(&fin(3), &INF), fin(3));
        assert_eq!(M::identity(), INF);
        assert_eq!(M::binary_operation(&M::identity(), &fin(5)), fin(5));
        assert_eq!(M::binary_operation(&M::identity(), &INF), INF);
    }
}
