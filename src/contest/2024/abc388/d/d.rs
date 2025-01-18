//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }

    fn solve(&self) -> Answer {
        // 遅延セグ木
        let n = self.n;
        let xs = &self.xs;
        let mut seg = RangeAffineRangeSumSegtree::new(&vec![0_i64; n + 2]);

        let mut ans = vec![0; n];

        for i in 0..n {
            let all = seg.get(i + 1) + xs[i];
            seg.apply_range_add(i + 2..(i + 2 + all as usize).min(n + 2), 1);

            ans[i] = i64::max(all - (n as i64 - i as i64 - 1), 0);
        }
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // いもす法
        let n = self.n;
        let xs = &self.xs;
        let mut ans = vec![0; n];
        let mut diff = vec![0; n + 2]; // (年度→石を持っている人数) の差分テーブル
        let mut cnt_non_zero = 0; // 石を持っている人の人数

        for i in 0..n {
            // i + 1年目に成人する
            cnt_non_zero += diff[i + 1];
            let all = cnt_non_zero + xs[i]; // i番目の人が成人して石をもらったあとの石の数

            // [i + 2, i + 2 + all) の区間で石を配る
            diff[i + 2] += 1;
            if (i + 2 + all as usize) < diff.len() {
                diff[i + 2 + all as usize] -= 1;
            }
            ans[i] = i64::max(all - (n - i - 1) as i64, 0);
        }

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // PriorityQueue で石を持っている人の人数を管理する
        // 石が0になる年度を持たせて適宜消す
        // 期限切れになったら消すとき、有効な対象の数を Priority Queue で管理するイメージ
        let n = self.n;
        let xs = &self.xs;
        let mut ans = vec![0; n];
        let mut pq = BinaryHeap::<Reverse<i64>>::new();

        for i in 0..n {
            while pq
                .peek()
                .map(|&Reverse(expired)| expired <= i as i64 + 1)
                .unwrap_or(false)
            {
                debug_assert!(pq.pop().is_some())
            }
            // i + 1年目に成人する
            let all = pq.len() as i64 + xs[i]; // i番目の人が成人して石をもらったあとの石の数

            // i + 2 + all 年目になると石の数が 0個になる
            pq.push(Reverse(i as i64 + 2 + all));

            ans[i] = i64::max(all - (n - i - 1) as i64, 0);
        }

        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // 全部 decrement する部分を offset を使って管理
        struct AllDecrementVec {
            raw: Vec<i64>,
            offset: i64,
        }
        impl AllDecrementVec {
            fn new() -> AllDecrementVec {
                AllDecrementVec {
                    raw: Vec::new(),
                    offset: 0,
                }
            }

            fn push(&mut self, x: i64) {
                self.raw.push(x - self.offset)
            }

            fn dec_all(&mut self) {
                self.offset -= 1;
            }

            fn get(&self, idx: usize) -> i64 {
                i64::max(self.raw[idx] + self.offset, 0)
            }
        }

        struct AllDecrementAndCounts {
            raw_cnts: HashMap<i64, usize>,
            cnt_dec: i64,
            cnt_zeros: usize,
            cnt_all: usize,
        }

        impl AllDecrementAndCounts {
            fn new() -> AllDecrementAndCounts {
                AllDecrementAndCounts {
                    raw_cnts: HashMap::new(),
                    cnt_dec: 0,
                    cnt_zeros: 0,
                    cnt_all: 0,
                }
            }

            fn insert(&mut self, val: i64) {
                *self.raw_cnts.entry(val + self.cnt_dec).or_insert(0) += 1;
                self.cnt_all += 1;
                if val == 0 {
                    self.cnt_zeros += 1;
                }
            }

            fn dec_all(&mut self) {
                self.cnt_dec += 1;
                self.cnt_zeros += self.raw_cnts.get(&self.cnt_dec).unwrap_or(&0);
            }

            fn count_zeros(&self) -> usize {
                self.cnt_zeros
            }

            fn count_non_zeros(&self) -> usize {
                self.cnt_all - self.cnt_zeros
            }

            fn count(&self, val: i64) -> usize {
                if val == 0 {
                    self.count_zeros()
                } else if val < 0 {
                    0
                } else {
                    self.raw_cnts
                        .get(&(val + self.cnt_dec))
                        .copied()
                        .unwrap_or(0)
                }
            }
        }

        let n = self.n;
        let xs = &self.xs;

        let mut stones = AllDecrementVec::new(); // それぞれの人が持っている石の数を管理する
        let mut counter = AllDecrementAndCounts::new(); // 持っている石の数が0個の人の人数を管理する

        for i in 0..n {
            let all = xs[i] + counter.count_non_zeros() as i64;
            stones.dec_all();
            counter.dec_all();
            stones.push(all);
            counter.insert(all);
            // dbg!((0..=i).map(|i| stones.get(i)).collect_vec());
        }

        let ans = (0..n).map(|i| stones.get(i)).collect_vec();
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec_1line(&self.ans);
    }
}

fn main() {
    Problem::read().solve3().print();
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

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_entropy();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

use ac_library::FenwickTree;
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
use std::ops::{Index, RangeBounds};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {

    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
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
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
use range_affine_range_sum::*;
pub mod range_affine_range_sum {
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Affine<T> {
        pub slope: T,
        pub intercept: T,
    }
    impl<T> Affine<T>
    where
        T: From<i64>,
    {
        /// 区間変更用（定数関数）
        pub fn constant_func(x: T) -> Affine<T> {
            Affine {
                slope: 0.into(),
                intercept: x,
            }
        }
        /// 区間加算用
        pub fn addition_func(x: T) -> Affine<T> {
            Affine {
                slope: 1.into(),
                intercept: x,
            }
        }
    }
    pub struct ValueLenSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenSum<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
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
    pub struct RangeAffineRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAffineRangeSum<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = Affine<T>;
        fn identity_map() -> Affine<T> {
            Affine {
                slope: 1.into(),
                intercept: 0.into(),
            }
        }
        fn composition(a: &Affine<T>, b: &Affine<T>) -> Affine<T> {
            Affine {
                slope: a.slope * b.slope,
                intercept: a.slope * b.intercept + a.intercept,
            }
        }
        fn mapping(f: &Affine<T>, x: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: f.slope * x.sum + f.intercept * x.len.into(),
                len: x.len,
            }
        }
    }
    pub struct RangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeAffineRangeSum<T>>,
        len: usize,
    }
    impl<T> RangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        pub fn new(xs: &[T]) -> RangeAffineRangeSumSegtree<T> {
            let xs = xs.iter().copied().map(RangeSum::unit).collect_vec();
            let len = xs.len();
            RangeAffineRangeSumSegtree {
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
        pub fn apply_affine(&mut self, p: usize, slope: T, intercept: T) {
            self.segtree.apply(p, Affine { slope, intercept })
        }
        pub fn apply_update(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::constant_func(x))
        }
        pub fn apply_add(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::addition_func(x))
        }
        pub fn apply_range_affine<R>(&mut self, range: R, slope: T, intercept: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine { slope, intercept })
        }
        pub fn apply_range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::constant_func(x))
        }
        pub fn apply_range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::addition_func(x))
        }
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
