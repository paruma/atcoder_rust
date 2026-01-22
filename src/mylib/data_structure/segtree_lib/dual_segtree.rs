use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use dual_segtree::*;")]
pub mod dual_segtree {
    use std::ops::{Bound, RangeBounds};

    fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }

    pub trait MapMonoid {
        type F: Clone;
        type S: Clone;
        fn identity_map() -> Self::F;
        fn mapping(f: &Self::F, x: &Self::S) -> Self::S;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }

    impl<F: MapMonoid> Default for DualSegtree<F>
    where
        F::S: Default,
    {
        fn default() -> Self {
            Self::new(0)
        }
    }
    impl<F: MapMonoid> DualSegtree<F> {
        pub fn new(n: usize) -> Self
        where
            F::S: Default,
        {
            vec![F::S::default(); n].into()
        }
    }

    impl<F: MapMonoid> From<Vec<F::S>> for DualSegtree<F>
    where
        F::S: Default,
    {
        fn from(v: Vec<F::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![F::S::default(); size];
            let lz = vec![F::identity_map(); size];
            d[..n].clone_from_slice(&v);
            DualSegtree {
                n,
                size,
                log,
                d,
                lz,
            }
        }
    }

    impl<F: MapMonoid> DualSegtree<F> {
        pub fn set(&mut self, p: usize, x: F::S) {
            assert!(p < self.n);
            for i in (1..=self.log).rev() {
                self.push((p + self.size) >> i);
            }
            self.d[p] = x;
        }

        pub fn get(&mut self, p: usize) -> F::S {
            assert!(p < self.n);
            for i in (1..=self.log).rev() {
                self.push((p + self.size) >> i);
            }
            self.d[p].clone()
        }

        pub fn apply(&mut self, p: usize, f: F::F) {
            assert!(p < self.n);
            for i in (1..=self.log).rev() {
                self.push((p + self.size) >> i);
            }
            self.d[p] = F::mapping(&f, &self.d[p]);
        }
        pub fn apply_range<R>(&mut self, range: R, f: F::F)
        where
            R: RangeBounds<usize>,
        {
            let mut r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let mut l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                // TODO: There are another way of optimizing [0..r)
                Bound::Unbounded => 0,
            };

            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }

            {
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
            }
        }

        pub fn to_vec(&mut self) -> Vec<F::S> {
            (0..self.n).map(|i| self.get(i)).collect()
        }
    }

    #[derive(Clone)]
    pub struct DualSegtree<F>
    where
        F: MapMonoid,
    {
        n: usize,
        size: usize,
        log: usize,
        d: Vec<F::S>,
        lz: Vec<F::F>,
    }
    impl<F> DualSegtree<F>
    where
        F: MapMonoid,
    {
        fn all_apply(&mut self, k: usize, f: F::F) {
            if k < self.size {
                self.lz[k] = F::composition(&f, &self.lz[k]);
            } else {
                self.d[k - self.size] = F::mapping(&f, &self.d[k - self.size]);
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lz[k].clone());
            self.all_apply(2 * k + 1, self.lz[k].clone());
            self.lz[k] = F::identity_map();
        }
    }
}

#[snippet(prefix = "use range_affine_dual_segtree::*;")]
pub mod range_affine_dual_segtree {
    use super::dual_segtree::*;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Affine<T> {
        pub slope: T,
        pub intercept: T,
    }

    impl<T> Affine<T> {
        pub fn new(slope: T, intercept: T) -> Affine<T> {
            Affine { slope, intercept }
        }
    }

