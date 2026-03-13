//#[derive_readable]
enum Query {
    Change { p: usize, x: i64 },
    Output { l: usize, r: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ElemCount {
    val: i64,
    cnt: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SegtreeEntry {
    first: Option<ElemCount>,
    second: Option<ElemCount>,
}

impl SegtreeEntry {
    fn unit(x: i64) -> SegtreeEntry {
        SegtreeEntry {
            first: Some(ElemCount { val: x, cnt: 1 }),
            second: None,
        }
    }

    fn second_count(&self) -> i64 {
        self.second.map(|x| x.cnt).unwrap_or(0)
    }
}

struct Concat(Infallible);
impl Monoid for Concat {
    type S = SegtreeEntry;
    fn identity() -> Self::S {
        SegtreeEntry {
            first: None,
            second: None,
        }
    }
    /*
    fn binary_operation_old(a: &Self::S, b: &Self::S) -> Self::S {
        let tmp = [a.first, a.second, b.first, b.second]
            .iter()
            .copied()
            .flatten()
            .sorted_by_key(|x| Reverse(x.val))
            .group_by(|x| x.val)
            .into_iter()
            .map(|(elem, group)| ElemCount { val: elem, cnt: group.map(|x| x.cnt).sum::<i64>() })
            .collect_vec();
        let first = tmp.get(0).copied();
        let second = tmp.get(1).copied();

        SegtreeEntry { first, second }
    }
     */

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let tmp = [a.first, a.second, b.first, b.second]
            .into_iter()
            .flatten()
            .collect_vec();
        let first_val = tmp.iter().map(|x| x.val).max();
        let second_val = first_val.and_then(|first_val| {
            tmp.iter()
                .filter(|x| x.val != first_val)
                .map(|x| x.val)
                .max()
        });

        let first = first_val.map(|first_val| ElemCount {
            val: first_val,
            cnt: tmp
                .iter()
                .filter(|x| x.val == first_val)
                .map(|x| x.cnt)
                .sum::<i64>(),
        });

        let second = second_val.map(|second_val| ElemCount {
            val: second_val,
            cnt: tmp
                .iter()
                .filter(|x| x.val == second_val)
                .map(|x| x.cnt)
                .sum::<i64>(),
        });

        SegtreeEntry { first, second }
    }
}

struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<i64>,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: [i64; n],
            qs: [(i64, i64, i64); nq]
        }

        let qs = qs
            .iter()
            .copied()
            .map(|(t, x, y)| {
                if t == 1 {
                    Query::Change {
                        p: x as usize - 1,
                        x: y,
                    }
                } else {
                    Query::Output {
                        l: x as usize - 1,
                        r: y as usize - 1,
                    }
                }
            })
            .collect_vec();
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
            match q {
                Query::Change { p, x } => {
                    let e = SegtreeEntry::unit(*x);
                    xs_segtree.set(*p, e);
                }
                Query::Output { l, r } => {
                    let l = *l;
                    let r = *r;
                    let e = xs_segtree.prod(l..=r);
                    ans.push(e.second_count())
                }
            }
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
