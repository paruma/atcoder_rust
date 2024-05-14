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
            xs: [i64; n]
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        // ソートして二分探索
        let n = self.n;
        let xs = &self.xs;
        let xs = xs.iter().copied().sorted().collect_vec();

        let term1 = (n as i64 - 1) * xs.iter().sum::<i64>();

        let term2 = {
            //
            let m = 100_000_000; //10^8
            (0..n)
                .map(|i| {
                    //
                    let current = xs[i];
                    let xs_sub = &xs[i + 1..];
                    // 10^8 - A_i 以上のがいくつあるか
                    let cnt = lower_bound(xs_sub, m - current);
                    xs_sub.len() - cnt
                })
                .sum::<usize>() as i64
                * m
        };
        let ans = term1 - term2;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // BIT を使う
        let n = self.n;
        let xs = &self.xs;
        let xs = xs.iter().copied().sorted().collect_vec();

        let term1 = (n as i64 - 1) * xs.iter().sum::<i64>();

        let term2 = {
            let m = 100_000_000; //10^8

            // A_i + A_j >= 10^8, i<j となる (i,j)の数
            let cnt = {
                let cc = CoordinateCompression::new(
                    &xs.iter().copied().flat_map(|x| [x, m - x]).collect_vec(),
                );
                let mut bit = FenwickTree::new(cc.space_size(), 0);
                let xs_cc = cc.compress_vec(&xs);
                let mut buf = vec![];
                for (x, x_cc) in izip!(xs, xs_cc) {
                    buf.push(bit.sum(cc.compress(m - x)..));
                    bit.add(x_cc, 1);
                }

                buf.iter().sum::<i64>()
            };
            cnt * m
        };
        let ans = term1 - term2;
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // BIT を使う (座標圧縮改良)
        let n = self.n;
        let xs = &self.xs;
        let xs = xs.iter().copied().sorted().collect_vec();

        let term1 = (n as i64 - 1) * xs.iter().sum::<i64>();

        let term2 = {
            let m = 100_000_000; //10^8

            // A_i + A_j >= 10^8, i<j となる (i,j)の数
            let cnt = {
                let cc = CoordinateCompression::new(&xs);
                let mut bit = FenwickTree::new(cc.space_size(), 0);
                let xs_cc = cc.compress_vec(&xs);
                let mut buf = vec![];
                for (x, x_cc) in izip!(xs, xs_cc) {
                    buf.push(bit.sum(cc.compress_ceil(m - x)..));
                    bit.add(x_cc, 1);
                }

                buf.iter().sum::<i64>()
            };
            cnt * m
        };
        let ans = term1 - term2;
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
    marker::{Bytes, Usize1},
};
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
/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// 計算量: O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok);
        assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
/// 指定された要素以上の値が現れる最初の位置を返す。
/// 計算量: O(log(|xs|))
/// ## Arguments
/// * xs: 単調増加
///     * 単調増加でなくても、 `|i| xs[i] >= key` が単調ならOK
/// ## Return
/// `I = {i in 0..xs.len() | xs[i] >= key}` としたとき、`min I` を返す。
/// ただし、`I` が空の場合は `xs.len()` を返す
/// 戻り値は、区間 `0..=xs.len()` の間で返る。
pub fn lower_bound<T: PartialOrd>(xs: &[T], key: T) -> usize {
    let pred = |i: i64| xs[i as usize] >= key;
    bin_search(xs.len() as i64, -1_i64, pred) as usize
}

use coordinate_compression::*;
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::*;

    pub struct CoordinateCompression {
        space: Vec<i64>,
    }
    impl CoordinateCompression {
        /// 計算量: O(|space|log(|space|))
        pub fn new(space: &[i64]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }
        /// 計算量: O(log(|space|))
        pub fn compress(&self, x: i64) -> usize {
            self.space.binary_search(&x).unwrap()
        }
        /// 計算量: O(log(|space|))
        pub fn compress_floor(&self, x: i64) -> usize {
            self.space.upper_bound(&x) - 1
        }

        /// 計算量: O(log(|space|))
        pub fn compress_ceil(&self, x: i64) -> usize {
            self.space.lower_bound(&x)
        }

        /// 計算量: O(|xs|log(|space|))
        pub fn compress_vec(&self, xs: &[i64]) -> Vec<usize> {
            xs.iter().copied().map(|x| self.compress(x)).collect_vec()
        }
        /// 計算量: O(1)
        pub fn decompress(&self, i: usize) -> i64 {
            self.space[i]
        }
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
    }
}
