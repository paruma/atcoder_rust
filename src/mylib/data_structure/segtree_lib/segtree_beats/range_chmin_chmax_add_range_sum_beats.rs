use cargo_snippet::snippet;

use super::abstract_segtree_beats::abstract_segtree_beats::{
    MapMonoidBeats, MonoidBeats, SegtreeBeats,
};

#[snippet(
    prefix = "use range_chmin_chmax_add_range_sum_beats::*;",
    include = "abstract_segtree_beats"
)]
#[allow(clippy::module_inception)]
pub mod range_chmin_chmax_add_range_sum_beats {
    use super::{MapMonoidBeats, MonoidBeats, SegtreeBeats};
    use itertools::Itertools;
    use std::{
        cmp::{max, min},
        convert::Infallible,
        ops::RangeBounds,
    };

    const INF: i64 = i64::MAX;
    const NEG_INF: i64 = i64::MIN;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeSum {
        pub sum: i64,
        pub len: usize,
        pub max: i64,
        pub max_2nd: i64,
        pub max_cnt: usize,
        pub min: i64,
        pub min_2nd: i64,
        pub min_cnt: usize,
    }

    impl RangeSum {
        pub fn unit(x: i64) -> Option<RangeSum> {
            Some(RangeSum {
                sum: x,
                len: 1,
                max: x,
                max_2nd: NEG_INF,
                max_cnt: 1,
                min: x,
                min_2nd: INF,
                min_cnt: 1,
            })
        }
    }

    fn second_lowest(a: i64, a2: i64, b: i64, b2: i64) -> i64 {
        if a == b {
            min(a2, b2)
        } else if a2 <= b {
            a2
        } else if b2 <= a {
            b2
        } else {
            max(a, b)
        }
    }

