//#[derive_readable]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Query {
    RangeReverse { l: usize, r: usize },
    RangeInversion { l: usize, r: usize },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InversionInfo {
    inversion: usize,
    count0: usize,
    count1: usize,
}

impl InversionInfo {
    fn unit(x: i64) -> Self {
        Self {
            inversion: 0,
            count0: (x == 0) as usize,
            count1: (x == 1) as usize,
        }
    }
}

struct InversionMonoid(Infallible);
impl Monoid for InversionMonoid {
    type S = InversionInfo;

    fn identity() -> Self::S {
        InversionInfo {
            inversion: 0,
            count0: 0,
            count1: 0,
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        // a       b
        // [001101][11010]
        // ↓ a, b それぞれでソート (aの転倒数 + bの転倒数)
        // [000111][00111]
        // ↓bにある0 を左、aにある1を右に (bの0の個数 * aの1の個数)
        // [00000111111]

        InversionInfo {
            inversion: a.inversion + b.inversion + b.count0 * a.count1,
            count0: a.count0 + b.count0,
            count1: a.count1 + b.count1,
        }
    }
}

struct RangeReverseRangeInversion(Infallible);
impl MapMonoid for RangeReverseRangeInversion {
    type M = InversionMonoid;

    type F = bool;

    fn identity_map() -> Self::F {
        false
    }

    fn mapping(
        f: &Self::F,
        x: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        if *f {
            // 0, 1反転する
            // [11010] (転倒数5)
            // [00101] (転倒数1)

            InversionInfo {
                inversion: x.count0 * x.count1 - x.inversion,
                count0: x.count1,
                count1: x.count0,
            }
            // inversion: x.count0 * x.count1 - x.inversion の説明
            // [00111] (転倒数0) : ↓との差が普通の転倒数
            // [11010] (転倒数5) : ↓との差が反転の転倒数
            // [11100] (転倒数6) : 0の数*1の数
        } else {
            // 0, 1 反転しない
            *x
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f ^ g
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
            qs: [(usize, Usize1, Usize1); nq],
        }

        let qs = qs
            .iter()
            .copied()
            .map(|(t, l, r)| {
                if t == 1 {
                    Query::RangeReverse { l, r }
                } else {
                    Query::RangeInversion { l, r }
                }
            })
            .collect_vec();
        Problem { n, nq, xs, qs }
    }
    fn solve(&self) -> Answer {
        let mut segtree = LazySegtree::<RangeReverseRangeInversion>::from(
            self.xs
                .iter()
                .copied()
                .map(InversionInfo::unit)
                .collect_vec(),
        );

        let mut ans = vec![];
        for &q in &self.qs {
            match q {
                Query::RangeReverse { l, r } => segtree.apply_range(l..=r, true),
                Query::RangeInversion { l, r } => {
                    ans.push(segtree.prod(l..=r).inversion);
                }
            }
        }

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
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

use ac_library::{LazySegtree, MapMonoid, Monoid};
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