    impl<T> Affine<T>
    where
        T: From<i64>,
    {
        /// 区間変更用（定数関数）
        pub fn constant_func(x: T) -> Affine<T> {
            Affine {
                slope: 0.into(),
                intercept: x,
            }
        }
        /// 区間加算用
        pub fn addition_func(x: T) -> Affine<T> {
            Affine {
                slope: 1.into(),
                intercept: x,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AffineMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for AffineMonoid<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type F = Affine<T>;
        type S = T;
        fn identity_map() -> Affine<T> {
            Affine {
                slope: 1.into(),
                intercept: 0.into(),
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
}

#[snippet(prefix = "use range_add_dual_segtree::*;")]
pub mod range_add_dual_segtree {
    use super::dual_segtree::*;
    use std::convert::Infallible;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AddMonoid(Infallible);
    impl MapMonoid for AddMonoid {
        type F = i64;
        type S = i64;

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(&f: &i64, &x: &i64) -> i64 {
            f + x
        }

        fn composition(&f: &i64, &g: &i64) -> i64 {
            f + g
        }
    }
}

#[cfg(test)]
mod tests_dual_segtree {
    use super::dual_segtree::DualSegtree;
    use super::range_add_dual_segtree::AddMonoid;

    #[test]
    fn test_new() {
        let mut segtree = DualSegtree::<AddMonoid>::new(10);
        assert_eq!(segtree.to_vec(), vec![0; 10]);
    }

    #[test]
    fn test_default() {
        let mut segtree = DualSegtree::<AddMonoid>::default();
        assert_eq!(segtree.to_vec(), Vec::<i64>::new());
    }

    #[test]
    fn test_apply_range_patterns() {
        let n = 10;
        let mut segtree = DualSegtree::<AddMonoid>::new(n);
        let mut expected = vec![0; n];

        // 1. ..
        segtree.apply_range(.., 1);
        expected.iter_mut().for_each(|x| *x += 1);
        assert_eq!(segtree.to_vec(), expected);

        // 2. l..r
        segtree.apply_range(2..5, 10);
        (2..5).for_each(|i| expected[i] += 10);
        assert_eq!(segtree.to_vec(), expected);

        // 3. l..=r
        segtree.apply_range(4..=6, 100);
        (4..=6).for_each(|i| expected[i] += 100);
        assert_eq!(segtree.to_vec(), expected);

        // 4. ..r
        segtree.apply_range(..2, 1000);
        (0..2).for_each(|i| expected[i] += 1000);
        assert_eq!(segtree.to_vec(), expected);

        // 5. ..=r
        segtree.apply_range(..=1, 10000);
        (0..=1).for_each(|i| expected[i] += 10000);
        assert_eq!(segtree.to_vec(), expected);

        // 6. l..
        segtree.apply_range(8.., 50);
        (8..10).for_each(|i| expected[i] += 50);
        assert_eq!(segtree.to_vec(), expected);

        // 7. Bound::Excluded for start
        use std::ops::Bound;
        segtree.apply_range((Bound::Excluded(1), Bound::Excluded(4)), 100000);
        (2..4).for_each(|i| expected[i] += 100000);
        assert_eq!(segtree.to_vec(), expected);
    }

    #[test]
    #[ignore]
    fn test_random_dual_segtree() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=30);
            let mut naive = vec![0i64; n];
            let mut seg = DualSegtree::<AddMonoid>::new(n);

            for _ in 0..100 {
                let op = rng.random_range(0..4);
                if op == 0 {
                    // set
                    let p = rng.random_range(0..n);
                    let x = rng.random_range(-100..=100);
                    naive[p] = x;
                    seg.set(p, x);
                } else if op == 1 {
                    // get
                    let p = rng.random_range(0..n);
                    assert_eq!(seg.get(p), naive[p]);
                } else if op == 2 {
                    // apply
                    let p = rng.random_range(0..n);
                    let f = rng.random_range(-100..=100);
                    naive[p] += f;
                    seg.apply(p, f);
                } else {
                    // apply_range
                    let l = rng.random_range(0..=n);
                    let r = rng.random_range(l..=n);
                    let f = rng.random_range(-100..=100);
                    for i in l..r {
                        naive[i] += f;
                    }
                    seg.apply_range(l..r, f);
                }
            }
            assert_eq!(seg.to_vec(), naive);
        }
    }

    // ACL の lazysegtree のテストを流用したもの
    #[test]
    fn test_range_add_dual_segtree() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let n = base.len();
        let mut segtree: DualSegtree<AddMonoid> = base.clone().into();
        check_segtree(&base, &mut segtree);

        let mut internal = vec![i64::MIN; n];
        let mut segtree = DualSegtree::<AddMonoid>::from(internal.clone());

        for i in 0..n {
            segtree.set(i, base[i]);
            internal[i] = base[i];
            check_segtree(&internal, &mut segtree);
        }

        segtree.set(6, 5);
        internal[6] = 5;
        check_segtree(&internal, &mut segtree);

        segtree.apply(5, 1);
        internal[5] += 1;
        check_segtree(&internal, &mut segtree);

        segtree.set(6, 0);
        internal[6] = 0;
        check_segtree(&internal, &mut segtree);

        segtree.apply_range(3..8, 2);
        internal[3..8].iter_mut().for_each(|e| *e += 2);
        check_segtree(&internal, &mut segtree);

        segtree.apply_range(2..=5, 7);
        internal[2..=5].iter_mut().for_each(|e| *e += 7);
        check_segtree(&internal, &mut segtree);
    }

    //noinspection DuplicatedCode
    fn check_segtree(base: &[i64], segtree: &mut DualSegtree<AddMonoid>) {
        let n = base.len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            assert_eq!(segtree.get(i), base[i]);
        }
        assert_eq!(segtree.to_vec(), base);
    }
}

#[cfg(test)]
pub mod test_range_affine {
    use crate::data_structure::segtree_lib::dual_segtree::dual_segtree::MapMonoid;

    use super::range_affine_dual_segtree::{Affine, AffineMonoid};

    #[test]
    fn test_affine_addition_func() {
        let f = Affine::addition_func(5_i64);
        assert_eq!(AffineMonoid::mapping(&f, &0), 5);
        assert_eq!(AffineMonoid::mapping(&f, &3), 8);
    }

    #[test]
    fn test_affine_constant_func() {
        let f = Affine::constant_func(5_i64);
        assert_eq!(AffineMonoid::mapping(&f, &0), 5);
        assert_eq!(AffineMonoid::mapping(&f, &3), 5);
        assert_eq!(AffineMonoid::mapping(&f, &5), 5);
        assert_eq!(AffineMonoid::mapping(&f, &10), 5);
    }

    #[test]
    fn test_affine() {
        let f = Affine::new(-2_i64, 1);
        assert_eq!(AffineMonoid::mapping(&f, &0), 1);
        assert_eq!(AffineMonoid::mapping(&f, &3), -5);
        assert_eq!(AffineMonoid::mapping(&f, &5), -9);
    }
    #[test]
    fn test_affine_composition() {
        let f1 = Affine {
            slope: 3_i64,
            intercept: 5,
        };

        let f2 = Affine {
            slope: 5,
            intercept: 2,
        };

        let f3 = Affine {
            slope: 0,
            intercept: 2,
        };

        // 3(5x + 2) + 5 = 15x + 11
        assert_eq!(
            AffineMonoid::composition(&f1, &f2),
            Affine {
                slope: 15,
                intercept: 11
            }
        );

        // 3*(0x + 2) + 5 = 11
        assert_eq!(
            AffineMonoid::composition(&f1, &f3),
            Affine {
                slope: 0,
                intercept: 11
            }
        );

        // 0(3x + 5) + 2 = 2
        assert_eq!(
            AffineMonoid::composition(&f3, &f1),
            Affine {
                slope: 0,
                intercept: 2
            }
        );
    }
}
