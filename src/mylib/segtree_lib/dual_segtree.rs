#[allow(clippy::module_inception)]
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
    }

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

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::dual_segtree::{DualSegtree, MapMonoid};

    struct RangeAdd(Infallible);
    impl MapMonoid for RangeAdd {
        type F = i32;
        type S = i32;

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(&f: &i32, &x: &i32) -> i32 {
            f + x
        }

        fn composition(&f: &i32, &g: &i32) -> i32 {
            f + g
        }
    }

    #[test]
    fn test_range_add_dual_segtree() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let n = base.len();
        let mut segtree: DualSegtree<RangeAdd> = base.clone().into();
        check_segtree(&base, &mut segtree);

        let mut internal = vec![i32::min_value(); n];
        let mut segtree = DualSegtree::<RangeAdd>::from(internal.clone());

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
    fn check_segtree(base: &[i32], segtree: &mut DualSegtree<RangeAdd>) {
        let n = base.len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            assert_eq!(segtree.get(i), base[i]);
        }
    }
}
