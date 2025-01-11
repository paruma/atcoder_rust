use cargo_snippet::snippet;

#[snippet(prefix = "use mod_ext_int::ExtInt::{self, *};")]
/// 整数型に +∞ を追加したもの
/// 実行時間・メモリ消費量が約2倍になるので注意
///
/// [参考]
/// * ExtInt 未使用:  https://atcoder.jp/contests/abc375/submissions/58810350 (124 ms, 5944 KB)
/// * ExtInt 使用: https://atcoder.jp/contests/abc375/submissions/58810373 (200 ms, 9840 KB)
pub mod mod_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        iter::Sum,
        ops::{Add, AddAssign},
    };
    use ExtInt::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum ExtInt {
        Inf,
        Fin(i64),
    }

    impl ExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `ExtInt::get_fin()` on a `Inf` value"),
            }
        }

        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                Inf => default,
            }
        }

        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }

        pub fn is_inf(self) -> bool {
            matches!(self, Inf)
        }

        pub fn to_option(self) -> Option<i64> {
            match self {
                Inf => None,
                Fin(a) => Some(a),
            }
        }

        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Fin(a),
                None => Inf,
            }
        }

        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Fin(0),
                Ordering::Greater => match self {
                    Inf => Inf,
                    Fin(a) => Fin(a * t),
                },
            }
        }
    }

    impl Add for ExtInt {
        type Output = ExtInt;

        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
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
            match self {
                Inf => Inf,
                Fin(a) => Fin(a + rhs),
            }
        }
    }

    impl AddAssign<i64> for ExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }

    impl Sum for ExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                match x {
                    Inf => return Inf,
                    Fin(x) => s += x,
                }
            }
            Fin(s)
        }
    }

    impl PartialOrd for ExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }

    impl Ord for ExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl fmt::Display for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Inf => write!(f, "+∞"),
                Fin(x) => write!(f, "{x}"),
            }
        }
    }

    impl fmt::Debug for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Inf => write!(f, "+∞"),
                Fin(x) => write!(f, "{x}"),
            }
        }
    }

    pub struct ExtIntAdditive(Infallible);
    impl Monoid for ExtIntAdditive {
        type S = ExtInt;

        fn identity() -> Self::S {
            Fin(0)
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }

    pub struct ExtIntMin(Infallible);
    impl Monoid for ExtIntMin {
        type S = ExtInt;

        fn identity() -> Self::S {
            Inf
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.min(b)
        }
    }
}

#[cfg(test)]
mod tests {
    use ac_library::Monoid;

    use super::mod_ext_int::ExtInt::{self, *};
    use super::mod_ext_int::{ExtIntAdditive, ExtIntMin};

    #[allow(clippy::eq_op)]
    #[test]
    fn test_ext_int_ord() {
        assert!(Inf <= Inf);
        assert!(Fin(3) <= Inf);
        assert!(Fin(4) <= Fin(6));
        assert!(Fin(4) <= Fin(4));

        assert!(Inf >= Inf);
        assert!(Inf >= Fin(3));
        assert!(Fin(6) >= Fin(4));
        assert!(Fin(4) >= Fin(4));

        use std::cmp::min;

        assert_eq!(min(Inf, Inf), Inf);
        assert_eq!(min(Inf, Fin(3)), Fin(3));
        assert_eq!(min(Fin(3), Inf), Fin(3));
        assert_eq!(min(Fin(6), Fin(4)), Fin(4));
        assert_eq!(min(Fin(4), Fin(4)), Fin(4));
    }

    #[test]
    fn test_ext_int_add() {
        assert_eq!(Inf + Inf, Inf);
        assert_eq!(Inf + Fin(3), Inf);
        assert_eq!(Fin(3) + Inf, Inf);
        assert_eq!(Fin(3) + Fin(4), Fin(7));
    }

    #[test]
    fn test_ext_int_add_assign() {
        let mut x = Fin(3);
        x += Fin(4);
        assert_eq!(x, Fin(7));
        x += Inf;
        assert_eq!(x, Inf);
    }

    #[test]
    fn test_ext_int_add_i64() {
        assert_eq!(Inf + 4, Inf);
        assert_eq!(Fin(3) + 4, Fin(7));
    }

    #[test]
    fn test_ext_int_add_assign_i64() {
        let mut x = Fin(3);
        x += 4;
        assert_eq!(x, Fin(7));

        let mut y = Inf;
        y += 4;
        assert_eq!(y, Inf);
    }

    #[test]
    fn test_ext_int_sum() {
        let test = |xs: &[ExtInt], expected: ExtInt| {
            assert_eq!(xs.iter().copied().sum::<ExtInt>(), expected);
        };
        test(&[Fin(3), Fin(4), Fin(5)], Fin(12));
        test(&[Fin(3), Inf, Fin(5)], Inf);
        test(&[Fin(3)], Fin(3));
        test(&[Inf], Inf);
        test(&[], Fin(0));
    }

    #[test]
    #[should_panic]
    fn test_ext_int_get_fin_panic() {
        Inf.get_fin();
    }

    #[test]
    fn test_ext_int_util() {
        assert_eq!(Fin(3).get_fin(), 3);

        assert_eq!(Fin(3).get_fin_or(0), 3);
        assert_eq!(Inf.get_fin_or(0), 0);

        assert!(Fin(3).is_fin());
        assert!(!Inf.is_fin());

        assert!(!Fin(3).is_inf());
        assert!(Inf.is_inf());

        assert_eq!(Fin(3).to_option(), Some(3));
        assert_eq!(Inf.to_option(), None);

        assert_eq!(ExtInt::from_option(Some(3)), Fin(3));
        assert_eq!(ExtInt::from_option(None), Inf);
    }

    #[test]
    fn test_ext_int_times() {
        assert_eq!(Fin(3).times(0), Fin(0));
        assert_eq!(Fin(3).times(10), Fin(30));
        assert_eq!(Fin(0).times(0), Fin(0));
        assert_eq!(Fin(0).times(10), Fin(0));
        assert_eq!(Fin(-3).times(0), Fin(0));
        assert_eq!(Fin(-3).times(10), Fin(-30));
        assert_eq!(Inf.times(0), Fin(0)); // Inf を 0 回足した場合と考え、足し算の単位元 Fin(0) 扱い
        assert_eq!(Inf.times(10), Inf);
    }

    #[test]
    fn test_ext_int_additive() {
        type M = ExtIntAdditive;
        assert_eq!(M::binary_operation(&Fin(3), &Fin(4)), Fin(7));
        assert_eq!(M::binary_operation(&Fin(3), &Inf), Inf);
        assert_eq!(M::identity(), Fin(0));
        assert_eq!(M::binary_operation(&M::identity(), &Fin(5)), Fin(5));
        assert_eq!(M::binary_operation(&M::identity(), &Inf), Inf);
    }

    #[test]
    fn test_ext_int_min() {
        type M = ExtIntMin;
        assert_eq!(M::binary_operation(&Fin(3), &Fin(4)), Fin(3));
        assert_eq!(M::binary_operation(&Fin(3), &Inf), Fin(3));
        assert_eq!(M::identity(), Inf);
        assert_eq!(M::binary_operation(&M::identity(), &Fin(5)), Fin(5));
        assert_eq!(M::binary_operation(&M::identity(), &Inf), Inf);
    }
}
