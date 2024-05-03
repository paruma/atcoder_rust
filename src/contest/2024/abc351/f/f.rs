//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let xs = &self.xs;
        let xsi = xs
            .iter()
            .copied()
            .enumerate()
            .sorted_by_key(|x| x.1)
            .collect_vec();

        let mut seg = Segbeats::new(xs);

        let mut ans = 0;
        for &(i, x) in &xsi {
            // i+1.. に対して range chmax して range sum する
            seg.change_max(i + 1.., x);
            let addition = seg.query_sum(i + 1..) - x * (n - 1 - i) as i64;
            ans += addition;
        }
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // sum_{i,j in [N], i < j} max(A_j - A_i, 0)
        // = sum_{i,j in [N], i < j} max(A_j,  A_i) - sum_{i in [N]} (N - i - 1) * A_i
        // 第1項は i < j を消して全範囲で計算して、対角消して2で割る

        let n = self.n;
        // i64 だとオーバーフローする
        // (n <= 4*10^5, x <= 10^8 に対して、n * n * x くらいの値が計算されるため)
        let xs = &self.xs.iter().copied().map(|x| x as i128).collect_vec();
        let xs_sorted = xs.iter().copied().sorted().collect_vec();
        // sum_{i,j in [N]} max(A_j,  A_i)
        let term1_all = xs_sorted // 正方形全体
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (2 * i as i128 + 1) * x)
            .sum::<i128>();

        let term1_diag = xs.iter().sum::<i128>(); // 対角の部分
        let term1 = (term1_all - term1_diag) / 2; // 下三角の部分

        let term2 = xs
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (n - i - 1) as i128 * x)
            .sum::<i128>();

        let ans = (term1 - term2) as i64;
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // sum_{i,j in [N], i < j} max(A_j - A_i, 0)
        // = sum_{i,j in [N], i < j} max(A_j,  A_i) - sum_{i in [N]} (N - i - 1) * A_i
        // 第1項は A をソートしても結果は変わらない。ソートしても探索される(A_i, A_j) の組の集合に変化はないため。

        let n = self.n;
        // i64 だとオーバーフローする
        // (n <= 4*10^5, x <= 10^8 に対して、n * n * x くらいの値が計算されるため)
        let xs = &self.xs.iter().copied().map(|x| x as i128).collect_vec();
        let xs_sorted = xs.iter().copied().sorted().collect_vec();
        // sum_{i,j in [N]} max(A_j,  A_i)

        let term1 = xs_sorted
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (i as i128) * x)
            .sum::<i128>();

        let term2 = xs
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (n - i - 1) as i128 * x)
            .sum::<i128>();

        let ans = (term1 - term2) as i64;
        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // 想定解 (転倒数を BIT で計算するのと同じノリ)

        let xs = &self.xs;
        let cc = CoordinateCompression::new(xs);

        let mut bit_cnt = FenwickTree::new(cc.space_size(), 0_usize);
        let mut bit_sum = FenwickTree::new(cc.space_size(), 0_i64);

        let mut ans = 0;

        for x in xs.iter().copied().rev() {
            let x_cc = cc.compress(x);
            bit_sum.add(x_cc, x);
            bit_cnt.add(x_cc, 1);
            let addition = bit_sum.sum(x_cc + 1..) - bit_cnt.sum(x_cc + 1..) as i64 * x;
            ans += addition
        }
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        self.solve()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve4().print();
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
        let main_ans = p.solve2();
        let naive_ans = p.solve();
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

    fn make_random_problem() -> Problem {
        let mut rng = SmallRng::from_entropy();
        let n = 2_usize;
        let xs = (0..n).map(|_| rng.gen_range(0..=10)).collect_vec();
        let p = Problem { n, xs };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 10000;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem();
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

use ac_library::FenwickTree;
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

// https://ngtkana.github.io/ac-adapter-rs/segbeats/struct.Segbeats.html
use std::ops::Bound;
use std::ops::Range;
use std::ops::RangeBounds;

pub fn open(len: usize, range: impl RangeBounds<usize>) -> Range<usize> {
    use Bound::Excluded;
    use Bound::Included;
    use Bound::Unbounded;
    (match range.start_bound() {
        Unbounded => 0,
        Included(&x) => x,
        Excluded(&x) => x + 1,
    })..(match range.end_bound() {
        Excluded(&x) => x,
        Included(&x) => x + 1,
        Unbounded => len,
    })
}
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segbeats<T> {
    len: usize,
    lg: u32,
    table: RefCell<Vec<Node<T>>>,
}

impl<T: Elm> Segbeats<T> {
    pub fn new(src: &[T]) -> Self {
        let len = src.len().next_power_of_two();
        let lg = len.trailing_zeros();
        let mut table = vec![Node::new(); 2 * len];
        for (i, &x) in src.iter().enumerate() {
            table[len + i] = Node::single(x);
        }
        (1..len)
            .rev()
            .for_each(|i| table[i] = Node::merge(table[2 * i], table[2 * i + 1]));
        Self {
            len,
            lg,
            table: RefCell::new(table),
        }
    }

    pub fn change_min(&mut self, range: impl Clone + RangeBounds<usize>, x: T) {
        let range = open(self.len, range);
        self.dfs::<ChangeMin<T>>(range, x)
    }

    pub fn change_max(&mut self, range: impl Clone + RangeBounds<usize>, x: T) {
        let range = open(self.len, range);
        self.dfs::<ChangeMax<T>>(range, x)
    }

    pub fn query_min(&self, range: impl RangeBounds<usize>) -> T {
        let range = open(self.len, range);
        self.dfs::<QueryMin<T>>(range, ())
    }

    pub fn query_max(&self, range: impl RangeBounds<usize>) -> T {
        let range = open(self.len, range);
        self.dfs::<QueryMax<T>>(range, ())
    }

    pub fn query_sum(&self, range: impl RangeBounds<usize>) -> T {
        let range = open(self.len, range);
        self.dfs::<QuerySum<T>>(range, ())
    }

    fn push(&self, i: usize) {
        let x = self.table.borrow()[i].max[0];
        for j in 2 * i..2 * i + 2 {
            let node = self.table.borrow()[j];
            if node.max[1] < x && x < node.max[0] {
                self.table.borrow_mut()[j].change_min(x);
            }
        }
        let x = self.table.borrow()[i].min[0];
        for j in 2 * i..2 * i + 2 {
            let node = self.table.borrow()[j];
            if node.min[0] < x && x < node.min[1] {
                self.table.borrow_mut()[j].change_max(x);
            }
        }
    }

    fn dfs<D: Dfs<Value = T>>(&self, range: Range<usize>, x: D::Param) -> D::Output {
        self.dfs_impl::<D>(1, 0..self.len, range, x)
    }

    fn dfs_impl<D: Dfs<Value = T>>(
        &self,
        root: usize,
        subtree: Range<usize>,
        range: Range<usize>,
        x: D::Param,
    ) -> D::Output {
        if disjoint(&range, &subtree) || D::break_condition(self.table.borrow()[root], x) {
            D::identity()
        } else if contains(&range, &subtree) && D::tag_condition(self.table.borrow()[root], x) {
            D::tag(&mut self.table.borrow_mut()[root], x);
            D::extract(self.table.borrow()[root])
        } else {
            let Range { start, end } = subtree;
            let mid = (start + end) / 2;
            self.push(root);
            let l = self.dfs_impl::<D>(root * 2, start..mid, range.clone(), x);
            let r = self.dfs_impl::<D>(root * 2 + 1, mid..end, range, x);
            self.update(root);
            D::merge(l, r)
        }
    }

    fn update(&self, i: usize) {
        let x = Node::merge(self.table.borrow()[2 * i], self.table.borrow()[2 * i + 1]);
        self.table.borrow_mut()[i] = x;
    }
}

trait Dfs {
    type Value: Elm;
    type Param: Copy + Debug;
    type Output: Debug;
    fn identity() -> Self::Output;
    fn break_condition(_node: Node<Self::Value>, _x: Self::Param) -> bool {
        false
    }
    fn tag_condition(_node: Node<Self::Value>, _x: Self::Param) -> bool {
        true
    }
    fn tag(_node: &mut Node<Self::Value>, _x: Self::Param) {}
    fn merge(left: Self::Output, right: Self::Output) -> Self::Output;
    fn extract(node: Node<Self::Value>) -> Self::Output;
}
struct ChangeMin<T>(std::marker::PhantomData<T>);
impl<T: Elm> Dfs for ChangeMin<T> {
    type Output = ();
    type Param = T;
    type Value = T;

    fn identity() -> Self::Output {}

    fn break_condition(node: Node<T>, x: Self::Param) -> bool {
        node.max[0] <= x
    }

    fn tag_condition(node: Node<T>, x: Self::Param) -> bool {
        node.max[1] < x
    }

    fn tag(node: &mut Node<Self::Value>, x: Self::Param) {
        node.change_min(x);
    }

    fn merge((): (), (): ()) {}

    fn extract(_node: Node<T>) {}
}
struct ChangeMax<T>(std::marker::PhantomData<T>);
impl<T: Elm> Dfs for ChangeMax<T> {
    type Output = ();
    type Param = T;
    type Value = T;

    fn identity() -> Self::Output {}

    fn break_condition(node: Node<T>, x: Self::Param) -> bool {
        x <= node.min[0]
    }

    fn tag_condition(node: Node<T>, x: Self::Param) -> bool {
        x < node.min[1]
    }

    fn tag(node: &mut Node<Self::Value>, x: Self::Param) {
        node.change_max(x);
    }

    fn merge((): (), (): ()) {}

    fn extract(_node: Node<T>) {}
}
struct QueryMin<T>(std::marker::PhantomData<T>);
impl<T: Elm> Dfs for QueryMin<T> {
    type Output = T;
    type Param = ();
    type Value = T;

    fn identity() -> Self::Output {
        T::max_value()
    }

    fn merge(left: T, right: T) -> T {
        left.min(right)
    }

    fn extract(node: Node<T>) -> T {
        node.min[0]
    }
}
struct QueryMax<T>(std::marker::PhantomData<T>);
impl<T: Elm> Dfs for QueryMax<T> {
    type Output = T;
    type Param = ();
    type Value = T;

    fn identity() -> Self::Output {
        T::min_value()
    }

    fn merge(left: T, right: T) -> T {
        left.max(right)
    }

    fn extract(node: Node<T>) -> T {
        node.max[0]
    }
}
struct QuerySum<T>(std::marker::PhantomData<T>);
impl<T: Elm> Dfs for QuerySum<T> {
    type Output = T;
    type Param = ();
    type Value = T;

    fn identity() -> Self::Output {
        T::zero()
    }

    fn merge(left: T, right: T) -> T {
        left + right
    }

    fn extract(node: Node<T>) -> T {
        node.sum
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
struct Node<T> {
    max: [T; 2],
    c_max: u32,
    min: [T; 2],
    c_min: u32,
    sum: T,
}
impl<T: Elm> Node<T> {
    fn new() -> Self {
        Self {
            max: [T::min_value(), T::min_value()],
            c_max: 0,
            min: [T::max_value(), T::max_value()],
            c_min: 0,
            sum: T::zero(),
        }
    }

    fn single(x: T) -> Self {
        Self {
            max: [x, T::min_value()],
            c_max: 1,
            min: [x, T::max_value()],
            c_min: 1,
            sum: x,
        }
    }

    fn change_min(&mut self, x: T) {
        assert!(self.max[1] < x && x < self.max[0]);
        self.sum += (x - self.max[0]).mul_u32(self.c_max);
        for i in 0..2 {
            if self.min[i] == self.max[0] {
                self.min[i] = x;
            }
        }
        self.max[0] = x;
    }

    fn change_max(&mut self, x: T) {
        assert!(self.min[0] < x && x < self.min[1]);
        self.sum += (x - self.min[0]).mul_u32(self.c_min);
        for i in 0..2 {
            if self.max[i] == self.min[0] {
                self.max[i] = x;
            }
        }
        self.min[0] = x;
    }

    fn merge(left: Self, right: Self) -> Self {
        use std::cmp::Ordering;
        let (max, c_max) = {
            let [a, b] = left.max;
            let [c, d] = right.max;
            match a.cmp(&c) {
                Ordering::Equal => ([a, b.max(d)], left.c_max + right.c_max),
                Ordering::Greater => ([a, b.max(c)], left.c_max),
                Ordering::Less => ([c, a.max(d)], right.c_max),
            }
        };
        let (min, c_min) = {
            let [a, b] = left.min;
            let [c, d] = right.min;
            match a.cmp(&c) {
                Ordering::Equal => ([a, b.min(d)], left.c_min + right.c_min),
                Ordering::Less => ([a, b.min(c)], left.c_min),
                Ordering::Greater => ([c, a.min(d)], right.c_min),
            }
        };
        Self {
            max,
            c_max,
            min,
            c_min,
            sum: left.sum + right.sum,
        }
    }
}

fn contains(i: &Range<usize>, j: &Range<usize>) -> bool {
    i.start <= j.start && j.end <= i.end
}
fn disjoint(i: &Range<usize>, j: &Range<usize>) -> bool {
    i.end <= j.start || j.end <= i.start
}

pub trait Elm:
    Sized
    + std::fmt::Debug
    + Copy
    + Ord
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
{
    fn max_value() -> Self;
    fn min_value() -> Self;
    fn zero() -> Self;
    fn mul_u32(&self, x: u32) -> Self;
}
macro_rules! impl_elm {
    {$($ty:ident;)*} => {
        $(
            impl Elm for $ty {
                fn min_value() -> Self {
                    std::$ty::MIN
                }
                fn max_value() -> Self {
                    std::$ty::MAX
                }
                fn zero() -> Self {
                    0
                }
                fn mul_u32(&self, x: u32) -> Self {
                    self * (x as $ty)
                }
            }
        )*
    }
}
impl_elm! {
    u8; u16; u32; u64; u128; usize;
    i8; i16; i32; i64; i128; isize;
}

// #[cfg(test)]
// mod tests {
//     mod impl_query;
//     mod queries;
//     mod vector;
//
//     use super::Segbeats;
//     use queries::{ChangeMax, ChangeMin, QueryMax, QuerySum};
//     use query_test::{impl_help, Config};
//     use rand::prelude::*;
//     use vector::{Len, Value, Vector};
//
//     type Tester<T, G> = query_test::Tester<StdRng, Vector<T>, Segbeats<T>, G>;
//
//     #[test]
//     fn test_i64() {
//         #[derive(Debug, Clone, PartialEq, Copy, Eq)]
//         struct G {}
//         impl_help! {Len, |rng| rng.gen_range(1..100); }
//         impl_help! {Value<i64>, |rng| rng.gen_range(-1_000_000_000, 1_000_000_000); }
//
//         let mut tester = Tester::<i64, G>::new(StdRng::seed_from_u64(42), Config::Short);
//         for _ in 0..10 {
//             tester.initialize();
//             for _ in 0..100 {
//                 let command = tester.rng_mut().gen_range(0..4);
//                 match command {
//                     0 => tester.mutate::<ChangeMin<_>>(),
//                     1 => tester.mutate::<ChangeMax<_>>(),
//                     2 => tester.compare::<QueryMax<_>>(),
//                     3 => tester.compare::<QuerySum<_>>(),
//                     _ => unreachable!(),
//                 }
//             }
//         }
//     }
// }

// ===========
use coordinate_compression::*;
pub mod coordinate_compression {
    use itertools::Itertools;
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
