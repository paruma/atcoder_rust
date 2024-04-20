//#[derive_readable]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range {
    begin: usize,
    end: usize,
}

use ac_library::LazySegtree;
use map_monoid_template::*;
#[allow(unused_variables)]
pub mod map_monoid_template {
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub max: i64,
    }
    impl RangeXxx {
        pub fn unit(x: i64) -> Self {
            Self { max: x }
        }
    }
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx { max: -1 }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeXxx {
                max: i64::max(a.max, b.max),
            }
        }
    }
    pub struct RangeYyyRangeXxx(Infallible);
    impl MapMonoid for RangeYyyRangeXxx {
        type M = RangeXxxMonoid;
        type F = bool;
        fn identity_map() -> Self::F {
            false
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            RangeXxx {
                max: if *f { -x.max } else { x.max },
            }
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f ^ g
        }
    }
}

impl Range {
    fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }
}

#[derive(Debug)]
struct Problem {
    s: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            s: Bytes
        }
        Problem { s }
    }
    fn solve(&self) -> Answer {
        let s = &self.s;
        let s_without_paren = s
            .iter()
            .copied()
            .filter(|ch| *ch != b'(' && *ch != b')')
            .collect_vec();

        let mut stack: Stack<usize> = Stack::new();
        let mut cur = 0_usize;
        let mut range = vec![];

        for &ch in s {
            match ch {
                b'(' => stack.push(cur),
                b')' => {
                    let begin = stack.pop().unwrap();
                    let end = cur;
                    range.push(Range::new(begin, end))
                }
                _ => cur += 1,
            }
        }
        dbg!(&range);

        let xs = s_without_paren
            .iter()
            .copied()
            .map(|x| if (b'a'..=b'z').contains(&x) { -1 } else { 1 })
            .collect_vec();

        let mut seg = LazySegtree::<RangeYyyRangeXxx>::from(
            xs.iter().copied().map(|x| RangeXxx::unit(x)).collect_vec(),
        );

        for r in range.clone() {
            seg.apply_range(r.begin..r.end, true);
        }

        let s_without_paren2 = s_without_paren
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| {
                if seg.get(i).max == -1 {
                    x.to_ascii_lowercase()
                } else {
                    x.to_ascii_uppercase()
                }
            })
            .collect_vec();

        dbg!(String::from_utf8(s_without_paren2.clone()).unwrap());
        let mut seg2 = LazySegtree::<RangeAffineRangeSum<i64>>::from(
            (0..s_without_paren2.len())
                .map(|i| RangeSum::unit(i as i64))
                .collect_vec(),
        );

        // 4 5 6, begin=4, end=7
        // 4 → 6
        // 5 → 5
        // 6 → 4

        //for r in range.iter().copied().sorted_by_key(|r| r.end) {
        for r in range.clone() {
            seg2.apply_range(
                r.begin..r.end,
                Affine {
                    slope: -1,
                    intercept: (r.begin + r.end) as i64 - 1,
                },
            );
            dbg!((0..s_without_paren2.len())
                .map(|i| seg2.get(i).sum)
                .join(","));
        }

        let ans = (0..s_without_paren2.len())
            .map(|i| {
                let x = seg2.get(i).sum as usize;
                dbg!(x);
                s_without_paren2[x]
            })
            .collect_vec();

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
    ans: Vec<u8>,
}

impl Answer {
    fn print(&self) {
        println!("{}", std::str::from_utf8(&self.ans).unwrap());
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

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

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

use mod_stack::*;
pub mod mod_stack {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Stack<T> {
        raw: Vec<T>,
    }
    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack { raw: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.last()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Stack<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

use range_affine_range_sum::*;
pub mod range_affine_range_sum {
    use ac_library::{MapMonoid, Monoid};
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSum<T> {
        pub sum: T,
        pub pos: T,
        pub len: i64,
    }
    impl<T> RangeSum<T>
    where
        T: Copy,
    {
        pub fn unit(x: T) -> RangeSum<T> {
            RangeSum {
                sum: x,
                pos: x,
                len: 1,
            }
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
                pos: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: a.sum + b.sum,
                pos: a.pos + b.pos, // ダミー
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
                sum: f.slope * x.pos + f.intercept * x.len.into(),
                pos: x.pos,
                len: x.len,
            }
        }
    }
}