    fn second_highest(a: i64, a2: i64, b: i64, b2: i64) -> i64 {
        if a == b {
            max(a2, b2)
        } else if a2 >= b {
            a2
        } else if b2 >= a {
            b2
        } else {
            min(a, b)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSumMonoid(Infallible);
    impl MonoidBeats for RangeSumMonoid {
        type S = Option<RangeSum>;

        fn identity() -> Self::S {
            Some(RangeSum {
                sum: 0,
                len: 0,
                max: NEG_INF,
                max_2nd: NEG_INF,
                max_cnt: 0,
                min: INF,
                min_2nd: INF,
                min_cnt: 0,
            })
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            match (a, b) {
                (None, None) => None,
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (Some(a), Some(b)) => {
                    if a.len == 0 {
                        return Some(*b);
                    }
                    if b.len == 0 {
                        return Some(*a);
                    }
                    Some(RangeSum {
                        sum: a.sum + b.sum,
                        len: a.len + b.len,
                        max: max(a.max, b.max),
                        max_2nd: second_highest(a.max, a.max_2nd, b.max, b.max_2nd),
                        max_cnt: if a.max > b.max {
                            a.max_cnt
                        } else if a.max < b.max {
                            b.max_cnt
                        } else {
                            a.max_cnt + b.max_cnt
                        },
                        min: min(a.min, b.min),
                        min_2nd: second_lowest(a.min, a.min_2nd, b.min, b.min_2nd),
                        min_cnt: if a.min < b.min {
                            a.min_cnt
                        } else if a.min > b.min {
                            b.min_cnt
                        } else {
                            a.min_cnt + b.min_cnt
                        },
                    })
                }
            }
        }

        fn fails(a: &Self::S) -> bool {
            a.is_none()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Func {
        pub add: i64,
        pub upper: i64,
        pub lower: i64,
    }

    impl Func {
        pub fn new_add(add: i64) -> Self {
            Func {
                add,
                upper: INF,
                lower: NEG_INF,
            }
        }
        pub fn new_chmin(x: i64) -> Self {
            Func {
                add: 0,
                upper: x,
                lower: NEG_INF,
            }
        }
        pub fn new_chmax(x: i64) -> Self {
            Func {
                add: 0,
                upper: INF,
                lower: x,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeChminChmaxAddRangeSum(Infallible);

    impl MapMonoidBeats for RangeChminChmaxAddRangeSum {
        type F = Func;
        type M = RangeSumMonoid;

        fn identity_map() -> Self::F {
            Func {
                add: 0,
                upper: INF,
                lower: NEG_INF,
            }
        }

        fn mapping(f: &Self::F, x: &<Self::M as MonoidBeats>::S) -> <Self::M as MonoidBeats>::S {
            match x {
                None => None,
                Some(x) => {
                    if x.len == 0 {
                        return Some(*x);
                    }

                    if x.len == 1 {
                        let mut next_x = *x;
                        if f.add != 0 {
                            next_x.sum += f.add;
                            next_x.max += f.add;
                            next_x.min += f.add;
                        }
                        if next_x.max > f.upper {
                            next_x.max = f.upper;
                            next_x.min = f.upper;
                            next_x.sum = f.upper;
                        }
                        if next_x.min < f.lower {
                            next_x.min = f.lower;
                            next_x.max = f.lower;
                            next_x.sum = f.lower;
                        }
                        return Some(next_x);
                    }

                    let mut next_x = *x;

                    // 1. Add
                    if f.add != 0 {
                        next_x.sum += f.add * (next_x.len as i64);
                        if next_x.max != NEG_INF {
                            next_x.max += f.add;
                        }
                        if next_x.max_2nd != NEG_INF {
                            next_x.max_2nd += f.add;
                        }
                        if next_x.min != INF {
                            next_x.min += f.add;
                        }
                        if next_x.min_2nd != INF {
                            next_x.min_2nd += f.add;
                        }
                    }

                    // 2. Chmin (upper)
                    if next_x.max > f.upper {
                        if next_x.max_2nd < f.upper {
                            next_x.sum -= (next_x.max - f.upper) * (next_x.max_cnt as i64);
                            next_x.max = f.upper;
                            if next_x.min > f.upper {
                                next_x.min = f.upper;
                            }
                            if next_x.min_2nd > f.upper {
                                next_x.min_2nd = f.upper;
                            }
                            if next_x.max == next_x.min {
                                next_x.min = f.upper;
                            }
                        } else {
                            return None;
                        }
                    }

                    // 3. Chmax (lower)
                    if next_x.min < f.lower {
                        if next_x.min_2nd > f.lower {
                            next_x.sum += (f.lower - next_x.min) * (next_x.min_cnt as i64);
                            next_x.min = f.lower;
                            if next_x.max < f.lower {
                                next_x.max = f.lower;
                            }
                            if next_x.max_2nd < f.lower {
                                next_x.max_2nd = f.lower;
                            }
                            if next_x.min == next_x.max {
                                next_x.max = f.lower;
                            }
                        } else {
                            return None;
                        }
                    }

                    Some(next_x)
                }
            }
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            let add_new = g.add + f.add;
            let upper_new = min(if g.upper == INF { INF } else { g.upper + f.add }, f.upper);
            let lower_new = max(
                min(
                    if g.lower == NEG_INF {
                        NEG_INF
                    } else {
                        g.lower + f.add
                    },
                    f.upper,
                ),
                f.lower,
            );

            Func {
                add: add_new,
                upper: upper_new,
                lower: lower_new,
            }
        }
    }

    #[derive(Clone)]
    pub struct RangeChminChmaxAddRangeSumSegtree {
        segtree: SegtreeBeats<RangeChminChmaxAddRangeSum>,
        len: usize,
    }

    impl RangeChminChmaxAddRangeSumSegtree {
        pub fn new(n: usize) -> Self {
            let segtree = SegtreeBeats::<RangeChminChmaxAddRangeSum>::new(n);
            Self { segtree, len: n }
        }

        pub fn from_slice(xs: &[i64]) -> Self {
            let len = xs.len();
            let segtree = SegtreeBeats::<RangeChminChmaxAddRangeSum>::from(
                xs.iter().copied().map(RangeSum::unit).collect_vec(),
            );
            Self { segtree, len }
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }

        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, RangeSum::unit(x));
        }

        pub fn get(&mut self, p: usize) -> i64 {
            self.segtree.get(p).unwrap().sum
        }

        pub fn sum<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
            self.segtree.prod(range).unwrap().sum
        }

        pub fn min<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
            self.segtree.prod(range).unwrap().min
        }

        pub fn max<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
            self.segtree.prod(range).unwrap().max
        }

        pub fn chmax<R: RangeBounds<usize>>(&mut self, range: R, x: i64) {
            self.segtree.apply_range(range, Func::new_chmax(x));
        }

        pub fn chmin<R: RangeBounds<usize>>(&mut self, range: R, x: i64) {
            self.segtree.apply_range(range, Func::new_chmin(x));
        }

        pub fn add<R: RangeBounds<usize>>(&mut self, range: R, x: i64) {
            self.segtree.apply_range(range, Func::new_add(x));
        }

        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod test_range_chmin_chmax_add_range_sum {
    use super::range_chmin_chmax_add_range_sum_beats::*;

    #[test]
    fn test_simple() {
        let xs = [0, 1, 2, 3, 4];
        let mut seg = RangeChminChmaxAddRangeSumSegtree::from_slice(&xs);
        // [0, 1, 2, 3, 4]
        seg.add(0..5, 1);
        // [1, 2, 3, 4, 5]
        assert_eq!(seg.to_vec(), vec![1, 2, 3, 4, 5]);
        seg.chmin(0..5, 3);
        // [1, 2, 3, 3, 3]
        assert_eq!(seg.to_vec(), vec![1, 2, 3, 3, 3]);
        seg.chmax(0..5, 2);
        // [2, 2, 3, 3, 3]
        assert_eq!(seg.to_vec(), vec![2, 2, 3, 3, 3]);
        assert_eq!(seg.sum(0..5), 13);
    }

    #[ignore]
    #[test]
    fn test_random() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};
        const INF: i64 = i64::MAX;
        const NEG_INF: i64 = i64::MIN;

        let mut rng = SmallRng::seed_from_u64(42);

        for t in 0..100 {
            let n = rng.random_range(1..=30);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeChminChmaxAddRangeSumSegtree::from_slice(&naive_vec);
            let mut ops = Vec::new();

            for _ in 0..100 {
                let op_type = rng.random_range(0..8);
                let p = rng.random_range(0..n);
                let l = rng.random_range(0..=n);
                let r = rng.random_range(l..=n);
                let x = rng.random_range(-100..=100);

                match op_type {
                    0 => {
                        // set(p, x)
                        ops.push(format!("set({}, {})", p, x));
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // chmax(range, x)
                        ops.push(format!("chmax({}..{}, {})", l, r, x));
                        for i in l..r {
                            naive_vec[i] = naive_vec[i].max(x);
                        }
                        segtree.chmax(l..r, x);
                    }
                    2 => {
                        // chmin(range, x)
                        ops.push(format!("chmin({}..{}, {})", l, r, x));
                        for i in l..r {
                            naive_vec[i] = naive_vec[i].min(x);
                        }
                        segtree.chmin(l..r, x);
                    }
                    3 => {
                        // add(range, x)
                        ops.push(format!("add({}..{}, {})", l, r, x));
                        for i in l..r {
                            naive_vec[i] += x;
                        }
                        segtree.add(l..r, x);
                    }
                    4 => {
                        // get(p)
                        ops.push(format!("get({})", p));
                        let expected = naive_vec[p];
                        let actual = segtree.get(p);
                        if actual != expected {
                            eprintln!("Test case #{}:", t);
                            eprintln!("Ops:");
                            for op in &ops {
                                eprintln!("{}", op);
                            }
                            assert_eq!(actual, expected, "get({}) failed", p);
                        }
                    }
                    5 => {
                        // sum(range)
                        ops.push(format!("sum({}..{})", l, r));
                        let expected_sum: i64 = naive_vec[l..r].iter().sum();
                        let actual_sum = segtree.sum(l..r);
                        if actual_sum != expected_sum {
                            eprintln!("Test case #{}:", t);
                            eprintln!("Ops:");
                            for op in &ops {
                                eprintln!("{}", op);
                            }
                            assert_eq!(actual_sum, expected_sum, "sum({}..{}) failed", l, r);
                        }
                    }
                    6 => {
                        // min(range)
                        ops.push(format!("min({}..{})", l, r));
                        let expected_min = if l == r {
                            INF
                        } else {
                            *naive_vec[l..r].iter().min().unwrap()
                        };
                        let actual_min = segtree.min(l..r);
                        if actual_min != expected_min {
                            eprintln!("Test case #{}:", t);
                            eprintln!("Ops:");
                            for op in &ops {
                                eprintln!("{}", op);
                            }
                            assert_eq!(actual_min, expected_min, "min({}..{}) failed", l, r);
                        }
                    }
                    7 => {
                        // max(range)
                        ops.push(format!("max({}..{})", l, r));
                        let expected_max = if l == r {
                            NEG_INF
                        } else {
                            *naive_vec[l..r].iter().max().unwrap()
                        };
                        let actual_max = segtree.max(l..r);
                        if actual_max != expected_max {
                            eprintln!("Test case #{}:", t);
                            eprintln!("Ops:");
                            for op in &ops {
                                eprintln!("{}", op);
                            }
                            assert_eq!(actual_max, expected_max, "max({}..{}) failed", l, r);
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if segtree.to_vec() != naive_vec {
                eprintln!("Test case #{}:", t);
                eprintln!("Ops:");
                for op in &ops {
                    eprintln!("{}", op);
                }
                assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
            }
        }
    }
}
