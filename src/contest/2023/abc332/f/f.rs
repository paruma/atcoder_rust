//#[derive_readable]

use std::convert::Infallible;
use std::ops::{Add, Mul};

use ac_library::lazysegtree::MapMonoid;
use ac_library::{ModInt998244353 as Mint, Monoid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AffineInput {
    value: Mint,
    times: usize,
}
struct AffineInputAdd(Infallible);
impl Monoid for AffineInputAdd {
    type S = AffineInput;
    fn identity() -> Self::S {
        AffineInput {
            value: 0.into(),
            times: 0,
        }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        AffineInput {
            value: a.value + b.value,
            times: a.times + b.times,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct AffineTransform<T> {
    slope: T,
    intercept: T,
}
impl<T> AffineTransform<T> {
    pub fn new(slope: T, intercept: T) -> Self {
        Self { slope, intercept }
    }

    pub fn apply(&self, x: T) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self.slope * x + self.intercept
    }

    pub fn identity() -> Self
    where
        T: From<i64>,
    {
        Self {
            slope: 1.into(),
            intercept: 0.into(),
        }
    }

    pub fn composite(&self, rhs: &Self) -> Self
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        Self {
            slope: self.slope * rhs.slope,
            intercept: self.slope * rhs.intercept + self.intercept,
        }
    }
}

struct AddAffine(Infallible);
impl MapMonoid for AddAffine {
    type M = AffineInputAdd;
    type F = AffineTransform<Mint>;

    fn identity_map() -> AffineTransform<Mint> {
        Self::F::identity()
    }

    fn mapping(&f: &AffineTransform<Mint>, &x: &AffineInput) -> AffineInput {
        AffineInput {
            value: f.slope * x.value + f.intercept * x.times,
            times: x.times,
        }
    }

    fn composition(
        &f: &AffineTransform<Mint>,
        &g: &AffineTransform<Mint>,
    ) -> AffineTransform<Mint> {
        f.composite(&g)
    }
}

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

use ac_library::LazySegtree;

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
            .map(|x| AffineInput {
                value: Mint::new(x),
                times: 1,
            })
            .collect_vec();
        let mut segtree = LazySegtree::<AddAffine>::from(xs);

        for op in ops {
            let prob = Mint::new(op.end - op.begin).inv();
            let affine = AffineTransform {
                slope: Mint::new(1) - prob,
                intercept: prob * Mint::new(op.value),
            };
            segtree.apply_range(op.begin..op.end, affine);
        }

        let ans = (0..*len).map(|i| segtree.get(i).value).collect_vec();
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
