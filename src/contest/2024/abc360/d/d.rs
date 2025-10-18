//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    t: i64,
    s: Vec<i64>,
    xs: Vec<i64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClosedRange {
    l: i64,
    r: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            t: i64,
            s: Bytes,
            xs: [i64; n],
        }
        let s = s
            .iter()
            .copied()
            .map(|ch| if ch == b'0' { -1 } else { 1 })
            .collect_vec();
        Problem { n, t, s, xs }
    }

    fn solve(&self) -> Answer {
        // 解法1: 二分探索
        let n = self.n;
        let t = self.t;
        let dirs = &self.s;
        let xs = &self.xs;

        let xs_left = (0..n)
            .filter(|i| dirs[*i] == -1)
            .map(|i| xs[i])
            .sorted()
            .collect_vec();

        let xs_right = (0..n)
            .filter(|i| dirs[*i] == 1)
            .map(|i| xs[i])
            .sorted()
            .collect_vec();

        let ans = xs_left
            .iter()
            .copied()
            .map(|x1| {
                // x1 は左に進む
                // x1 - 2 * t <= x2 < x1に入っている x2 in  xs_right の数。

                // xs_right
                //     .iter()
                //     .copied()
                //     .filter(|x2| x1 - 2 * t <= *x2 && *x2 < x1)
                //     .count()

                // [j_star1, j_star2] が条件を満たす範囲 (長さは j_star2 - j_star1 + 1)
                let j_star1 = bin_search(xs_right.len() as i64, -1, |j| {
                    //
                    x1 - 2 * t <= xs_right[j as usize]
                });

                let j_star2 = bin_search(-1, xs_right.len() as i64, |j| {
                    //
                    xs_right[j as usize] < x1
                });
                j_star2 - j_star1 + 1
            })
            .sum::<i64>();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解法2: 二分探索 (lower_bound)
        let n = self.n;
        let t = self.t;
        let dirs = &self.s;
        let xs = &self.xs;

        let xs_left = (0..n)
            .filter(|i| dirs[*i] == -1)
            .map(|i| xs[i])
            .sorted()
            .collect_vec();

        let xs_right = (0..n)
            .filter(|i| dirs[*i] == 1)
            .map(|i| xs[i])
            .sorted()
            .collect_vec();

        let ans = xs_left
            .iter()
            .copied()
            .map(|x1| {
                // x1 は左に進む
                // x1 - 2 * t <= x2 < x1に入っているxs_right の数。

                // xs_right
                //     .iter()
                //     .copied()
                //     .filter(|x2| x1 - 2 * t <= *x2 && *x2 < x1)
                //     .count()

                // [begin, end) が条件を満たす範囲 (長さは end - begin)
                let begin = lower_bound(&xs_right, x1 - 2 * t);
                let end = lower_bound(&xs_right, x1);
                (end - begin) as i64
            })
            .sum::<i64>();

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // 解法3: 転倒数を使う
        let t = self.t;
        let dirs = &self.s;
        let xs = &self.xs;

        let xs_after = izip!(xs, dirs)
            .sorted_by_key(|(x, _)| **x)
            .map(|(x, dir)| x + t * dir)
            .collect_vec();

        fn inversion_number(xs: &[i64]) -> usize {
            // i < j で xs[i] >= xs[j] となる (i, j) の組 (通常の転倒数と少し違う)
            let cc = CoordinateCompression::new(xs);
            let xs_cc = cc.compress_vec(xs);
            let mut bit = FenwickTree::new(cc.space_size(), 0);
            let mut ret = 0;
            for x in xs_cc {
                ret += bit.sum(x..); // xも含めた和
                bit.add(x, 1);
            }
            ret
        }
        let ans = inversion_number(&xs_after) as i64;

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
    use superslice::Ext;
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
        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        /// 計算量: O(log(|space|))
        pub fn compress_floor(&self, x: i64) -> usize {
            self.space.upper_bound(&x) - 1
        }
        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
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
