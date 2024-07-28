//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    t: usize,
    ts: Vec<SubProblem>,
}

pub fn segtree_to_vec<M: ac_library::Monoid>(
    seg: &ac_library::Segtree<M>,
    len: usize,
) -> Vec<M::S> {
    (0..len).map(|i| seg.get(i)).collect()
}

#[derive(Debug, Clone)]
struct SubProblem {
    n: usize,
    xs: Vec<i64>,
}

impl SubProblem {
    fn read() -> SubProblem {
        input! {
            n: usize,
            xs: [i64; n]
        }

        SubProblem { n, xs }
    }
    fn solve(&self) -> SubAnswer {
        // LIS の dp の復元を頑張る
        let n = self.n;
        let xs = &self.xs;
        let cc = CoordinateCompression::new(xs);
        let xs_cc = cc.compress_vec(xs);
        let lis_len = {
            // lis_len[i] = xs[0..=i] での LIS の長さ
            let mut dp = vec![0; n];

            let mut seg = Segtree::<Max<usize>>::from(vec![0; cc.space_size() + 1]);

            for (i, x) in xs_cc.iter().copied().enumerate() {
                dp[i] = seg.prod(0..x) + 1;
                seg.set(x, usize::max(dp[i], seg.get(x)));
            }
            dp
        };

        let lis_len_max = lis_len.iter().copied().max().unwrap();
        // xs_cc[i] < map[lis_len[i]]  だったら i は LIS に入り得る。
        // もし xs_cc[i] >= map[lis_len[i]] だと、i は LIS に入れない。
        let mut map = vec![0; lis_len_max + 1];
        map[lis_len_max] = usize::MAX;
        let mut ans = vec![];
        for i in (0..n).rev() {
            if xs_cc[i] < map[lis_len[i]] {
                ans.push(i);
                map[lis_len[i] - 1] = map[lis_len[i] - 1].max(xs_cc[i]);
            }
        }

        ans.sort();

        SubAnswer {
            n: ans.len(),
            is: ans,
        }
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

    fn solve2(&self) -> SubAnswer {
        // i を含む LIS が存在する
        // ⟺ iを含むという制約化でのLIS長 が 普通のLIS長と一致している
        // で判定する
        // iを含むという制約化でのLIS長 = 「0..=i の LIS長」-「i..n の LIS長」 + 1
        let n = self.n;
        let xs = self.xs.clone();
        let xs_rev = xs.iter().copied().rev().map(|x| -x).collect_vec();

        let lis_normal = Self::calc_lis(&xs);
        let lis_rev = Self::calc_lis(&xs_rev);

        let max_lis = lis_normal.iter().copied().max().unwrap(); // 0..n での LIS

        let ans = (0..n)
            .filter(|&i| {
                // i が LIS に含まれるか考える。
                // 「0..=i の LIS長」-「i..n の LIS長」 + 1
                // が i を含むという制約化での LIS

                let lis_contains_i = {
                    let prefix_lis = lis_normal[i]; // 0..=i での LIS 長
                    let suffix_lis = lis_rev[n - i - 1]; // i..n での LIS長
                    prefix_lis + suffix_lis - 1
                };

                lis_contains_i == max_lis
            })
            .collect_vec();

        SubAnswer {
            n: ans.len(),
            is: ans,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct SubAnswer {
    n: usize,
    is: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            t: usize,
        }
        let ts = (0..t).map(|_| SubProblem::read()).collect_vec();

        Problem { t, ts }
    }
    fn solve(&self) -> Answer {
        let ans = self.ts.iter().map(|x| x.solve2()).collect_vec();
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
    ans: Vec<SubAnswer>,
}

impl Answer {
    fn print(&self) {
        for s in &self.ans {
            println!("{}", s.n);
            // 1オリジンにする
            print_vec_1line(&s.is.iter().copied().map(|x| x + 1).collect_vec());
        }
    }
}

fn main() {
    Problem::read().solve().print();
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

use ac_library::{FenwickTree, Max, Segtree};
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
