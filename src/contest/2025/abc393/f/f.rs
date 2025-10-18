#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    right: Usize1,
    x: i64,
}
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<i64>,
    qs: Vec<Query>,
}

fn calc_lis(xs: &[i64]) -> Vec<usize> {
    let n = xs.len();
    let cc = CoordinateCompression::new(xs);
    let xs_cc = cc.compress_vec(xs);

    // lis_len[i] = xs[0..=i] での LIS の長さ
    let mut dp = vec![0; n];

    let mut seg = Segtree::<Max<usize>>::from(vec![0; cc.space_size() + 1]);

    for (i, x) in xs_cc.iter().copied().enumerate() {
        dp[i] = seg.prod(0..x) + 1;
        seg.set(x, usize::max(dp[i], seg.get(x)));
    }
    dp
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: [i64; n],
            qs: [Query; nq]
        }
        Problem { n, nq, xs, qs }
    }

    fn solve(&self) -> Answer {
        // セグ木ベースの LIS (末尾がxのときのLIS長を求める)
        let n = self.n;
        let xs = &self.xs;
        let qs = &self.qs;
        let coards = chain!(xs.iter().copied(), qs.iter().copied().map(|q| q.x)).collect_vec();
        let cc = CoordinateCompression::new(&coards);

        let iqs = qs
            .iter()
            .copied()
            .enumerate()
            .sorted_by_key(|(_i, q)| q.right)
            .collect_vec();

        let mut ans = vec![i64::MAX; self.nq];

        let mut dp = vec![0; n];

        let mut seg = Segtree::<Max<usize>>::from(vec![0; cc.space_size() + 1]);

        let xs_cc = cc.compress_vec(xs);

        let mut iqs_iter = iqs.iter().copied().peekable();

        for (i, x) in xs_cc.iter().copied().enumerate() {
            dp[i] = seg.prod(0..x) + 1;
            seg.set(x, usize::max(dp[i], seg.get(x)));

            while let Some(&(qi, q)) = iqs_iter.peek() {
                if q.right <= i {
                    ans[qi] = seg.prod(0..=cc.compress(q.x)) as i64;
                    iqs_iter.next();
                } else {
                    break;
                }
            }
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // solve のリファクタリング
        let n = self.n;
        let xs = &self.xs;
        let qs = &self.qs;
        let coards = chain!(xs.iter().copied(), qs.iter().copied().map(|q| q.x)).collect_vec();
        let cc = CoordinateCompression::new(&coards);

        // r ごとにまとめる
        let iqs_by_r = qs
            .iter()
            .copied()
            .enumerate()
            .fold(vec![vec![]; n], |mut acc, (i, q)| {
                acc[q.right].push((i, q));
                acc
            });

        let mut ans = vec![i64::MAX; self.nq];

        let mut dp = vec![0; n];

        let mut seg = Segtree::<Max<usize>>::from(vec![0; cc.space_size() + 1]);

        let xs_cc = cc.compress_vec(xs);

        for (i, x) in xs_cc.iter().copied().enumerate() {
            dp[i] = seg.prod(0..x) + 1;
            seg.set(x, usize::max(dp[i], seg.get(x)));

            for &(qid, q) in &iqs_by_r[i] {
                ans[qid] = seg.prod(0..=cc.compress(q.x)) as i64;
            }
        }
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
        print_vec(&self.ans);
    }
}

fn main() {
    Problem::read().solve2().print();
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

use ac_library::{Max, Segtree};
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
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
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

pub fn segtree_to_vec<M: ac_library::Monoid>(
    seg: &ac_library::Segtree<M>,
    len: usize,
) -> Vec<M::S> {
    (0..len).map(|i| seg.get(i)).collect()
}
