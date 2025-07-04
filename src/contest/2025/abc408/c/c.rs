fn main() {
    input! {
        n: usize,
        m: usize,
        ranges: [(Usize1, Usize1); m],
    }

    let mut seg = RangeAffineRangeMinMaxSegtree::new(&vec![0_i64; n]);

    for (l, r) in ranges {
        seg.apply_range_add(l..=r, 1);
    }
    let ans: i64 = seg.all_min();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
use range_affine_range_minmax::*;
pub mod range_affine_range_minmax {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::{cmp::Ordering, convert::Infallible, ops::RangeBounds};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMinMax {
        pub min: i64,
        pub max: i64,
        pub len: i64,
    }
    impl RangeMinMax {
        pub fn unit(x: i64) -> RangeMinMax {
            RangeMinMax {
                min: x,
                max: x,
                len: 1,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Affine {
        pub slope: i64,
        pub intercept: i64,
    }
    impl Affine {
        /// 区間変更用（定数関数）
        pub fn constant_func(x: i64) -> Affine {
            Affine {
                slope: 0,
                intercept: x,
            }
        }
        /// 区間加算用
        pub fn addition_func(x: i64) -> Affine {
            Affine {
                slope: 1,
                intercept: x,
            }
        }
    }
    pub struct RangeMinMaxMonoid(Infallible);
    impl Monoid for RangeMinMaxMonoid {
        type S = RangeMinMax;
        fn identity() -> RangeMinMax {
            RangeMinMax {
                min: INF,
                max: -INF,
                len: 0,
            }
        }
        fn binary_operation(a: &RangeMinMax, b: &RangeMinMax) -> RangeMinMax {
            RangeMinMax {
                min: Ord::min(a.min, b.min),
                max: Ord::max(a.max, b.max),
                len: a.len + b.len,
            }
        }
    }
    const INF: i64 = i64::MAX;
    pub struct RangeAffineRangeMinMax(Infallible);
    impl MapMonoid for RangeAffineRangeMinMax {
        type M = RangeMinMaxMonoid;
        type F = Affine;
        fn identity_map() -> Affine {
            Affine {
                slope: 1,
                intercept: 0,
            }
        }
        fn composition(a: &Affine, b: &Affine) -> Affine {
            Affine {
                slope: a.slope * b.slope,
                intercept: a.slope * b.intercept + a.intercept,
            }
        }
        fn mapping(f: &Affine, x: &RangeMinMax) -> RangeMinMax {
            if x.len == 0 {
                return RangeMinMaxMonoid::identity();
            }
            match f.slope.cmp(&0) {
                Ordering::Equal => RangeMinMax {
                    min: f.intercept,
                    max: f.intercept,
                    len: x.len,
                },
                Ordering::Greater => RangeMinMax {
                    min: f.intercept + f.slope * x.min,
                    max: f.intercept + f.slope * x.max,
                    len: x.len,
                },
                Ordering::Less => RangeMinMax {
                    min: f.intercept + f.slope * x.max,
                    max: f.intercept + f.slope * x.min,
                    len: x.len,
                },
            }
        }
    }
    pub struct RangeAffineRangeMinMaxSegtree {
        segtree: LazySegtree<RangeAffineRangeMinMax>,
        len: usize,
    }
    impl RangeAffineRangeMinMaxSegtree {
        pub fn new(xs: &[i64]) -> RangeAffineRangeMinMaxSegtree {
            let xs = xs.iter().copied().map(RangeMinMax::unit).collect_vec();
            let len = xs.len();
            RangeAffineRangeMinMaxSegtree {
                segtree: LazySegtree::from(xs),
                len,
            }
        }
        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, RangeMinMax::unit(x));
        }
        pub fn get(&mut self, p: usize) -> i64 {
            self.segtree.get(p).min
        }
        pub fn range_min<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).min
        }
        pub fn range_max<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).max
        }
        pub fn all_min(&self) -> i64 {
            self.segtree.all_prod().min
        }
        pub fn all_max(&self) -> i64 {
            self.segtree.all_prod().max
        }
        pub fn apply_affine(&mut self, p: usize, slope: i64, intercept: i64) {
            self.segtree.apply(p, Affine { slope, intercept })
        }
        pub fn apply_update(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, Affine::constant_func(x))
        }
        pub fn apply_add(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, Affine::addition_func(x))
        }
        pub fn apply_range_affine<R>(&mut self, range: R, slope: i64, intercept: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine { slope, intercept })
        }
        pub fn apply_range_update<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::constant_func(x))
        }
        pub fn apply_range_add<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::addition_func(x))
        }
        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
