/*
ABC 386 E - Maximize XOR

- 再帰を使わず itertools の combinations を使い、K が大きい場合は N - K 個を選ぶ (solve2)
- 再帰を使う
    - 分岐1の squash をせずに、K が大きい場合は N - K 個を選ぶ
        - 0 <= seq[0] < seq[1] < ... < seq[k-1] < N を多重 for ループな再帰で全列挙 (solve)
        - 各 i = 0,...,N - 1 に対して選ぶ選ばないの再帰をする (solve4)
    - 分岐1の squash をする
        - 0 <= seq[0] < seq[1] < ... < seq[k-1] < N を多重 for ループな再帰で全列挙 (solve3)
        - 各 i = 0,...,N - 1 に対して選ぶ選ばないの再帰をする (solve5)
*/

#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: usize,
            xs: [i64; n],
        }
        Problem { n, k, xs }
    }

    fn solve(&self) -> Answer {
        // DFS を使用。分岐1あり。
        /// xs から異なる k 個選んだときの xor sum を列挙する
        fn enumerate_sub_sequence_xor_sum(n: usize, k: usize, xs: &[i64]) -> Vec<i64> {
            struct DfsCombinations<'a> {
                n: usize,
                k: usize,
                xs: &'a [i64],
            }

            impl<'a> DfsCombinations<'a> {
                fn new(n: usize, k: usize, xs: &'a [i64]) -> Self {
                    Self { n, k, xs }
                }

                fn exec(&self) -> Vec<i64> {
                    let mut xor_sum_list = vec![];
                    self.exec_rec(&mut vec![], 0, &mut xor_sum_list);
                    xor_sum_list
                }

                // seq が現在の状態
                fn exec_rec(
                    &self,
                    seq: &mut Vec<usize>,
                    xor_sum: i64,
                    xor_sum_list: &mut Vec<i64>,
                ) {
                    if seq.len() == self.k {
                        // ここがforループの中のようなもの
                        xor_sum_list.push(xor_sum);
                        return;
                    }

                    let begin = seq.last().copied().map(|x| x + 1).unwrap_or(0);

                    // ループ範囲は具体例 (k=2 くらい) を考えるとわかる
                    for i in begin..self.n - self.k + 1 + seq.len() {
                        seq.push(i);
                        self.exec_rec(seq, xor_sum ^ self.xs[i], xor_sum_list);
                        seq.pop();
                    }
                }
            }
            DfsCombinations::new(n, k, xs).exec()
        }
        let n = self.n;
        let k = self.k;
        let xs = &self.xs;
        let ans = if k * 2 <= n {
            enumerate_sub_sequence_xor_sum(n, k, xs)
                .iter()
                .copied()
                .max()
                .unwrap()
        } else {
            let all = xs.iter().copied().fold(0, |acc, x| acc ^ x);
            enumerate_sub_sequence_xor_sum(n, n - k, xs)
                .iter()
                .copied()
                .map(|x| all ^ x)
                .max()
                .unwrap()
        };
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // combinations で組合せ全列挙。xor は全て素朴に計算 (k または n-k が 11以下なので。)

        let n = self.n;
        let k = self.k;
        let xs = &self.xs;

        let ans = if k * 2 <= n {
            xs.iter()
                .copied()
                .combinations(k)
                .map(|sub| sub.iter().copied().fold(0, |acc, x| acc ^ x))
                .max()
                .unwrap()
        } else {
            let all = xs.iter().copied().fold(0, |acc, x| acc ^ x);
            xs.iter()
                .copied()
                .combinations(n - k)
                .map(|sub| all ^ sub.iter().copied().fold(0, |acc, x| acc ^ x))
                .max()
                .unwrap()
        };
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // DFS を使用。分岐1を squash
        /// xs から異なる k 個選んだときの xor sum を列挙する
        fn enumerate_sub_sequence_xor_sum(n: usize, k: usize, xs: &[i64]) -> Vec<i64> {
            struct DfsCombinations<'a> {
                n: usize,
                k: usize,
                xs: &'a [i64],
                cum_xor: CumMonoid<BitwiseXor<i64>>,
            }

            impl<'a> DfsCombinations<'a> {
                fn new(n: usize, k: usize, xs: &'a [i64]) -> Self {
                    let cum_xor = CumMonoid::new(xs);
                    Self { n, k, xs, cum_xor }
                }

                fn exec(&self) -> Vec<i64> {
                    let mut xor_sum_list = vec![];
                    self.exec_rec(&mut vec![], 0, &mut xor_sum_list);
                    xor_sum_list
                }

                // seq が現在の状態
                fn exec_rec(
                    &self,
                    seq: &mut Vec<usize>,
                    xor_sum: i64,
                    xor_sum_list: &mut Vec<i64>,
                ) {
                    if seq.len() == self.k {
                        // ここがforループの中のようなもの
                        xor_sum_list.push(xor_sum);
                        return;
                    }

                    let begin = seq.last().copied().map(|x| x + 1).unwrap_or(0);
                    // ループ範囲は具体例 (k=2 くらい) を考えるとわかる
                    let end = self.n - self.k + 1 + seq.len();

                    // 分岐の数が1の場合は squash する
                    if end - begin == 1 {
                        // xs[begin..] の xor を加える
                        let final_xor_sum = xor_sum ^ self.cum_xor.suffix_prod(begin);
                        xor_sum_list.push(final_xor_sum);
                        return;
                    }

                    for i in begin..self.n - self.k + 1 + seq.len() {
                        seq.push(i);
                        self.exec_rec(seq, xor_sum ^ self.xs[i], xor_sum_list);
                        seq.pop();
                    }
                }
            }
            DfsCombinations::new(n, k, xs).exec()
        }
        let n = self.n;
        let k = self.k;
        let xs = &self.xs;
        let ans = enumerate_sub_sequence_xor_sum(n, k, xs)
            .iter()
            .copied()
            .max()
            .unwrap();
        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // 選ぶ選ばないの DFS を使用。
        /// xs から異なる k 個選んだときの xor sum を列挙する
        fn enumerate_sub_sequence_xor_sum(n: usize, k: usize, xs: &[i64]) -> Vec<i64> {
            struct DfsCombinations<'a> {
                n: usize,
                k: usize,
                xs: &'a [i64],
            }

            impl<'a> DfsCombinations<'a> {
                fn new(n: usize, k: usize, xs: &'a [i64]) -> Self {
                    Self { n, k, xs }
                }

                fn exec(&self) -> Vec<i64> {
                    let mut xor_sum_list = vec![];
                    self.exec_rec(0, &mut vec![], 0, &mut xor_sum_list);
                    xor_sum_list
                }

                // seq が現在の状態
                fn exec_rec(
                    &self,
                    i: usize,
                    seq: &mut Vec<usize>,
                    xor_sum: i64,
                    xor_sum_list: &mut Vec<i64>,
                ) {
                    if seq.len() == self.k {
                        // ここがforループの中のようなもの
                        xor_sum_list.push(xor_sum);
                        return;
                    }

                    // self.k - seq.len(): 残りの選ぶ数
                    // self.n - i: 残りの選ぶ候補
                    if self.k - seq.len() < self.n - i {
                        // 選ばない
                        self.exec_rec(i + 1, seq, xor_sum, xor_sum_list);
                    }

                    // 選ぶ
                    seq.push(i);
                    self.exec_rec(i + 1, seq, xor_sum ^ self.xs[i], xor_sum_list);
                    seq.pop();
                }
            }
            DfsCombinations::new(n, k, xs).exec()
        }
        let n = self.n;
        let k = self.k;
        let xs = &self.xs;
        let ans = if k * 2 <= n {
            enumerate_sub_sequence_xor_sum(n, k, xs)
                .iter()
                .copied()
                .max()
                .unwrap()
        } else {
            let all = xs.iter().copied().fold(0, |acc, x| acc ^ x);
            enumerate_sub_sequence_xor_sum(n, n - k, xs)
                .iter()
                .copied()
                .map(|x| all ^ x)
                .max()
                .unwrap()
        };
        Answer { ans }
    }

    fn solve5(&self) -> Answer {
        // 選ぶ選ばないの DFS を使用。分岐1を squash
        /// xs から異なる k 個選んだときの xor sum を列挙する
        fn enumerate_sub_sequence_xor_sum(n: usize, k: usize, xs: &[i64]) -> Vec<i64> {
            struct DfsCombinations<'a> {
                n: usize,
                k: usize,
                xs: &'a [i64],
                cum_xor: CumMonoid<BitwiseXor<i64>>,
            }

            impl<'a> DfsCombinations<'a> {
                fn new(n: usize, k: usize, xs: &'a [i64]) -> Self {
                    let cum_xor = CumMonoid::new(xs);
                    Self { n, k, xs, cum_xor }
                }

                fn exec(&self) -> Vec<i64> {
                    let mut xor_sum_list = vec![];
                    self.exec_rec(0, &mut vec![], 0, &mut xor_sum_list);
                    xor_sum_list
                }

                // seq が現在の状態
                fn exec_rec(
                    &self,
                    i: usize,
                    seq: &mut Vec<usize>,
                    xor_sum: i64,
                    xor_sum_list: &mut Vec<i64>,
                ) {
                    if seq.len() == self.k {
                        // ここがforループの中のようなもの
                        xor_sum_list.push(xor_sum);
                        return;
                    }

                    // self.k - seq.len(): 残りの選ぶ数
                    // self.n - i: 残りの選ぶ候補
                    if self.k - seq.len() == self.n - i {
                        // 残りはすべて選ぶ (squash)
                        let final_xor_sum = xor_sum ^ self.cum_xor.suffix_prod(i);
                        xor_sum_list.push(final_xor_sum);
                        return;
                    }

                    // 選ばない
                    self.exec_rec(i + 1, seq, xor_sum, xor_sum_list);

                    // 選ぶ
                    seq.push(i);
                    self.exec_rec(i + 1, seq, xor_sum ^ self.xs[i], xor_sum_list);
                    seq.pop();
                }
            }
            DfsCombinations::new(n, k, xs).exec()
        }
        let n = self.n;
        let k = self.k;
        let xs = &self.xs;
        let ans = enumerate_sub_sequence_xor_sum(n, k, xs)
            .iter()
            .copied()
            .max()
            .unwrap();
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
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    // let n = 200_000;
    // println!("{n} {n}");
    // println!("{}", std::iter::repeat(2).take(n).join(" "));
    Problem::read().solve5().print();
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
        // let n = rng.random_range(1..=10);
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
        // let mut rng = SmallRng::from_os_rng();
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
use cum_monoid::*;
pub mod cum_monoid {
    use ac_library::{Max, Min, Monoid};
    pub struct CumMonoid<M>
    where
        M: Monoid,
    {
        prefix_prod: Vec<M::S>,
        suffix_prod: Vec<M::S>,
    }
    impl<M> CumMonoid<M>
    where
        M: Monoid,
    {
        pub fn new(xs: &[M::S]) -> CumMonoid<M> {
            let mut prefix_prod = vec![M::identity(); xs.len() + 1];
            let mut suffix_prod = vec![M::identity(); xs.len() + 1];
            for i in 0..xs.len() {
                prefix_prod[i + 1] = M::binary_operation(&prefix_prod[i], &xs[i]);
            }
            for i in (0..xs.len()).rev() {
                suffix_prod[i] = M::binary_operation(&xs[i], &suffix_prod[i + 1]);
            }
            CumMonoid {
                prefix_prod,
                suffix_prod,
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_prod(&self, i: usize) -> M::S {
            self.prefix_prod[i].clone()
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_prod(&self, i: usize) -> M::S {
            self.suffix_prod[i].clone()
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[i], &self.suffix_prod[i + 1])
        }
        pub fn prod_without_range(&self, l: usize, r: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[l], &self.suffix_prod[r])
        }
    }
    pub struct CumMin {
        cum: CumMonoid<Min<i64>>,
    }
    impl CumMin {
        pub fn new(xs: &[i64]) -> CumMin {
            CumMin {
                cum: CumMonoid::new(xs),
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_min(&self, i: usize) -> i64 {
            self.cum.prefix_prod(i)
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_min(&self, i: usize) -> i64 {
            self.cum.suffix_prod(i)
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn min_without1(&self, i: usize) -> i64 {
            self.cum.prod_without1(i)
        }
        pub fn min_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }
    pub struct CumMax {
        cum: CumMonoid<Max<i64>>,
    }
    impl CumMax {
        pub fn new(xs: &[i64]) -> CumMax {
            CumMax {
                cum: CumMonoid::new(xs),
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_max(&self, i: usize) -> i64 {
            self.cum.prefix_prod(i)
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_max(&self, i: usize) -> i64 {
            self.cum.suffix_prod(i)
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn max_without1(&self, i: usize) -> i64 {
            self.cum.prod_without1(i)
        }
        pub fn max_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }
}
use monoid_bitwise::*;
pub mod monoid_bitwise {
    use ac_library::Monoid;
    use num_traits::Zero;
    use std::{
        convert::Infallible,
        marker::PhantomData,
        ops::{BitAnd, BitOr, BitXor, Not},
    };
    pub struct BitwiseOr<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseOr<S>
    where
        S: Copy + BitOr<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a | *b
        }
    }
    pub struct BitwiseAnd<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseAnd<S>
    where
        S: Copy + BitAnd<Output = S> + Not<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            !S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a & *b
        }
    }
    pub struct BitwiseXor<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseXor<S>
    where
        S: Copy + BitXor<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
    }
}
