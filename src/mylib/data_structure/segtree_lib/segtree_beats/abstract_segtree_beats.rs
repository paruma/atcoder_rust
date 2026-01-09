use cargo_snippet::snippet;

#[snippet(prefix = "use abstract_segtree_beats::*;")]
#[allow(clippy::module_inception)]
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

        pub fn to_vec(&mut self) -> Vec<<F::M as MonoidBeats>::S> {
            (0..self.n).map(|i| self.get(i)).collect()
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

#[cfg(test)]
mod tests {
    use super::abstract_segtree_beats::*;
    use itertools::Itertools;
    use std::ops::Bound;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct S {
        sum: i64,
        size: i64,
    }

    struct AddSum;
    impl MonoidBeats for AddSum {
        type S = S;
        fn identity() -> Self::S {
            S { sum: 0, size: 0 }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            S {
                sum: a.sum + b.sum,
                size: a.size + b.size,
            }
        }
        fn fails(_: &Self::S) -> bool {
            false
        }
    }

    struct MapAddSum;
    impl MapMonoidBeats for MapAddSum {
        type M = AddSum;
        type F = i64;
        fn identity_map() -> Self::F {
            0
        }
        fn mapping(f: &Self::F, x: &S) -> S {
            S {
                sum: x.sum + f * x.size,
                size: x.size,
            }
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f + g
        }
    }

    #[test]
    fn test_default() {
        let seg = SegtreeBeats::<MapAddSum>::default();
        assert_eq!(seg.all_prod(), AddSum::identity());
    }

    #[test]
    fn test_new() {
        let mut seg = SegtreeBeats::<MapAddSum>::new(10);
        // identity_element() で初期化されるため、size は 0 になる
        assert_eq!(seg.all_prod().size, 0);
        assert_eq!(seg.get(0), S { sum: 0, size: 0 });
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn test_prod_range_patterns() {
        let v = (0..10).map(|i| S { sum: i, size: 1 }).collect_vec();
        let mut seg: SegtreeBeats<MapAddSum> = v.into();

        // 1. .. (Unbounded, Unbounded)
        assert_eq!(seg.prod(..).sum, 45);

        // 2. l..r (Included, Excluded)
        assert_eq!(seg.prod(2..5).sum, 2 + 3 + 4);

        // 3. l..=r (Included, Included)
        assert_eq!(seg.prod(2..=4).sum, 2 + 3 + 4);

        // 4. ..r (Unbounded, Excluded)
        assert_eq!(seg.prod(..3).sum, 0 + 1 + 2);

        // 5. ..=r (Unbounded, Included)
        assert_eq!(seg.prod(..=2).sum, 0 + 1 + 2);

        // 6. l.. (Included, Unbounded)
        assert_eq!(seg.prod(7..).sum, 7 + 8 + 9);

        // 7. Bound::Excluded for start
        assert_eq!(
            seg.prod((Bound::Excluded(1), Bound::Included(3))).sum,
            2 + 3
        );
    }

    #[test]
    fn test_apply_range_patterns() {
        let n = 10;
        let mut seg: SegtreeBeats<MapAddSum> = vec![S { sum: 0, size: 1 }; n].into();

        let sums =
            |seg: &mut SegtreeBeats<MapAddSum>| seg.to_vec().iter().map(|s| s.sum).collect_vec();

        // 1. ..
        seg.apply_range(.., 1);
        assert_eq!(sums(&mut seg), vec![1; 10]);

        // 2. l..r
        seg.apply_range(2..5, 10);
        assert_eq!(sums(&mut seg), vec![1, 1, 11, 11, 11, 1, 1, 1, 1, 1]);

        // 3. l..=r
        seg.apply_range(2..=3, 100);
        assert_eq!(sums(&mut seg), vec![1, 1, 111, 111, 11, 1, 1, 1, 1, 1]);

        // 4. ..r
        seg.apply_range(..2, 1000);
        assert_eq!(
            sums(&mut seg),
            vec![1001, 1001, 111, 111, 11, 1, 1, 1, 1, 1]
        );

        // 5. ..=r
        seg.apply_range(..=1, 10000);
        assert_eq!(
            sums(&mut seg),
            vec![11001, 11001, 111, 111, 11, 1, 1, 1, 1, 1]
        );

        // 6. l..
        seg.apply_range(8.., 50);
        assert_eq!(
            sums(&mut seg),
            vec![11001, 11001, 111, 111, 11, 1, 1, 1, 51, 51]
        );

        // 7. Bound::Excluded for start
        seg.apply_range((Bound::Excluded(8), Bound::Unbounded), 100000);
        assert_eq!(
            sums(&mut seg),
            vec![11001, 11001, 111, 111, 11, 1, 1, 1, 51, 100051]
        );
    }

    #[test]
    fn test_apply() {
        let v = vec![
            S { sum: 1, size: 1 },
            S { sum: 2, size: 1 },
            S { sum: 3, size: 1 },
        ];
        let mut seg: SegtreeBeats<MapAddSum> = v.clone().into();
        seg.apply(1, 10);
        assert_eq!(seg.get(1).sum, 12);
        assert_eq!(seg.all_prod().sum, 16);
        assert_eq!(
            seg.to_vec(),
            vec![
                S { sum: 1, size: 1 },
                S { sum: 12, size: 1 },
                S { sum: 3, size: 1 }
            ]
        );
    }

    #[test]
    fn test_max_right() {
        let v = vec![S { sum: 1, size: 1 }; 10];
        let mut seg: SegtreeBeats<MapAddSum> = v.into();
        assert_eq!(seg.max_right(0, |s| s.sum <= 5), 5);
        assert_eq!(seg.max_right(0, |s| s.sum <= 10), 10);
        assert_eq!(seg.max_right(0, |s| s.sum <= 0), 0);
    }

    #[test]
    fn test_min_left() {
        let v = vec![S { sum: 1, size: 1 }; 10];
        let mut seg: SegtreeBeats<MapAddSum> = v.into();
        assert_eq!(seg.min_left(10, |s| s.sum <= 5), 5);
        assert_eq!(seg.min_left(10, |s| s.sum <= 10), 0);
        assert_eq!(seg.min_left(10, |s| s.sum <= 0), 10);
    }
}
