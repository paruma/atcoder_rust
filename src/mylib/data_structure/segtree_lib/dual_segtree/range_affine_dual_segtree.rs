use crate::data_structure::segtree_lib::dual_segtree::dual_segtree::dual_segtree::{
    DualSegtree, MapMonoid,
};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_affine_dual_segtree::*;", include = "dual_segtree")]
pub mod range_affine_dual_segtree {
    use super::{DualSegtree, MapMonoid};
    use std::convert::Infallible;
    use std::iter::{Product, Sum};
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }
    fn one<T: Product>() -> T {
        std::iter::empty::<T>().product()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Affine<T> {
        pub slope: T,
        pub intercept: T,
    }

    impl<T> Affine<T>
    where
        T: Sum + Product,
    {
        pub fn new(slope: T, intercept: T) -> Affine<T> {
            Affine { slope, intercept }
        }

        /// 区間変更用（定数関数）
        pub fn constant_func(x: T) -> Affine<T> {
            Affine {
                slope: zero(),
                intercept: x,
            }
        }

        /// 区間加算用
        pub fn addition_func(x: T) -> Affine<T> {
            Affine {
                slope: one(),
                intercept: x,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AffineMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for AffineMonoid<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + Sum + Product,
    {
        type F = Affine<T>;
        type S = T;
        fn identity_map() -> Affine<T> {
            Affine {
                slope: one(),
                intercept: zero(),
            }
        }
        fn composition(a: &Affine<T>, b: &Affine<T>) -> Affine<T> {
            Affine {
                slope: a.slope * b.slope,
                intercept: a.slope * b.intercept + a.intercept,
            }
        }
        fn mapping(f: &Affine<T>, x: &T) -> T {
            f.slope * *x + f.intercept
        }
    }

    #[derive(Clone)]
    pub struct RangeAffineDualSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + Sum + Product + Default,
    {
        segtree: DualSegtree<AffineMonoid<T>>,
    }

    impl<T> RangeAffineDualSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + Sum + Product + Default,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![zero(); n];
            RangeAffineDualSegtree {
                segtree: DualSegtree::from(xs),
            }
        }

        pub fn from_slice(xs: &[T]) -> Self {
            RangeAffineDualSegtree {
                segtree: DualSegtree::from(xs.to_vec()),
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }

        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p)
        }

        pub fn affine(&mut self, p: usize, slope: T, intercept: T) {
            self.segtree.apply(p, Affine { slope, intercept });
        }

        pub fn update(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::constant_func(x));
        }

        pub fn add(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::addition_func(x));
        }

        pub fn range_affine<R>(&mut self, range: R, slope: T, intercept: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine { slope, intercept });
        }

        pub fn range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::constant_func(x));
        }

        pub fn range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::addition_func(x));
        }

        pub fn to_vec(&mut self) -> Vec<T> {
            self.segtree.to_vec()
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.segtree.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_affine_dual_segtree::*;

    #[test]
    fn test_affine_new() {
        let f = Affine::new(2, 3);
        assert_eq!(f.slope, 2);
        assert_eq!(f.intercept, 3);
    }

    #[test]
    fn test_range_affine_dual_segtree_basic() {
        let initial = vec![1, 2, 3];
        let mut seg = RangeAffineDualSegtree::<i64>::from_slice(&initial);
        assert_eq!(seg.len(), 3);
        assert_eq!(seg.to_vec(), initial);

        let n = 10;
        let mut seg = RangeAffineDualSegtree::<i64>::new(n);
        assert_eq!(seg.len(), n);
        assert_eq!(seg.to_vec(), vec![0; n]);

        seg.range_add(2..5, 10);
        let mut expected = vec![0, 0, 10, 10, 10, 0, 0, 0, 0, 0];
        assert_eq!(seg.to_vec(), expected);

        seg.range_update(4..7, 5);
        expected[4] = 5;
        expected[5] = 5;
        expected[6] = 5;
        assert_eq!(seg.to_vec(), expected);

        // test point update/add
        seg.update(0, 100);
        expected[0] = 100;
        seg.add(1, 50);
        expected[1] = 50;
        assert_eq!(seg.to_vec(), expected);

        // test affine (point)
        seg.affine(1, 2, 3);
        expected[1] = expected[1] * 2 + 3;
        assert_eq!(seg.get(1), expected[1]);

        // test range full
        seg.range_affine(.., 2, 1);
        for x in expected.iter_mut() {
            *x = *x * 2 + 1;
        }
        assert_eq!(seg.to_vec(), expected);

        // test other bounds (Excluded, etc.)
        use std::ops::Bound;
        seg.range_affine((Bound::Excluded(1), Bound::Excluded(4)), 1, 10);
        expected[2] += 10;
        expected[3] += 10;
        assert_eq!(seg.to_vec(), expected);
    }

    #[test]
    fn test_random_affine_dual() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..50 {
            let n = rng.random_range(1..=30);
            let mut naive = vec![0i64; n];
            let mut seg = RangeAffineDualSegtree::<i64>::new(n);

            for _ in 0..100 {
                let op = rng.random_range(0..7);
                match op {
                    0 => {
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive[p] = x;
                        seg.set(p, x);
                    }
                    1 => {
                        let p = rng.random_range(0..n);
                        assert_eq!(seg.get(p), naive[p]);
                    }
                    2 => {
                        let p = rng.random_range(0..n);
                        let slope = rng.random_range(-2..=2);
                        let intercept = rng.random_range(-50..=50);
                        naive[p] = naive[p] * slope + intercept;
                        seg.affine(p, slope, intercept);
                    }
                    3 => {
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let slope = rng.random_range(-2..=2);
                        let intercept = rng.random_range(-50..=50);
                        for i in l..r {
                            naive[i] = naive[i] * slope + intercept;
                        }
                        seg.range_affine(l..r, slope, intercept);
                    }
                    4 => {
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = rng.random_range(-100..=100);
                        for i in l..r {
                            naive[i] = x;
                        }
                        seg.range_update(l..r, x);
                    }
                    5 => {
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = rng.random_range(-50..=50);
                        for i in l..r {
                            naive[i] += x;
                        }
                        seg.range_add(l..r, x);
                    }
                    _ => {
                        assert_eq!(seg.to_vec(), naive);
                    }
                }
            }
        }
    }
}
