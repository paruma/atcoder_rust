// 1点更新のたびに最大の区間和出力
//
#[derive_readable]
struct Query {
    idx: usize,
    val: i64,
}
struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<i64>,
    qs: Vec<Query>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SegtreeEntry {
    prefix_sum_max: i64,
    internal_sum_max: i64,
    suffix_sum_max: i64,
    sum: i64,
}

impl SegtreeEntry {
    fn unit(x: i64) -> SegtreeEntry {
        SegtreeEntry {
            prefix_sum_max: x,
            internal_sum_max: x,
            suffix_sum_max: x,
            sum: x,
        }
    }
}

struct Concat(Infallible);
impl Monoid for Concat {
    type S = SegtreeEntry;
    fn identity() -> Self::S {
        SegtreeEntry {
            prefix_sum_max: 0,
            internal_sum_max: 0,
            suffix_sum_max: 0,
            sum: 0,
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        SegtreeEntry {
            prefix_sum_max: i64::max(a.prefix_sum_max, a.sum + b.prefix_sum_max),
            internal_sum_max: {
                *[
                    a.internal_sum_max,
                    b.internal_sum_max,
                    a.suffix_sum_max + b.prefix_sum_max,
                ]
                .iter()
                .max()
                .unwrap()
            },
            suffix_sum_max: i64::max(b.suffix_sum_max, b.sum + a.suffix_sum_max),
            sum: a.sum + b.sum,
        }
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: [i64; n],
            qs: [Query; nq],
        }
        Problem { n, nq, xs, qs }
    }
    fn solve(&self) -> Answer {
        let mut xs_segtree = Segtree::<Concat>::from(
            self.xs
                .iter()
                .copied()
                .map(SegtreeEntry::unit)
                .collect_vec(),
        );
        let mut ans = vec![];
        for q in &self.qs {
            ans.push(xs_segtree.all_prod().internal_sum_max);
            xs_segtree.set(q.idx, SegtreeEntry::unit(q.val));
        }
        ans.push(xs_segtree.all_prod().internal_sum_max);
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

use std::convert::Infallible;

use ac_library::{Monoid, Segtree};
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
