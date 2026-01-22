use cargo_snippet::snippet;

#[allow(unused_variables)]
#[allow(clippy::module_inception)]
#[snippet(prefix = "use map_monoid_template::*;")]
pub mod map_monoid_template {
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub len: usize,
    }

    impl RangeXxx {
        pub fn unit(x: i64) -> Self {
            Self { len: 1 }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;

        fn identity() -> Self::S {
            RangeXxx { len: 0 }
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeXxx { len: a.len + b.len }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeYyyRangeXxx(Infallible);
    impl MapMonoid for RangeYyyRangeXxx {
        type M = RangeXxxMonoid;

        type F = (); // 用途に応じて実装する

        fn identity_map() -> Self::F {}

        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            RangeXxx { len: x.len }
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {}
    }
}

#[cfg(test)]
pub mod test_map_monoid_template {

    use ac_library::{MapMonoid, Monoid};

    use super::map_monoid_template::*;

    #[test]
    fn test_map_monoid_template() {
        let x1 = RangeXxx::unit(2);
        let x2 = RangeXxx::unit(3);

        assert_eq!(
            RangeXxxMonoid::binary_operation(&x1, &x2),
            RangeXxx { len: 2 }
        );

        assert_eq!(RangeYyyRangeXxx::mapping(&(), &x1), RangeXxx { len: 1 });
    }
}
