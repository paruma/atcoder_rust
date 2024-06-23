use cargo_snippet::snippet;

#[snippet(prefix = "use abstract_segtree_beats::*;")]
pub mod abstract_segtree_beats {
    // 参考: atcoder::lazy_segtree に1行書き足すだけの抽象化 Segment Tree Beats - ひとなので
    // https://rsm9.hatenablog.com/entry/2021/02/01/220408

    fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }
    pub trait MonoidBeats {
        type S: Clone;
        fn identity() -> Self::S;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
        fn fails(a: &Self::S) -> bool;
    }

    pub trait MapMonoidBeats {
        type M: MonoidBeats;
        type F: Clone;
        // type S = <Self::M as Monoid>::S;
        fn identity_element() -> <Self::M as MonoidBeats>::S {
            Self::M::identity()
        }
        fn binary_operation(
            a: &<Self::M as MonoidBeats>::S,
            b: &<Self::M as MonoidBeats>::S,
        ) -> <Self::M as MonoidBeats>::S {
            Self::M::binary_operation(a, b)
        }

        fn fails(a: &<Self::M as MonoidBeats>::S) -> bool {
            Self::M::fails(a)
        }
        fn identity_map() -> Self::F;
        fn mapping(f: &Self::F, x: &<Self::M as MonoidBeats>::S) -> <Self::M as MonoidBeats>::S;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }

    impl<F: MapMonoidBeats> Default for SegtreeBeats<F> {
        fn default() -> Self {
            Self::new(0)
        }
    }
    impl<F: MapMonoidBeats> SegtreeBeats<F> {
        pub fn new(n: usize) -> Self {
            vec![F::identity_element(); n].into()
        }
    }
    impl<F: MapMonoidBeats> From<Vec<<F::M as MonoidBeats>::S>> for SegtreeBeats<F> {
        fn from(v: Vec<<F::M as MonoidBeats>::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![F::identity_element(); 2 * size];
            let lz = vec![F::identity_map(); size];
            d[size..(size + n)].clone_from_slice(&v);
            let mut ret = SegtreeBeats {
                n,
                size,
                log,
                d,
                lz,
            };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }

    impl<F: MapMonoidBeats> SegtreeBeats<F> {
        pub fn set(&mut self, mut p: usize, x: <F::M as MonoidBeats>::S) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn get(&mut self, mut p: usize) -> <F::M as MonoidBeats>::S {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p].clone()
        }

        pub fn prod<R>(&mut self, range: R) -> <F::M as MonoidBeats>::S
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
            if l == r {
                return F::identity_element();
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }

            let mut sml = F::identity_element();
            let mut smr = F::identity_element();
            while l < r {
                if l & 1 != 0 {
                    sml = F::binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = F::binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            F::binary_operation(&sml, &smr)
        }

        pub fn all_prod(&self) -> <F::M as MonoidBeats>::S {
            self.d[1].clone()
        }

        pub fn apply(&mut self, mut p: usize, f: F::F) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = F::mapping(&f, &self.d[p]);
            for i in 1..=self.log {
                self.update(p >> i);
            }
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
                let l2 = l;
                let r2 = r;
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
                l = l2;
                r = r2;
            }

            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }

        pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
        where
            G: Fn(<F::M as MonoidBeats>::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(g(F::identity_element()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            for i in (1..=self.log).rev() {
                self.push(l >> i);
            }
            let mut sm = F::identity_element();
            while {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !g(F::binary_operation(&sm, &self.d[l])) {
                    while l < self.size {
                        self.push(l);
                        l *= 2;
                        let res = F::binary_operation(&sm, &self.d[l]);
                        if g(res.clone()) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = F::binary_operation(&sm, &self.d[l]);
                l += 1;
                //while
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }

        pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
        where
            G: Fn(<F::M as MonoidBeats>::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(g(F::identity_element()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            for i in (1..=self.log).rev() {
                self.push((r - 1) >> i);
            }
            let mut sm = F::identity_element();
            while {
                // do
                r -= 1;
                while r > 1 && r % 2 != 0 {
                    r >>= 1;
                }
                if !g(F::binary_operation(&self.d[r], &sm)) {
                    while r < self.size {
                        self.push(r);
                        r = 2 * r + 1;
                        let res = F::binary_operation(&self.d[r], &sm);
                        if g(res.clone()) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = F::binary_operation(&self.d[r], &sm);
                // while
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }
    }

    pub struct SegtreeBeats<F>
    where
        F: MapMonoidBeats,
    {
        n: usize,
        size: usize,
        log: usize,
        d: Vec<<F::M as MonoidBeats>::S>,
        lz: Vec<F::F>,
    }
    impl<F> SegtreeBeats<F>
    where
        F: MapMonoidBeats,
    {
        fn update(&mut self, k: usize) {
            self.d[k] = F::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
        fn all_apply(&mut self, k: usize, f: F::F) {
            self.d[k] = F::mapping(&f, &self.d[k]);
            if k < self.size {
                self.lz[k] = F::composition(&f, &self.lz[k]);

                // ACL の LazySegtree からの追加分
                // データの計算に失敗した場合は、遅延させていた作用を伝搬させていき、下から上にデータの計算を行う。
                if F::fails(&self.d[k]) {
                    self.push(k);
                    self.update(k)
                }
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lz[k].clone());
            self.all_apply(2 * k + 1, self.lz[k].clone());
            self.lz[k] = F::identity_map();
        }
    }

    // TODO is it useful?
    use std::{
        fmt::{Debug, Error, Formatter, Write},
        ops::{Bound, RangeBounds},
    };

    impl<F> Debug for SegtreeBeats<F>
    where
        F: MapMonoidBeats,
        F::F: Debug,
        <F::M as MonoidBeats>::S: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            for i in 0..self.log {
                for j in 0..1 << i {
                    f.write_fmt(format_args!(
                        "{:?}[{:?}]\t",
                        self.d[(1 << i) + j],
                        self.lz[(1 << i) + j]
                    ))?;
                }
                f.write_char('\n')?;
            }
            for i in 0..self.size {
                f.write_fmt(format_args!("{:?}\t", self.d[self.size + i]))?;
            }
            Ok(())
        }
    }
}

#[snippet(
    prefix = "use range_chmax_range_sum::*;",
    include = "abstract_segtree_beats"
)]
pub mod range_chmax_range_sum {
    use std::{
        cmp::{max, min},
        convert::Infallible,
    };

    use super::abstract_segtree_beats::{MapMonoidBeats, MonoidBeats};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeSum {
        pub sum: i64,
        pub len: usize,
        pub min: i64,
        pub min_cnt: usize,
        pub min_2nd: i64,
    }

    impl RangeSum {
        pub fn unit(x: i64) -> Option<RangeSum> {
            Some(RangeSum {
                sum: x,
                len: 1,
                min: x,
                min_cnt: 1,
                min_2nd: i64::MAX,
            })
        }
    }

    fn second_smallest(a0: i64, a1: i64, b0: i64, b1: i64) -> i64 {
        // a0 < a1, b0 < b1 のとき、{a0, a1, b0, b1} で2番目に小さい値
        if a0 == b0 {
            min(a1, b1)
        } else if a1 <= b0 {
            a1
        } else if b1 <= a0 {
            b1
        } else {
            max(a0, b0)
        }
    }

    pub struct RangeSumMonoid(Infallible);
    impl MonoidBeats for RangeSumMonoid {
        type S = Option<RangeSum>;

        fn identity() -> Self::S {
            Some(RangeSum {
                sum: 0,
                len: 0,
                min: i64::MAX,
                min_cnt: 0,
                min_2nd: i64::MAX,
            })
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            match (a, b) {
                (None, None) => None,
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (Some(a), Some(b)) => Some(RangeSum {
                    sum: a.sum + b.sum,
                    len: a.len + b.len,
                    min: min(a.min, b.min),
                    min_cnt: a.min_cnt * (a.min <= b.min) as usize
                        + b.min_cnt * (b.min <= a.min) as usize,
                    min_2nd: second_smallest(a.min, a.min_2nd, b.min, b.min_2nd),
                }),
            }
        }

        fn fails(a: &Self::S) -> bool {
            a.is_none()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ChmaxFunc {
        pub chmax_val: i64,
    }

    impl ChmaxFunc {
        pub fn new(x: i64) -> Self {
            ChmaxFunc { chmax_val: x }
        }
    }

    pub struct RangeChmaxRangeSum(Infallible);

    impl MapMonoidBeats for RangeChmaxRangeSum {
        type F = ChmaxFunc;
        type M = RangeSumMonoid;

        fn identity_map() -> Self::F {
            ChmaxFunc {
                chmax_val: i64::MIN,
            }
        }

        #[allow(clippy::if_same_then_else)]
        fn mapping(f: &Self::F, x: &<Self::M as MonoidBeats>::S) -> <Self::M as MonoidBeats>::S {
            match x {
                None => None,
                Some(x) => {
                    if x.len == 0 {
                        Some(*x)
                    } else if f.chmax_val <= x.min {
                        Some(*x)
                    } else if f.chmax_val < x.min_2nd {
                        Some(RangeSum {
                            sum: x.sum + (f.chmax_val - x.min) * x.min_cnt as i64,
                            len: x.len,
                            min: f.chmax_val,
                            min_cnt: x.min_cnt,
                            min_2nd: x.min_2nd,
                        })
                    } else {
                        // 計算失敗
                        None
                    }
                }
            }
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            ChmaxFunc {
                chmax_val: max(f.chmax_val, g.chmax_val),
            }
        }
    }
}

#[snippet(
    prefix = "use range_chmin_range_sum::*;",
    include = "abstract_segtree_beats"
)]
pub mod range_chmin_range_sum {
    use std::{
        cmp::{max, min},
        convert::Infallible,
    };

    use super::abstract_segtree_beats::{MapMonoidBeats, MonoidBeats};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeSum {
        pub sum: i64,
        pub len: usize,
        pub max: i64,
        pub max_cnt: usize,
        pub max_2nd: i64,
    }

    impl RangeSum {
        pub fn unit(x: i64) -> Option<RangeSum> {
            Some(RangeSum {
                sum: x,
                len: 1,
                max: x,
                max_cnt: 1,
                max_2nd: i64::MIN,
            })
        }
    }

    fn second_largest(a0: i64, a1: i64, b0: i64, b1: i64) -> i64 {
        // a0 > a1, b0 > b1 のとき、{a0, a1, b0, b1} で2番目に大きい値
        if a0 == b0 {
            max(a1, b1)
        } else if a1 >= b0 {
            a1
        } else if b1 >= a0 {
            b1
        } else {
            min(a0, b0)
        }
    }

    pub struct RangeSumMonoid(Infallible);
    impl MonoidBeats for RangeSumMonoid {
        type S = Option<RangeSum>;

        fn identity() -> Self::S {
            Some(RangeSum {
                sum: 0,
                len: 0,
                max: i64::MIN,
                max_cnt: 0,
                max_2nd: i64::MIN,
            })
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            match (a, b) {
                (None, None) => None,
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (Some(a), Some(b)) => Some(RangeSum {
                    sum: a.sum + b.sum,
                    len: a.len + b.len,
                    max: max(a.max, b.max),
                    max_cnt: a.max_cnt * (a.max >= b.max) as usize
                        + b.max_cnt * (b.max >= a.max) as usize,
                    max_2nd: second_largest(a.max, a.max_2nd, b.max, b.max_2nd),
                }),
            }
        }

        fn fails(a: &Self::S) -> bool {
            a.is_none()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ChminFunc {
        pub chmin_val: i64,
    }

    impl ChminFunc {
        pub fn new(x: i64) -> Self {
            ChminFunc { chmin_val: x }
        }
    }

    pub struct RangeChminRangeSum(Infallible);

    impl MapMonoidBeats for RangeChminRangeSum {
        type F = ChminFunc;
        type M = RangeSumMonoid;

        fn identity_map() -> Self::F {
            ChminFunc {
                chmin_val: i64::MAX,
            }
        }

        #[allow(clippy::if_same_then_else)]
        fn mapping(f: &Self::F, x: &<Self::M as MonoidBeats>::S) -> <Self::M as MonoidBeats>::S {
            match x {
                None => None,
                Some(x) => {
                    if x.len == 0 {
                        Some(*x)
                    } else if f.chmin_val >= x.max {
                        Some(*x)
                    } else if f.chmin_val > x.max_2nd {
                        Some(RangeSum {
                            sum: x.sum + (f.chmin_val - x.max) * x.max_cnt as i64,
                            len: x.len,
                            max: f.chmin_val,
                            max_cnt: x.max_cnt,
                            max_2nd: x.max_2nd,
                        })
                    } else {
                        // 計算失敗
                        None
                    }
                }
            }
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            ChminFunc {
                chmin_val: min(f.chmin_val, g.chmin_val),
            }
        }
    }
}

#[cfg(test)]
mod test_range_chmax_range_sum {
    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::Rng;
    use rand::SeedableRng;

    use super::abstract_segtree_beats::*;
    use super::range_chmax_range_sum::*;

    #[test]
    fn test_range_chmax_range_sum() {
        let xs = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let f = ChmaxFunc::new(6);

        let mut seg = SegtreeBeats::<RangeChmaxRangeSum>::from(
            xs.iter().copied().map(RangeSum::unit).collect_vec(),
        );
        seg.apply_range(3..8, f); // [0,1,2,6,6,6,6,7,8,9]
        assert_eq!(seg.prod(2..5).unwrap().sum, 14); // [2,6,6]
        assert_eq!(
            (0..xs.len()).map(|i| seg.get(i).unwrap().sum).collect_vec(),
            vec![0, 1, 2, 6, 6, 6, 6, 7, 8, 9]
        )
    }

    #[test]
    fn test_range_chmax_range_sum_random() {
        let mut rng = SmallRng::seed_from_u64(42);

        // テスト回数を大きくしすぎると実行時間がかかるため控えめに
        let n_tests = 20;
        let n_changes = 3;

        for _ in 0..n_tests {
            let n = rng.gen_range(0..15);
            let mut xs = (0..n).map(|_| rng.gen_range(0..10)).collect_vec();

            for _ in 0..n_changes {
                let begin = rng.gen_range(0..=n);
                let end = rng.gen_range(begin..=n);

                let chmax_val = rng.gen_range(0..10);
                let chmax_func = ChmaxFunc::new(chmax_val);

                let mut seg = SegtreeBeats::<RangeChmaxRangeSum>::from(
                    xs.iter().copied().map(RangeSum::unit).collect_vec(),
                );

                seg.apply_range(begin..end, chmax_func);

                for i in begin..end {
                    xs[i] = i64::max(xs[i], chmax_val);
                }

                assert_eq!(
                    seg.prod(begin..end).unwrap().sum,
                    xs[begin..end].iter().sum::<i64>()
                );
                assert_eq!((0..n).map(|i| seg.get(i).unwrap().sum).collect_vec(), xs)
            }
        }
    }
}

#[cfg(test)]
mod test_range_chmin_range_sum {
    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::Rng;
    use rand::SeedableRng;

    use super::abstract_segtree_beats::*;
    use super::range_chmin_range_sum::*;

    #[test]
    fn test_range_chmax_range_sum() {
        let xs = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let f = ChminFunc::new(4);

        let mut seg = SegtreeBeats::<RangeChminRangeSum>::from(
            xs.iter().copied().map(RangeSum::unit).collect_vec(),
        );
        seg.apply_range(3..8, f); // [0,1,2,3,4,4,4,4,8,9]
        assert_eq!(seg.prod(3..6).unwrap().sum, 11); // [3,4,4]
        assert_eq!(
            (0..xs.len()).map(|i| seg.get(i).unwrap().sum).collect_vec(),
            vec![0, 1, 2, 3, 4, 4, 4, 4, 8, 9]
        )
    }

    #[test]
    fn test_range_chmax_range_sum_random() {
        let mut rng = SmallRng::seed_from_u64(42);

        // テスト回数を大きくしすぎると実行時間がかかるため控えめに
        let n_tests = 20;
        let n_changes = 3;

        for _ in 0..n_tests {
            let n = rng.gen_range(0..15);
            let mut xs = (0..n).map(|_| rng.gen_range(0..10)).collect_vec();

            for _ in 0..n_changes {
                let begin = rng.gen_range(0..=n);
                let end = rng.gen_range(begin..=n);

                let chmax_val = rng.gen_range(0..10);
                let chmax_func = ChminFunc::new(chmax_val);

                let mut seg = SegtreeBeats::<RangeChminRangeSum>::from(
                    xs.iter().copied().map(RangeSum::unit).collect_vec(),
                );

                seg.apply_range(begin..end, chmax_func);

                for i in begin..end {
                    xs[i] = i64::min(xs[i], chmax_val);
                }

                assert_eq!(
                    seg.prod(begin..end).unwrap().sum,
                    xs[begin..end].iter().sum::<i64>()
                );
                assert_eq!((0..n).map(|i| seg.get(i).unwrap().sum).collect_vec(), xs)
            }
        }
    }
}
