// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xs: [i64; n],
    }

    let mut s = RangeAddRangeMinSegtree::from_slice(&xs);

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                l: Usize1,
                r: Usize1,
                x: i64
            }
            s.range_add(l..=r, x);
        }
        if t == 2 {
            input! {
                l: Usize1,
                r: Usize1
            }
            let ans = s.range_min(l..=r);
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
use range_add_range_min::*;
#[allow(clippy::module_inception)]
pub mod range_add_range_min {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::Sum;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};
    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMin<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RangeMin<T>
    where
        T: Copy + Ord + Bounded,
    {
        type S = T;
        fn identity() -> T {
            T::max_value()
        }
        fn binary_operation(a: &T, b: &T) -> T {
            *a.min(b)
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    pub struct AddAction<T> {
        add_val: T,
    }
    impl<T> AddAction<T>
    where
        T: Copy,
    {
        pub fn new(val: T) -> Self {
            Self { add_val: val }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeAddRangeMin<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAddRangeMin<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Sum,
    {
        type M = RangeMin<T>;
        type F = AddAction<T>;
        fn identity_map() -> Self::F {
            AddAction { add_val: zero() }
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            AddAction {
                add_val: g.add_val + f.add_val,
            }
        }
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            if *x == T::max_value() {
                T::max_value()
            } else {
                *x + f.add_val
            }
        }
    }
    pub trait Bounded {
        fn min_value() -> Self;
        fn max_value() -> Self;
    }
    impl Bounded for i64 {
        fn min_value() -> Self {
            i64::MIN
        }
        fn max_value() -> Self {
            i64::MAX
        }
    }
    #[derive(Clone)]
    pub struct RangeAddRangeMinSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Sum,
    {
        segtree: LazySegtree<RangeAddRangeMin<T>>,
        len: usize,
    }
    impl<T> RangeAddRangeMinSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Sum,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![zero(); n];
            Self::from_slice(&xs)
        }
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: LazySegtree::from(xs.to_vec()),
                len,
            }
        }
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }
        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p)
        }
        pub fn range_min<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }
        pub fn all_min(&self) -> T {
            self.segtree.all_prod()
        }
        pub fn range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, AddAction::new(x))
        }
        /// 左端 `l` を固定し、区間 `[l, r)` での最小値が述語 `g` を満たすような最大の `r` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.max_right(l, g)
        }
        /// 右端 `r` を固定し、区間 `[l, r)` での最小値が述語 `g` を満たすような最小の `l` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.min_left(r, g)
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
