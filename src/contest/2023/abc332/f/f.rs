#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Op {
    begin: usize,
    end: usize,
    value: i64,
}
struct Problem {
    len: usize,
    n_ops: usize,
    xs: Vec<i64>,
    ops: Vec<Op>,
}

use ac_library::ModInt998244353 as Mint;

impl Problem {
    fn read() -> Problem {
        input! {
            len: usize,
            n_ops: usize,
            xs: [i64; len],
            ops: [(usize, usize, i64); n_ops],
        }

        let ops = ops
            .iter()
            .copied()
            .map(|(left, right, value)| Op {
                begin: left - 1,
                end: right,
                value,
            })
            .collect_vec();
        Problem {
            len,
            n_ops,
            xs,
            ops,
        }
    }
    fn solve(&self) -> Answer {
        let Problem {
            len,
            n_ops,
            xs,
            ops,
        } = self;

        let xs = xs
            .iter()
            .copied()
            .map(|x| RangeSum::unit(x.into()))
            .collect_vec();
        let mut segtree = LazySegtree::<RangeAffineRangeSum<Mint>>::from(xs);

        for op in ops {
            let prob = Mint::new(op.end - op.begin).inv();
            let affine = Affine {
                slope: Mint::new(1) - prob,
                intercept: prob * Mint::new(op.value),
            };
            segtree.apply_range(op.begin..op.end, affine);
        }

        let ans = (0..*len).map(|i| segtree.get(i).sum).collect_vec();
        Answer { ans }
    }
    fn solve_wrong(&self) -> Answer {
        let Problem {
            len,
            n_ops,
            xs,
            ops,
        } = self;

        // (index, 確率, 値, in(1)/out(0))
        let mut pos_to_ops = vec![vec![]; len + 1];

        for (i, op) in ops.iter().enumerate() {
            let prob = (Mint::new(op.end) - Mint::new(op.begin)).inv();

            pos_to_ops[op.begin].push((i, prob, op.value, 1));
            pos_to_ops[op.end].push((i, prob, op.value, 0));
        }

        let mut imos1 = vec![Mint::new(0); len + 1];
        let mut imos2 = vec![Mint::new(1); len + 1];

        for pos in 0..*len {
            if pos != 0 {
                imos1[pos] = imos1[pos - 1];
                imos2[pos] = imos1[pos - 1];
            }
            for (i, prob, value, kind) in pos_to_ops[pos].iter().sorted_by_key(|x| x.0) {
                if *kind == 1 {
                    // in
                    imos1[pos] = prob * Mint::new(*value) + (Mint::new(1) - prob) * imos1[pos];
                    imos2[pos] *= Mint::new(1) - prob;
                } else {
                    // out
                    imos1[pos] = (imos1[pos] - prob * Mint::new(*value)) / (Mint::new(1) - prob);
                    imos2[pos] /= Mint::new(1) - prob;
                }
            }
        }
        let ans = (0..*len).map(|i| imos1[i] + imos2[i] * xs[i]).collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<Mint>,
}

impl Answer {
    fn print(&self) {
        print_vec_1line(&self.ans.iter().copied().map(|x| x.val()).collect_vec());
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use ac_library::LazySegtree;
// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

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

use range_affine_range_sum::*;
pub mod range_affine_range_sum {
    use ac_library::{MapMonoid, Monoid};
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul};
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
}
