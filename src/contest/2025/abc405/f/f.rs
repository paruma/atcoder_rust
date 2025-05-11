#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ring {
    len: usize,
}

impl Ring {
    fn new(len: usize) -> Ring {
        Ring { len }
    }
    fn inc(&self, x: usize) -> usize {
        (x + 1) % self.len
    }

    fn dec(&self, x: usize) -> usize {
        (x + self.len - 1) % self.len
    }
    /// 時計回りに src から dst に移動したときの道のり
    fn dist_right(&self, src: usize, dst: usize) -> usize {
        assert!((0..self.len).contains(&src));
        assert!((0..self.len).contains(&dst));
        if src > dst {
            dst + self.len - src
        } else {
            dst - src
        }
    }

    /// 反時計回りに src から dst に移動したときの道のり
    fn dist_left(&self, src: usize, dst: usize) -> usize {
        assert!((0..self.len).contains(&src));
        assert!((0..self.len).contains(&dst));
        self.dist_right(dst, src)
    }

    /// begin から end に時計回りに回ったときに x に当たるかどうか
    fn contains(&self, begin: usize, end: usize, x: usize) -> bool {
        assert!((0..self.len).contains(&begin));
        assert!((0..self.len).contains(&end));
        if end < begin {
            (begin..end + self.len).contains(&x)
                || (begin..end + self.len).contains(&(x + self.len))
        } else {
            (begin..end).contains(&x)
        }
    }
}

struct CircularRangeAddSegtree {
    n: usize,
    seg: DualSegtree<AddMonoid>,
}

impl CircularRangeAddSegtree {
    fn new(n: usize) -> Self {
        let seg = DualSegtree::new(n);
        Self { n, seg }
    }

    fn range_add(&mut self, begin: usize, end: usize, x: i64) {
        if begin <= end {
            self.seg.apply_range(begin..end, x);
        } else {
            let range1 = begin..self.n;
            let range2 = 0..end;
            self.seg.apply_range(range1, x);
            self.seg.apply_range(range2, x);
        }
    }

    fn get(&mut self, i: usize) -> i64 {
        self.seg.get(i)
    }

    fn to_vec(&mut self) -> Vec<i64> {
        (0..self.n).map(|i| self.get(i)).collect_vec()
    }
}

fn solve_naive(n_point_div2: usize, ls: &[(usize, usize)], qs: &[(usize, usize)]) -> Vec<i64> {
    let n_point = n_point_div2 * 2;
    qs.iter()
        .copied()
        .map(|(c, d)| {
            let (c, d) = (min(c, d), max(c, d));
            ls.iter()
                .copied()
                .filter(|&(a, b)| {
                    let (a, b) = (min(a, b), max(a, b));
                    // 区間 (a,b) と 区間(b, a + n_point) を考える
                    let range_ab = a + 1..b;

                    let c_in_ab = range_ab.contains(&c) || range_ab.contains(&(c + n_point));
                    let d_in_ab = range_ab.contains(&d) || range_ab.contains(&(d + n_point));

                    c_in_ab ^ d_in_ab
                })
                .count() as i64
        })
        .collect_vec()
}

