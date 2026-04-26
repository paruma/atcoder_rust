// 問題文と制約は読みましたか？
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        lras: [(Usize1, Usize1, i64); q],
    }
    use ac_library::ModInt998244353 as Mint;

    let mut seg = RangeAffineRangeSumOfSquareSegtree::from_slice(&vec![Mint::new(0); n]);
    for (l, r, a) in lras {
        seg.range_add(l..=r, Mint::new(a));
        let sum = seg.range_sum(l..=r);
        let sum_sq = seg.range_sum_of_square(l..=r);
        let ans: Mint = (sum * sum - sum_sq) / 2;
        println!("{}", ans);
    }
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

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    },
};

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
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
use range_affine_range_sum_of_square::*;
#[allow(clippy::module_inception)]
pub mod range_affine_range_sum_of_square {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
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
    pub struct RangeSum<T> {
        pub sum_sq: T,
        pub sum: T,
        pub len: i64,
    }
    impl<T> RangeSum<T>
    where
        T: Copy + Mul<Output = T>,
    {
        pub fn unit(x: T) -> RangeSum<T> {
            RangeSum {
                sum_sq: x * x,
                sum: x,
                len: 1,
            }
        }
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
    pub struct ValueLenSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenSum<T>
    where
        T: Copy + Add<Output = T> + Sum,
    {
        type S = RangeSum<T>;
        fn identity() -> RangeSum<T> {
            RangeSum {
                sum_sq: zero(),
                sum: zero(),
                len: 0,
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum_sq: a.sum_sq + b.sum_sq,
                sum: a.sum + b.sum,
                len: a.len + b.len,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeAffineRangeSumOfSquare<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAffineRangeSumOfSquare<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + Mul<i64, Output = T> + Sum + Product,
    {
        type M = ValueLenSum<T>;
        type F = Affine<T>;
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
        fn mapping(f: &Affine<T>, x: &RangeSum<T>) -> RangeSum<T> {
            let a = f.slope;
            let b = f.intercept;
            let new_sum_sq = a * a * x.sum_sq + (a + a) * b * x.sum + b * b * x.len;
            let new_sum = a * x.sum + b * x.len;
            RangeSum {
                sum_sq: new_sum_sq,
                sum: new_sum,
                len: x.len,
            }
        }
    }
    #[derive(Clone)]
    pub struct RangeAffineRangeSumOfSquareSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + Mul<i64, Output = T> + Sum + Product,
    {
        segtree: LazySegtree<RangeAffineRangeSumOfSquare<T>>,
        len: usize,
    }
    impl<T> RangeAffineRangeSumOfSquareSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + Mul<i64, Output = T> + Sum + Product,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![zero(); n];
            Self::from_slice(&xs)
        }
        pub fn from_slice(xs: &[T]) -> RangeAffineRangeSumOfSquareSegtree<T> {
            let xs = xs.iter().copied().map(RangeSum::unit).collect_vec();
            let len = xs.len();
            RangeAffineRangeSumOfSquareSegtree {
                segtree: LazySegtree::from(xs),
                len,
            }
        }
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeSum::unit(x));
        }
        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).sum
        }
        pub fn range_sum_of_square<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).sum_sq
        }
        pub fn all_sum_of_square(&self) -> T {
            self.segtree.all_prod().sum_sq
        }
        pub fn range_sum<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).sum
        }
        pub fn all_sum(&self) -> T {
            self.segtree.all_prod().sum
        }
        pub fn affine(&mut self, p: usize, slope: T, intercept: T) {
            self.segtree.apply(p, Affine { slope, intercept })
        }
        pub fn update(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::constant_func(x))
        }
        pub fn add(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::addition_func(x))
        }
        pub fn range_affine<R>(&mut self, range: R, slope: T, intercept: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine { slope, intercept })
        }
        pub fn range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::constant_func(x))
        }
        pub fn range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::addition_func(x))
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
