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

        let xs = xs.iter().copied().map(Mint::new).collect_vec();

        let mut segtree = dual_segtree::DualSegtree::<RangeAffine<Mint>>::from(xs);

        for op in ops {
            let prob = Mint::new(op.end - op.begin).inv();
            let affine = Affine {
                slope: Mint::new(1) - prob,
                intercept: prob * Mint::new(op.value),
            };
            segtree.apply_range(op.begin..op.end, affine);
        }

        let ans = (0..*len).map(|i| segtree.get(i)).collect_vec();
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
                // TODO: There are another way of optimizing [0..r)
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

use range_affine::*;
pub mod range_affine {
    use super::dual_segtree::*;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul};

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

    pub struct RangeAffine<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAffine<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type F = Affine<T>;
        type S = T;
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
        fn mapping(f: &Affine<T>, x: &T) -> T {
            f.slope * *x + f.intercept
        }
    }
}
