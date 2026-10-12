// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xs: [i64; n],
    }

    let mut s = RangeSumSegtree::from_slice(&xs);

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: Usize1,
                x: i64
            }
            s.set(i, x);
        }
        if t == 2 {
            input! {
                l: Usize1,
                t: i64
            }
            let ans = s.max_right(l, |s| *s < t) + 1;
            println!("{}", ans);
        }
        if t == 3 {
            input! {
                r: Usize1,
                t: i64
            }
            let ans = s.min_left(r + 1, |s| *s < t);
            println!("{}", ans);
        }
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
use range_sum_segtree::*;
#[allow(clippy::module_inception)]
pub mod range_sum_segtree {
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::Sum;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};
    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }
    /// 汎用的な加算モノイド。
    ///
    /// `std::ops::Add` と `std::iter::Sum` を実装している型に対応。
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct GeneralAdditive<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for GeneralAdditive<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        type S = T;
        #[inline]
        fn identity() -> Self::S {
            zero()
        }
        #[inline]
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    /// ACL の Segtree を使用した区間和セグメント木。
    ///
    /// 数値型 T に対して点更新・区間和取得を行う。
    #[derive(Clone)]
    pub struct RangeSumSegtree<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        segtree: Segtree<GeneralAdditive<T>>,
        len: usize,
    }
    impl<T> RangeSumSegtree<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        /// 単位元で初期化されたセグメント木を構築する
        pub fn new(n: usize) -> Self {
            Self {
                segtree: Segtree::<GeneralAdditive<T>>::new(n),
                len: n,
            }
        }
        /// 配列からセグメント木を構築する
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: Segtree::<GeneralAdditive<T>>::from(xs.to_vec()),
                len,
            }
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        /// p 番目の要素を x に更新する
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }
        /// p 番目の要素を取得する
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p)
        }
        /// 指定した範囲の和を取得する
        pub fn range_sum<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }
        /// 全要素の和を取得する
        pub fn all_sum(&self) -> T {
            self.segtree.all_prod()
        }
        /// p 番目の要素に x を加算する
        pub fn add(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, current + x);
        }
        /// セグメント木上の二分探索。
        ///
        /// [l, r) の和 s について f(&s) が true となる最大の r を返す。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * 加法の単位元に対して `f` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.max_right(l, f)
        }
        /// セグメント木上の二分探索。
        ///
        /// [l, r) の和 s について f(&s) が true となる最小の l を返す。
        ///
        /// # 前提条件
        /// * `r <= n`
        /// * 加法の単位元に対して `f` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.min_left(r, f)
        }
        /// 現在の状態を Vec として返す
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
