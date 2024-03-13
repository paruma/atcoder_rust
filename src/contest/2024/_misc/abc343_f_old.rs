//#[derive_readable]
enum Query {
    Change { p: usize, x: i64 },
    Output { l: usize, r: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SegtreeEntry {
    first: Option<i64>,
    first_cnt: i64,
    second: Option<i64>,
    second_cnt: i64,
}

impl SegtreeEntry {
    fn unit(x: i64) -> SegtreeEntry {
        SegtreeEntry {
            first: Some(x),
            first_cnt: 1,
            second: None,
            second_cnt: 0,
        }
    }
}

struct Concat(Infallible);
impl Monoid for Concat {
    type S = SegtreeEntry;
    fn identity() -> Self::S {
        SegtreeEntry {
            first: None,
            first_cnt: 0,
            second: None,
            second_cnt: 0,
        }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let mut ret = Self::identity();
        // 一番大きい値/2番目に大きい値を先に求めてから、カウントの合計値を求める方法でも良かったかもしれない。
        for (elem, cnt) in [
            (a.first, a.first_cnt),
            (a.second, a.second_cnt),
            (b.first, b.first_cnt),
            (b.second, b.second_cnt),
        ] {
            if let Some(elem) = elem {
                match ret.first {
                    Some(x) => {
                        if x == elem {
                            ret.first_cnt += cnt;
                        } else if x < elem {
                            ret.second = ret.first;
                            ret.second_cnt = ret.first_cnt;
                            ret.first = Some(elem);
                            ret.first_cnt = cnt;
                        } else {
                            match ret.second {
                                Some(y) => {
                                    if y == elem {
                                        ret.second_cnt += cnt;
                                    } else if y < elem {
                                        ret.second = Some(elem);
                                        ret.second_cnt = cnt;
                                    }
                                }
                                None => {
                                    ret.second = Some(elem);
                                    ret.second_cnt = cnt;
                                }
                            }
                        }
                    }
                    None => {
                        ret.first = Some(elem);
                        ret.first_cnt = cnt;
                    }
                }
            }
        }

        ret
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
                    ans.push(e.second_cnt)
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

use std::{collections::HashMap, convert::Infallible, marker::PhantomData};

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
