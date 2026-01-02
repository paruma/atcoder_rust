use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use dynamic_monoid_segtree::*;")]
pub mod dynamic_monoid_segtree {
    use std::ops::{Bound, RangeBounds};

    fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }

    pub trait DynamicMonoid {
        type S: Clone;
        fn identity(&self) -> Self::S;
        fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S;
    }

    impl<M: DynamicMonoid> DynamicMonoidSegtree<M> {
        pub fn new(n: usize, m: M) -> DynamicMonoidSegtree<M> {
            Self::from_vec(vec![m.identity(); n], m)
        }

        fn from_vec(v: Vec<M::S>, m: M) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![m.identity(); 2 * size];
            d[size..(size + n)].clone_from_slice(&v);
            let mut ret = DynamicMonoidSegtree { n, size, log, d, m };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }

    impl<M: DynamicMonoid> DynamicMonoidSegtree<M> {
        pub fn set(&mut self, mut p: usize, x: M::S) {
            assert!(p < self.n);
            p += self.size;
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn get(&self, p: usize) -> M::S {
            assert!(p < self.n);
            self.d[p + self.size].clone()
        }

        pub fn prod<R>(&self, range: R) -> M::S
        where
            R: RangeBounds<usize>,
        {
            // Trivial optimization
            if range.start_bound() == Bound::Unbounded && range.end_bound() == Bound::Unbounded {
                return self.all_prod();
            }

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
            let mut sml = self.m.identity();
            let mut smr = self.m.identity();
            l += self.size;
            r += self.size;

            while l < r {
                if l & 1 != 0 {
                    sml = self.m.binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = self.m.binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            self.m.binary_operation(&sml, &smr)
        }

        pub fn all_prod(&self) -> M::S {
            self.d[1].clone()
        }

        pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(f(&self.m.identity()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            let mut sm = self.m.identity();
            while {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !f(&self.m.binary_operation(&sm, &self.d[l])) {
                    while l < self.size {
                        l *= 2;
                        let res = self.m.binary_operation(&sm, &self.d[l]);
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = self.m.binary_operation(&sm, &self.d[l]);
                l += 1;
                // while
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }

        pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(f(&self.m.identity()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            let mut sm = self.m.identity();
            while {
                // do
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }
                if !f(&self.m.binary_operation(&self.d[r], &sm)) {
                    while r < self.size {
                        r = 2 * r + 1;
                        let res = self.m.binary_operation(&self.d[r], &sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = self.m.binary_operation(&self.d[r], &sm);
                // while
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }

        fn update(&mut self, k: usize) {
            self.d[k] = self.m.binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
    }

    pub struct DynamicMonoidSegtree<M>
    where
        M: DynamicMonoid,
    {
        // variable name is _n in original library
        n: usize,
        size: usize,
        log: usize,
        d: Vec<M::S>,
        m: M,
    }
}

#[cfg(test)]
mod tests_dynamic_monoid_segtree {

    use super::dynamic_monoid_segtree::{DynamicMonoid, DynamicMonoidSegtree};
    use std::ops::Bound::{Excluded, Included};
    use std::ops::RangeBounds;

    struct VecAddMonoid {
        len: usize,
    }

    impl DynamicMonoid for VecAddMonoid {
        type S = Vec<i64>;

        fn identity(&self) -> Self::S {
            vec![0; self.len]
        }

        fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S {
            assert_eq!(a.len(), self.len);
            assert_eq!(b.len(), self.len);
            a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
        }
    }

    #[test]
    fn test_vec_add_segtree() {
        let base_len = 3;
        let base = [
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];
        let n = base.len();
        let monoid = VecAddMonoid { len: base_len };
        let segtree: DynamicMonoidSegtree<VecAddMonoid> = DynamicMonoidSegtree::new(n, monoid);

        // Initial check with identity elements
        check_segtree(&vec![vec![0; base_len]; n], &segtree, base_len);

        let mut segtree = DynamicMonoidSegtree::new(n, VecAddMonoid { len: base_len });
        let mut internal = vec![vec![0; base_len]; n];
        for i in 0..n {
            segtree.set(i, base[i].clone());
            internal[i] = base[i].clone();
            check_segtree(&internal, &segtree, base_len);
        }

        segtree.set(2, vec![100, 200, 300]);
        internal[2] = vec![100, 200, 300];
        check_segtree(&internal, &segtree, base_len);

        segtree.set(0, vec![0, 0, 0]);
        internal[0] = vec![0, 0, 0];
        check_segtree(&internal, &segtree, base_len);
    }

    fn check_segtree(
        base: &[Vec<i64>],
        segtree: &DynamicMonoidSegtree<VecAddMonoid>,
        base_len: usize,
    ) {
        let n = base.len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            assert_eq!(segtree.get(i), base[i]);
        }

        check(base, segtree, .., base_len);
        for i in 0..=n {
            check(base, segtree, ..i, base_len);
            check(base, segtree, i.., base_len);
            if i < n {
                check(base, segtree, ..=i, base_len);
            }
            for j in i..=n {
                check(base, segtree, i..j, base_len);
                if j < n {
                    check(base, segtree, i..=j, base_len);
                    check(base, segtree, (Excluded(i), Included(j)), base_len);
                }
            }
        }
        let expected_all_prod = base.iter().fold(vec![0; base_len], |acc, x| {
            acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect()
        });
        assert_eq!(segtree.all_prod(), expected_all_prod);

        // max_right and min_left are tricky with VecAddMonoid as there's no clear "order" or "threshold" for a vector.
        // The original example uses `f: |&x: &i32| x < k;` which implies a scalar comparison.
        // For Vec<i64>, we need a different kind of predicate.
        // For simplicity, let's implement a basic check for max_right/min_left that makes sense for VecAddMonoid.
        // For example, check if the sum of elements in the vector is less than a threshold.

        // Example for max_right: find the rightmost index `j` such that the sum of elements in `base[l..j]` is less than `k`.
        for k_sum in 1..=100 {
            // Iterate through possible sum thresholds
            let f = |x: &Vec<i64>| x.iter().sum::<i64>() < k_sum;
            for i in 0..=n {
                let mut expected_max_right = i;
                let mut current_sum_vec = vec![0; base_len];
                for j in i..n {
                    let next_sum_vec: Vec<i64> = current_sum_vec
                        .iter()
                        .zip(base[j].iter())
                        .map(|(a, b)| a + b)
                        .collect();
                    if f(&next_sum_vec) {
                        current_sum_vec = next_sum_vec;
                        expected_max_right = j + 1;
                    } else {
                        break;
                    }
                }
                assert_eq!(segtree.max_right(i, f), expected_max_right);
            }
        }

        // Example for min_left: find the leftmost index `j` such that the sum of elements in `base[j..r]` is less than `k`.
        for k_sum in 1..=100 {
            // Iterate through possible sum thresholds
            let f = |x: &Vec<i64>| x.iter().sum::<i64>() < k_sum;
            for r in 0..=n {
                let mut expected_min_left = r;
                let mut current_sum_vec = vec![0; base_len];
                for j in (0..r).rev() {
                    let next_sum_vec: Vec<i64> = current_sum_vec
                        .iter()
                        .zip(base[j].iter())
                        .map(|(a, b)| a + b)
                        .collect();
                    if f(&next_sum_vec) {
                        current_sum_vec = next_sum_vec;
                        expected_min_left = j;
                    } else {
                        break;
                    }
                }
                assert_eq!(segtree.min_left(r, f), expected_min_left);
            }
        }
    }

    fn check(
        base: &[Vec<i64>],
        segtree: &DynamicMonoidSegtree<VecAddMonoid>,
        range: impl RangeBounds<usize>,
        base_len: usize,
    ) {
        let expected = base
            .iter()
            .enumerate()
            .filter_map(|(i, a)| Some(a).filter(|_| range.contains(&i)))
            .fold(vec![0; base_len], |acc, x| {
                acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect()
            });
        assert_eq!(segtree.prod(range), expected);
    }
}
