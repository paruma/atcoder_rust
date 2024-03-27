use cargo_snippet::snippet;

#[snippet(prefix = "use mod_neg_ext_int::NegExtInt::{self, *};")]
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        ops::{Add, AddAssign},
    };
    use NegExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NegExtInt {
        NegInf,
        Fin(i64),
    }

    impl NegExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => panic!("called `NegExtInt::get_fin()` on a `Fin` value"),
            }
        }

        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => default,
            }
        }

        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }

        pub fn is_neginf(self) -> bool {
            matches!(self, NegInf)
        }

        pub fn to_option(self) -> Option<i64> {
            match self {
                NegInf => None,
                Fin(a) => Some(a),
            }
        }

        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Fin(a),
                None => NegInf,
            }
        }

        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Fin(0),
                Ordering::Greater => match self {
                    NegInf => NegInf,
                    Fin(a) => Fin(a * t),
                },
            }
        }
    }

    impl Add for NegExtInt {
        type Output = NegExtInt;

        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (NegInf, NegInf) => NegInf,
                (NegInf, Fin(_)) => NegInf,
                (Fin(_), NegInf) => NegInf,
                (Fin(a), Fin(b)) => Fin(a + b),
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
            match self {
                NegInf => NegInf,
                Fin(a) => Fin(a + rhs),
            }
        }
    }

    impl AddAssign<i64> for NegExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }

    impl std::iter::Sum for NegExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                match x {
                    NegInf => return NegInf,
                    Fin(x) => s += x,
                }
            }
            Fin(s)
        }
    }

    impl PartialOrd for NegExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (NegInf, NegInf) => Some(Ordering::Equal),
                (NegInf, Fin(_)) => Some(Ordering::Less),
                (Fin(_), NegInf) => Some(Ordering::Greater),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }

    impl Ord for NegExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;

        fn identity() -> Self::S {
            Fin(0)
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }

    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;

        fn identity() -> Self::S {
            NegInf
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
}

#[cfg(test)]
mod tests {

    use ac_library::Monoid;

    use super::mod_neg_ext_int::NegExtInt::{self, *};
    use super::mod_neg_ext_int::{NegExtIntAdditive, NegExtIntMax};

    #[allow(clippy::eq_op)]
    #[test]
    fn test_neg_ext_int_ord() {
        let _x: NegExtInt = Fin(3);

        assert!(NegInf >= NegInf);
        assert!(Fin(3) >= NegInf);
        assert!(Fin(6) >= Fin(4));
        assert!(Fin(4) >= Fin(4));

        assert!(NegInf <= NegInf);
        assert!(NegInf <= Fin(3));
        assert!(Fin(4) <= Fin(6));
        assert!(Fin(4) <= Fin(4));

        use std::cmp::max;

        assert_eq!(max(NegInf, NegInf), NegInf);
        assert_eq!(max(NegInf, Fin(3)), Fin(3));
        assert_eq!(max(Fin(3), NegInf), Fin(3));
        assert_eq!(max(Fin(6), Fin(4)), Fin(6));
        assert_eq!(max(Fin(4), Fin(4)), Fin(4));
    }

    #[test]
    fn test_neg_ext_int_add() {
        assert_eq!(NegInf + NegInf, NegInf);
        assert_eq!(NegInf + Fin(3), NegInf);
        assert_eq!(Fin(3) + NegInf, NegInf);
        assert_eq!(Fin(3) + Fin(4), Fin(7));
    }

    #[test]
    fn test_neg_ext_int_add_assign() {
        let mut x = Fin(3);
        x += Fin(4);
        assert_eq!(x, Fin(7));
        x += NegInf;
        assert_eq!(x, NegInf);
    }

    #[test]
    fn test_ext_int_add_i64() {
        assert_eq!(NegInf + 4, NegInf);
        assert_eq!(Fin(3) + 4, Fin(7));
    }

    #[test]
    fn test_ext_int_add_assign_i64() {
        let mut x = Fin(3);
        x += 4;
        assert_eq!(x, Fin(7));

        let mut y = NegInf;
        y += 4;
        assert_eq!(y, NegInf);
    }

    #[test]
    fn test_neg_ext_int_sum() {
        let test = |xs: &[NegExtInt], expected: NegExtInt| {
            assert_eq!(xs.iter().copied().sum::<NegExtInt>(), expected);
        };
        test(&[Fin(3), Fin(4), Fin(5)], Fin(12));
        test(&[Fin(3), NegInf, Fin(5)], NegInf);
        test(&[Fin(3)], Fin(3));
        test(&[NegInf], NegInf);
        test(&[], Fin(0));
    }

    #[test]
    #[should_panic]
    fn test_neg_ext_int_get_fin_panic() {
        NegInf.get_fin();
    }

    #[test]
    fn test_neg_ext_int_util() {
        assert_eq!(Fin(3).get_fin(), 3);

        assert_eq!(Fin(3).get_fin_or(0), 3);
        assert_eq!(NegInf.get_fin_or(0), 0);

        assert!(Fin(3).is_fin());
        assert!(!NegInf.is_fin());

        assert!(!Fin(3).is_neginf());
        assert!(NegInf.is_neginf());

        assert_eq!(Fin(3).to_option(), Some(3));
        assert_eq!(NegInf.to_option(), None);

        assert_eq!(NegExtInt::from_option(Some(3)), Fin(3));
        assert_eq!(NegExtInt::from_option(None), NegInf);
    }

    #[test]
    fn test_extint_times() {
        assert_eq!(Fin(3).times(0), Fin(0));
        assert_eq!(Fin(3).times(10), Fin(30));
        assert_eq!(Fin(0).times(0), Fin(0));
        assert_eq!(Fin(0).times(10), Fin(0));
        assert_eq!(Fin(-3).times(0), Fin(0));
        assert_eq!(Fin(-3).times(10), Fin(-30));
        assert_eq!(NegInf.times(0), Fin(0)); // Inf を 0 回足した場合と考え、足し算の単位元 Fin(0) 扱い
        assert_eq!(NegInf.times(10), NegInf);
    }

    #[test]
    fn test_ext_int_additive() {
        type M = NegExtIntAdditive;
        assert_eq!(M::binary_operation(&Fin(3), &Fin(4)), Fin(7));
        assert_eq!(M::binary_operation(&Fin(3), &NegInf), NegInf);
        assert_eq!(M::identity(), Fin(0));
        assert_eq!(M::binary_operation(&M::identity(), &Fin(5)), Fin(5));
        assert_eq!(M::binary_operation(&M::identity(), &NegInf), NegInf);
    }

    #[test]
    fn test_ext_int_min() {
        type M = NegExtIntMax;
        assert_eq!(M::binary_operation(&Fin(3), &Fin(4)), Fin(4));
        assert_eq!(M::binary_operation(&Fin(3), &NegInf), Fin(3));
        assert_eq!(M::identity(), NegInf);
        assert_eq!(M::binary_operation(&M::identity(), &Fin(5)), Fin(5));
        assert_eq!(M::binary_operation(&M::identity(), &NegInf), NegInf);
    }
}
