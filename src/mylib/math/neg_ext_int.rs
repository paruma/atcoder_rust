use crate::math::algebra::min_max_monoid::min_max_monoid::{BoundedAbove, BoundedBelow};
use cargo_snippet::snippet;

#[snippet(prefix = "use mod_neg_ext_int::*;")]
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign, Sub, SubAssign},
    };

    pub const NEG_INF: NegExtInt = NegExtInt::NEG_INF;
    pub fn fin(x: i64) -> NegExtInt {
        NegExtInt::fin(x)
    }

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NegExtInt(i64);
    impl NegExtInt {
        pub const NEG_INF: Self = Self(i64::MIN);

        pub fn fin(x: i64) -> Self {
            Self(x)
        }
        pub fn get_fin(self) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                panic!("called `NegExtInt::get_fin()` on a negative infinity")
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
            self.0 != i64::MIN
        }
        pub fn is_neg_inf(self) -> bool {
            self.0 == i64::MIN
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() {
                Some(self.0)
            } else {
                None
            }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Self(a),
                None => Self::NEG_INF,
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
                        Self::NEG_INF
                    }
                }
            }
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            if self.is_neg_inf() || rhs.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 + rhs.0)
            }
        }
    }
    impl AddAssign for NegExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            if self.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 + rhs)
            }
        }
    }
    impl AddAssign<i64> for NegExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl Sub<i64> for NegExtInt {
        type Output = NegExtInt;
        fn sub(self, rhs: i64) -> Self::Output {
            if self.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 - rhs)
            }
        }
    }
    impl SubAssign<i64> for NegExtInt {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs;
        }
    }
    impl std::iter::Sum for NegExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                if x.is_neg_inf() {
                    return Self::NEG_INF;
                }
                s += x.0;
            }
            Self::fin(s)
        }
    }
    impl fmt::Display for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_neg_inf() {
                write!(f, "-∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    impl fmt::Debug for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_neg_inf() {
                write!(f, "-∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegExtInt::fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegExtInt::NEG_INF
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
}

#[snippet(prefix = "use mod_neg_ext_int_bounded::*;")]
pub mod mod_neg_ext_int_bounded {
    use super::mod_neg_ext_int::NegExtInt;
    use super::{BoundedAbove, BoundedBelow};

    impl BoundedBelow for NegExtInt {
        fn min_value() -> Self {
            Self::NEG_INF
        }
    }

    impl BoundedAbove for NegExtInt {
        fn max_value() -> Self {
            Self::fin(i64::MAX)
        }
    }
}

#[cfg(test)]
mod tests {
    use ac_library::Monoid;

    use super::mod_neg_ext_int::*;

    #[allow(clippy::eq_op)]
    #[test]
    fn test_neg_ext_int_ord() {
        let _x: NegExtInt = fin(3);

        assert!(NEG_INF >= NEG_INF);
        assert!(fin(3) >= NEG_INF);
        assert!(fin(6) >= fin(4));
        assert!(fin(4) >= fin(4));

        assert!(NEG_INF <= NEG_INF);
        assert!(NEG_INF <= fin(3));
        assert!(fin(4) <= fin(6));
        assert!(fin(4) <= fin(4));

        use std::cmp::max;

        assert_eq!(max(NEG_INF, NEG_INF), NEG_INF);
        assert_eq!(max(NEG_INF, fin(3)), fin(3));
        assert_eq!(max(fin(3), NEG_INF), fin(3));
        assert_eq!(max(fin(6), fin(4)), fin(6));
        assert_eq!(max(fin(4), fin(4)), fin(4));
    }

    #[test]
    fn test_neg_ext_int_add() {
        assert_eq!(NEG_INF + NEG_INF, NEG_INF);
        assert_eq!(NEG_INF + fin(3), NEG_INF);
        assert_eq!(fin(3) + NEG_INF, NEG_INF);
        assert_eq!(fin(3) + fin(4), fin(7));
    }

    #[test]
    fn test_neg_ext_int_add_assign() {
        let mut x = fin(3);
        x += fin(4);
        assert_eq!(x, fin(7));
        x += NEG_INF;
        assert_eq!(x, NEG_INF);
    }

    #[test]
    fn test_neg_ext_int_add_i64() {
        assert_eq!(NEG_INF + 4, NEG_INF);
        assert_eq!(fin(3) + 4, fin(7));
    }

    #[test]
    fn test_neg_ext_int_add_assign_i64() {
        let mut x = fin(3);
        x += 4;
        assert_eq!(x, fin(7));

        let mut y = NEG_INF;
        y += 4;
        assert_eq!(y, NEG_INF);
    }

    #[test]
    fn test_ext_int_sub_i64() {
        assert_eq!(NEG_INF - 4, NEG_INF);
        assert_eq!(fin(3) - 4, fin(-1));
    }

    #[test]
    fn test_ext_int_sub_assign_i64() {
        let mut x = fin(3);
        x -= 4;
        assert_eq!(x, fin(-1));

        let mut y = NEG_INF;
        y -= 4;
        assert_eq!(y, NEG_INF);
    }

    #[test]
    fn test_neg_ext_int_sum() {
        let test = |xs: &[NegExtInt], expected: NegExtInt| {
            assert_eq!(xs.iter().copied().sum::<NegExtInt>(), expected);
        };
        test(&[fin(3), fin(4), fin(5)], fin(12));
        test(&[fin(3), NEG_INF, fin(5)], NEG_INF);
        test(&[fin(3)], fin(3));
        test(&[NEG_INF], NEG_INF);
        test(&[], fin(0));
    }

    #[test]
    #[should_panic]
    fn test_neg_ext_int_get_fin_panic() {
        NEG_INF.get_fin();
    }

    #[test]
    fn test_neg_ext_int_util() {
        assert_eq!(fin(3).get_fin(), 3);

        assert_eq!(fin(3).get_fin_or(0), 3);
        assert_eq!(NEG_INF.get_fin_or(0), 0);

        assert!(fin(3).is_fin());
        assert!(!NEG_INF.is_fin());

        assert!(!fin(3).is_neg_inf());
        assert!(NEG_INF.is_neg_inf());

        assert_eq!(fin(3).to_option(), Some(3));
        assert_eq!(NEG_INF.to_option(), None);

        assert_eq!(NegExtInt::from_option(Some(3)), fin(3));
        assert_eq!(NegExtInt::from_option(None), NEG_INF);
    }

    #[test]
    fn test_neg_ext_int_times() {
        assert_eq!(fin(3).times(0), fin(0));
        assert_eq!(fin(3).times(10), fin(30));
        assert_eq!(fin(0).times(0), fin(0));
        assert_eq!(fin(0).times(10), fin(0));
        assert_eq!(fin(-3).times(0), fin(0));
        assert_eq!(fin(-3).times(10), fin(-30));
        assert_eq!(NEG_INF.times(0), fin(0)); // NEG_INF を 0 回足した場合と考え、足し算の単位元 fin(0) 扱い
        assert_eq!(NEG_INF.times(10), NEG_INF);
    }

    #[test]
    fn test_neg_ext_int_additive() {
        type M = NegExtIntAdditive;
        assert_eq!(M::binary_operation(&fin(3), &fin(4)), fin(7));
        assert_eq!(M::binary_operation(&fin(3), &NEG_INF), NEG_INF);
        assert_eq!(M::identity(), fin(0));
        assert_eq!(M::binary_operation(&M::identity(), &fin(5)), fin(5));
        assert_eq!(M::binary_operation(&M::identity(), &NEG_INF), NEG_INF);
    }

    #[test]
    fn test_neg_ext_int_min() {
        type M = NegExtIntMax;
        assert_eq!(M::binary_operation(&fin(3), &fin(4)), fin(4));
        assert_eq!(M::binary_operation(&fin(3), &NEG_INF), fin(3));
        assert_eq!(M::identity(), NEG_INF);
        assert_eq!(M::binary_operation(&M::identity(), &fin(5)), fin(5));
        assert_eq!(M::binary_operation(&M::identity(), &NEG_INF), NEG_INF);
    }

    #[test]
    fn test_neg_ext_int_fmt() {
        assert_eq!(format!("{}", fin(3)), "3");
        assert_eq!(format!("{:?}", fin(3)), "3");

        assert_eq!(format!("{}", NEG_INF), "-∞");
        assert_eq!(format!("{:?}", NEG_INF), "-∞");
    }
}
