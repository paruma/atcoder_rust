#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut cnts = RangeUpdateRangeSumSegtree::new(&vec![1_i64; n]);

    for _ in 0..q {
        input! {
            x: Usize1,
            y: Usize1,
        }

        let sum = cnts.range_sum(0..=x);
        cnts.apply_range_update(0..=x, 0);
        let prev_y = cnts.get(y);
        cnts.set(y, prev_y + sum);
        println!("{}", sum);
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

use ac_library::FenwickTree;
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
        collections::{BinaryHeap, HashMap, HashSet},
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
use range_update_range_sum::*;
#[allow(clippy::module_inception)]
pub mod range_update_range_sum {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSum<T> {
        pub sum: T,
        pub len: i64,
    }
    impl<T> RangeSum<T> {
        pub fn unit(x: T) -> RangeSum<T> {
            RangeSum { sum: x, len: 1 }
        }
    }
    pub struct ValueLenSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenSum<T>
    where
        T: Copy + Add<Output = T> + From<i64>,
    {
        type S = RangeSum<T>;
        fn identity() -> RangeSum<T> {
            RangeSum {
                sum: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: a.sum + b.sum,
                len: a.len + b.len,
            }
        }
    }
    pub struct RangeUpdateRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeUpdateRangeSum<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = Option<T>;
        fn identity_map() -> Option<T> {
            None
        }
        fn composition(a: &Option<T>, b: &Option<T>) -> Option<T> {
            if a.is_some() { *a } else { *b }
        }
        fn mapping(f: &Option<T>, x: &RangeSum<T>) -> RangeSum<T> {
            match f {
                Some(val) => RangeSum {
                    sum: *val * x.len.into(),
                    len: x.len,
                },
                None => *x,
            }
        }
    }
    pub struct RangeUpdateRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeUpdateRangeSum<T>>,
        len: usize,
    }
    impl<T> RangeUpdateRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        pub fn new(xs: &[T]) -> RangeUpdateRangeSumSegtree<T> {
            let xs = xs.iter().copied().map(RangeSum::unit).collect_vec();
            let len = xs.len();
            RangeUpdateRangeSumSegtree {
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
        pub fn range_sum<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).sum
        }
        pub fn all_sum(&self) -> T {
            self.segtree.all_prod().sum
        }
        pub fn apply_update(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Some(x))
        }
        pub fn apply_range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Some(x))
        }
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
