use cargo_snippet::snippet;

#[snippet(prefix = "use mod_neg_ext_int::NegExtInt::{self, *};")]
pub mod mod_neg_ext_int {
    use std::{cmp::Ordering, ops::Add};
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
}

#[cfg(test)]
mod tests {
    use super::mod_neg_ext_int::NegExtInt::{self, *};

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
}
