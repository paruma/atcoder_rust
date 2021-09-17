use cargo_snippet::snippet;

#[snippet]
pub mod tropical {
    use std::{cmp::Ordering, ops::Add};
    use Trop::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Trop {
        Inf,
        Fin(i64),
    }

    impl Trop {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `Trap::get_fin()` on a `Fin` value"),
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
    }

    impl Add for Trop {
        type Output = Trop;

        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }

    impl PartialOrd for Trop {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }

    impl Ord for Trop {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tropical::Trop::*;

    #[allow(clippy::eq_op)]
    #[test]
    fn test_trop_ord() {
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
    fn test_trop_add() {
        assert_eq!(Inf + Inf, Inf);
        assert_eq!(Inf + Fin(3), Inf);
        assert_eq!(Fin(3) + Inf, Inf);
        assert_eq!(Fin(3) + Fin(4), Fin(7));
    }

    // VSCodeでは #[should_panic]は無視されてしまうので通らない。
    #[test]
    #[should_panic]
    fn test_trop_get_fin_panic() {
        Inf.get_fin();
    }

    #[test]
    fn test_trop_util() {
        assert_eq!(Fin(3).get_fin(), 3);

        assert_eq!(Fin(3).get_fin_or(0), 3);
        assert_eq!(Inf.get_fin_or(0), 0);

        assert!(Fin(3).is_fin());
        assert!(!Inf.is_fin());

        assert!(!Fin(3).is_inf());
        assert!(Inf.is_inf());
    }
}
