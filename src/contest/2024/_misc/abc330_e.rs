//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<i64>,
    qs: Vec<Query<i64>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query<T> {
    i: usize,
    x: T,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: [i64; n],
            qs: [(Usize1, i64); nq],
        }
        let qs = qs
            .iter()
            .copied()
            .map(|(i, x)| Query { i, x })
            .collect_vec();
        Problem { n, nq, xs, qs }
    }
    fn solve(&self) -> Answer {
        // セグ木を使った解法

        // 隣の値も空間に含めないと [0, 2] の mex が 3 になってしまう。
        // (mex では数値の間に値があるかどうかが大事)
        let space = chain!([0], self.xs.clone(), self.qs.iter().copied().map(|q| q.x))
            .flat_map(|x| [x, x + 1])
            .collect_vec();

        let cc = CoordinateCompression::new(&space);
        let mut xs = cc.compress_vec(&self.xs);
        let qs = self
            .qs
            .iter()
            .copied()
            .map(|q| Query {
                i: q.i,
                x: cc.compress(q.x),
            })
            .collect_vec();

        let cnts_vec = {
            let mut cnts_vec = vec![0; cc.space_size()];
            for &x in &xs {
                cnts_vec[x] += 1;
            }
            cnts_vec
        };

        let mut cnts = Segtree::<Min<usize>>::from(cnts_vec);

        let _cnts_to_vec =
            |cnts: &Segtree<Min<usize>>| (0..cc.space_size()).map(|i| cnts.get(i)).collect_vec();

        let mut ans = vec![];
        for &q in &qs {
            cnts.set(xs[q.i], cnts.get(xs[q.i]) - 1);
            xs[q.i] = q.x;
            cnts.set(xs[q.i], cnts.get(xs[q.i]) + 1);

            let ans_e_cc = cnts.max_right(0, |x| *x != 0); // min(cnts[0..r]) = 0となる最小のr
            let ans_e = if ans_e_cc == 0 {
                0
            } else {
                cc.decompress(ans_e_cc - 1) + 1
            };
            ans.push(ans_e);
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // BtreeSet を使う
        let space = chain!([0], self.xs.clone(), self.qs.iter().copied().map(|q| q.x))
            .flat_map(|x| [x, x + 1])
            .collect_vec();

        let cc = CoordinateCompression::new(&space);
        let mut xs_cc = cc.compress_vec(&self.xs);

        let qs_cc = self
            .qs
            .iter()
            .copied()
            .map(|q| Query {
                i: q.i,
                x: cc.compress(q.x),
            })
            .collect_vec();

        let mut existing = xs_cc.iter().copied().collect::<HashBag<_>>();
        let mut missing = (0..cc.space_size())
            .filter(|i| existing.contains(i) == 0)
            .collect::<BTreeSet<_>>();

        let mut ans = vec![];

        for &q in &qs_cc {
            existing.remove(&xs_cc[q.i]);
            if existing.contains(&xs_cc[q.i]) == 0 {
                missing.insert(xs_cc[q.i]);
            }
            xs_cc[q.i] = q.x;
            existing.insert(xs_cc[q.i]);
            missing.remove(&xs_cc[q.i]);
            let mex_cc = *missing.iter().next().unwrap();
            let mex = cc.decompress(mex_cc);
            ans.push(mex);
        }

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // 解法: 長さ n の数列の mex はたかだかn。よって。数列のn以上の値は mex に寄与しない。
        // そう考えると、座標圧縮は不要になる。

        let n = self.n;
        let mut xs = self
            .xs
            .iter()
            .copied()
            .map(|x| i64::min(x, n as i64) as usize)
            .collect_vec();

        let qs = self
            .qs
            .iter()
            .copied()
            .map(|q| Query {
                i: q.i,
                x: i64::min(q.x, n as i64) as usize,
            })
            .collect_vec();

        let mut existing = xs.iter().copied().collect::<HashBag<_>>();
        let mut missing = (0..=n)
            .filter(|i| existing.contains(i) == 0)
            .collect::<BTreeSet<_>>();

        let mut ans = vec![];

        for &q in &qs {
            existing.remove(&xs[q.i]);
            if existing.contains(&xs[q.i]) == 0 {
                missing.insert(xs[q.i]);
            }
            xs[q.i] = q.x;
            existing.insert(xs[q.i]);
            missing.remove(&xs[q.i]);
            let mex = *missing.iter().next().unwrap();
            ans.push(mex as i64);
        }

        Answer { ans }
    }

    fn mex(xs: &[i64]) -> i64 {
        let xs_set = xs.iter().copied().collect::<HashSet<_>>();
        (0..).find(|x| !xs_set.contains(x)).unwrap()
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let mut xs = self.xs.clone();

        let mut ans = vec![];
        for &q in &self.qs {
            xs[q.i] = q.x;
            ans.push(Self::mex(&xs));
        }

        Answer { ans }
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
        let n = rng.gen_range(1..=5);
        let nq = rng.gen_range(1..=20);
        let xs = (0..n).map(|_| rng.gen_range(0..10)).collect_vec();
        let qs = (0..nq)
            .map(|_| {
                let i = rng.gen_range(0..n);
                let x = rng.gen_range(0..10);
                Query { i, x }
            })
            .collect_vec();
        let p = Problem { n, nq, xs, qs };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 100;
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

use ac_library::{Min, Segtree};
use hashbag::HashBag;
// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
use std::collections::BTreeSet;
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

    #[derive(Clone, Debug, PartialEq, Eq)]
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