fn solve(n_point_div2: usize, ls: &[(usize, usize)], qs: &[(usize, usize)]) -> Vec<i64> {
    let ls = ls
        .iter()
        .copied()
        .map(|(a, b)| (min(a, b), (max(a, b))))
        .collect_vec();
    let qs = qs
        .iter()
        .copied()
        .map(|(a, b)| (min(a, b), (max(a, b))))
        .collect_vec();

    let n_point = n_point_div2 * 2;
    // 一方の端点を与えたらもう一方の端点を返す
    let line_map =
        ls.iter()
            .copied()
            .enumerate()
            .fold(vec![vec![]; n_point], |mut acc, (line_id, (a, b))| {
                acc[a].push((line_id, b));
                acc[b].push((line_id, a));
                acc
            });

    let mut counts = CircularRangeAddSegtree::new(n_point);
    let ring = Ring::new(n_point);

    // counts の初期値を決める
    {
        let mut line_visited = vec![false; ls.len()];
        for a in 0..n_point {
            for &(line_id, b) in &line_map[a] {
                if line_visited[line_id] {
                    continue;
                }
                // 0 を含まない方の区間を考える
                let (begin, end) = if ring.contains(a, b, 0) {
                    (b, a)
                } else {
                    (a, b)
                };

                line_visited[line_id] = true;

                counts.range_add((begin + 1) % n_point, end, 1);
            }
        }
    }

    let mut ans = vec![-1; qs.len()];
    let query_map = qs.iter().copied().enumerate().fold(
        vec![vec![]; n_point],
        |mut acc, (query_id, (a, b))| {
            acc[a].push((query_id, b));
            acc
        },
    );

    for &(query_id, d) in &query_map[0] {
        ans[query_id] = counts.get(d);
    }

    for a in 1..n_point {
        for &(_, b) in &line_map[a] {
            counts.range_add((a + 1) % n_point, b, -1);
            counts.range_add((b + 1) % n_point, a, 1);
        }

        for &(query_id, d) in &query_map[a] {
            ans[query_id] = counts.get(d);
        }
    }
    ans
}
fn main() {
    input! {
        n_point_div2: usize,
        n_line: usize,
        ls: [(Usize1, Usize1); n_line],
        q: usize,
        qs: [(Usize1, Usize1); q],
    }
    let ans: Vec<i64> = solve(n_point_div2, &ls, &qs);
    print_vec(&ans);
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
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {

    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
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
use dual_segtree::*;
#[allow(clippy::module_inception)]
pub mod dual_segtree {
    use std::ops::{Bound, RangeBounds};
    fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }
    pub trait MapMonoid {
        type F: Clone;
        type S: Clone;
        fn identity_map() -> Self::F;
        fn mapping(f: &Self::F, x: &Self::S) -> Self::S;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }
    impl<F: MapMonoid> Default for DualSegtree<F>
    where
        F::S: Default,
    {
        fn default() -> Self {
            Self::new(0)
        }
    }
    impl<F: MapMonoid> DualSegtree<F> {
        pub fn new(n: usize) -> Self
        where
            F::S: Default,
        {
            vec![F::S::default(); n].into()
        }
    }
    impl<F: MapMonoid> From<Vec<F::S>> for DualSegtree<F>
    where
        F::S: Default,
    {
        fn from(v: Vec<F::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![F::S::default(); size];
            let lz = vec![F::identity_map(); size];
            d[..n].clone_from_slice(&v);
            DualSegtree {
                n,
                size,
                log,
                d,
                lz,
            }
        }
    }
    impl<F: MapMonoid> DualSegtree<F> {
        pub fn set(&mut self, p: usize, x: F::S) {
            assert!(p < self.n);
            for i in (1..=self.log).rev() {
                self.push((p + self.size) >> i);
            }
            self.d[p] = x;
        }
        pub fn get(&mut self, p: usize) -> F::S {
            assert!(p < self.n);
            for i in (1..=self.log).rev() {
                self.push((p + self.size) >> i);
            }
            self.d[p].clone()
        }
        pub fn apply(&mut self, p: usize, f: F::F) {
            assert!(p < self.n);
            for i in (1..=self.log).rev() {
                self.push((p + self.size) >> i);
            }
            self.d[p] = F::mapping(&f, &self.d[p]);
        }
        pub fn apply_range<R>(&mut self, range: R, f: F::F)
        where
            R: RangeBounds<usize>,
        {
            let mut r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let mut l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                Bound::Unbounded => 0,
            };
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }
            l += self.size;
            r += self.size;
            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }
            {
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
            }
        }
        pub fn to_vec(&mut self) -> Vec<F::S> {
            (0..self.n).map(|i| self.get(i)).collect()
        }
    }
    pub struct DualSegtree<F>
    where
        F: MapMonoid,
    {
        n: usize,
        size: usize,
        log: usize,
        d: Vec<F::S>,
        lz: Vec<F::F>,
    }
    impl<F> DualSegtree<F>
    where
        F: MapMonoid,
    {
        fn all_apply(&mut self, k: usize, f: F::F) {
            if k < self.size {
                self.lz[k] = F::composition(&f, &self.lz[k]);
            } else {
                self.d[k - self.size] = F::mapping(&f, &self.d[k - self.size]);
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lz[k].clone());
            self.all_apply(2 * k + 1, self.lz[k].clone());
            self.lz[k] = F::identity_map();
        }
    }
}

use range_add::AddMonoid;
pub mod range_add {
    use super::dual_segtree::*;
    use std::convert::Infallible;

    pub struct AddMonoid(Infallible);
    impl MapMonoid for AddMonoid {
        type F = i64;
        type S = i64;

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(&f: &i64, &x: &i64) -> i64 {
            f + x
        }

        fn composition(&f: &i64, &g: &i64) -> i64 {
            f + g
        }
    }
}
