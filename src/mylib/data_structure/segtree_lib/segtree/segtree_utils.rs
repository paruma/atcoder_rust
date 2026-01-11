use cargo_snippet::snippet;

#[snippet]
pub fn segtree_to_vec<M: ac_library::Monoid>(seg: &ac_library::Segtree<M>, len: usize) -> Vec<M::S> {
    (0..len).map(|i| seg.get(i)).collect()
}

// セグ木用の Monoidテンプレート
#[allow(unused_variables)]
#[snippet(prefix = "use monoid_template::*;")]
pub mod monoid_template {
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
}

#[cfg(test)]
mod test_segtree_to_vec {
    use ac_library::{Additive, Segtree};
    use super::segtree_to_vec;

    #[test]
    fn test_segtree_to_vec() {
        let seg = Segtree::<Additive<i64>>::from(vec![1, 2, 3]);
        assert_eq!(segtree_to_vec(&seg, 3), vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod test_monoid_template {
    use ac_library::Monoid;
    use super::monoid_template::*;

    #[test]
    fn test_map_monoid_template() {
        let x1 = RangeXxx::unit(2);
        let x2 = RangeXxx::unit(3);

        assert_eq!(
            RangeXxxMonoid::binary_operation(&x1, &x2),
            RangeXxx { len: 2 }
        );
    }
}
