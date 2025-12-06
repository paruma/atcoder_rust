// コンテスト中の解法。
// two sequence range affine range sum を使った
// 1列目に座圧したマスの数
// 2列目に白か黒か
// を持たせて、黒のマスの数を求めた
#[fastout]
fn main() {
    input! {
        n: i64,
        q: usize,
        lrs: [(i64, i64); q],
    }

    // 開区間にしてみる
    let lrs = lrs.iter().copied().map(|(l, r)| (l, r + 1)).collect_vec();

    let coord = {
        let mut coord = vec![1, n];
        for &(l, r) in &lrs {
            coord.push(l);
            coord.push(r);
        }
        coord.sort();
        coord.dedup();
        coord
    };

    let cnts = coord
        .iter()
        .copied()
        .tuple_windows()
        .map(|(l, r)| r - l)
        .collect_vec();

    let n_range = cnts.len();

    let mut seg = TwoSequenceRangeAffineRangeSumSegtree::new(&vec![0; n_range], &cnts);

    let black_sum = seg.query_sum_xy(..);
    // dbg!(black_sum);

    for &(l, r) in &lrs {
        let li = coord.binary_search(&l).unwrap();
        let ri = coord.binary_search(&r).unwrap();
        seg.apply_range_affine_x(li..ri, 0, 1);
        let black_sum = seg.query_sum_xy(..);
        let ans = n - black_sum;
        println!("{}", ans);
    }

    // dbg!(cnts);

    // dbg!(coord);
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

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
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
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
use two_sequence_range_affine_range_sum::*;
#[allow(clippy::module_inception)]
pub mod two_sequence_range_affine_range_sum {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TwoSequenceData<T> {
        pub sum_x: T,
        pub sum_y: T,
        pub sum_xy: T,
        pub len: i64,
    }
    impl<T> TwoSequenceData<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        pub fn unit(x_val: T, y_val: T) -> Self {
            Self {
                sum_x: x_val,
                sum_y: y_val,
                sum_xy: x_val * y_val,
                len: 1,
            }
        }
    }
    pub struct TwoSequenceDataMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for TwoSequenceDataMonoid<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type S = TwoSequenceData<T>;
        fn identity() -> Self::S {
            Self::S {
                sum_x: 0.into(),
                sum_y: 0.into(),
                sum_xy: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            Self::S {
                sum_x: a.sum_x + b.sum_x,
                sum_y: a.sum_y + b.sum_y,
                sum_xy: a.sum_xy + b.sum_xy,
                len: a.len + b.len,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TwoSequenceAffine<T> {
        pub a: T,
        pub b: T,
        pub c: T,
        pub d: T,
    }
    pub struct TwoSequenceRangeAffineRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for TwoSequenceRangeAffineRangeSum<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type M = TwoSequenceDataMonoid<T>;
        type F = TwoSequenceAffine<T>;
        fn identity_map() -> Self::F {
            Self::F {
                a: 1.into(),
                b: 0.into(),
                c: 1.into(),
                d: 0.into(),
            }
        }
        fn composition(f1: &Self::F, f2: &Self::F) -> Self::F {
            Self::F {
                a: f1.a * f2.a,
                b: f1.a * f2.b + f1.b,
                c: f1.c * f2.c,
                d: f1.c * f2.d + f1.d,
            }
        }
        fn mapping(f: &Self::F, x: &TwoSequenceData<T>) -> TwoSequenceData<T> {
            TwoSequenceData {
                sum_xy: f.a * f.c * x.sum_xy
                    + f.a * f.d * x.sum_x
                    + f.b * f.c * x.sum_y
                    + f.b * f.d * x.len.into(),
                sum_x: f.a * x.sum_x + f.b * x.len.into(),
                sum_y: f.c * x.sum_y + f.d * x.len.into(),
                len: x.len,
            }
        }
    }
    pub struct TwoSequenceRangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        segtree: LazySegtree<TwoSequenceRangeAffineRangeSum<T>>,
        len: usize,
    }
    impl<T> TwoSequenceRangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        /// `xs` と `ys` の初期シーケンスでセグメント木を構築します。
        pub fn new(xs: &[T], ys: &[T]) -> Self {
            assert_eq!(xs.len(), ys.len(), "xs and ys must have the same length");
            let xs_ys = xs
                .iter()
                .zip(ys.iter())
                .map(|(&x, &y)| TwoSequenceData::unit(x, y))
                .collect_vec();
            let len = xs_ys.len();
            Self {
                segtree: LazySegtree::from(xs_ys),
                len,
            }
        }
        /// 指定された区間 `range` に対して、`xs[i] ← a * xs[i] + b`, `ys[i] ← c * ys[i] + d`
        /// のアフィン変換を適用します。
        pub fn apply_range_affine(
            &mut self,
            range: impl RangeBounds<usize>,
            a: T,
            b: T,
            c: T,
            d: T,
        ) {
            self.segtree
                .apply_range(range, TwoSequenceAffine { a, b, c, d })
        }
        /// 指定された区間 `range` に対して、`xs[i] ← a * xs[i] + b` のアフィン変換を適用します。
        pub fn apply_range_affine_x(&mut self, range: impl RangeBounds<usize>, a: T, b: T) {
            self.apply_range_affine(range, a, b, 1.into(), 0.into())
        }
        /// 指定された区間 `range` に対して、`ys[i] ← c * ys[i] + d` のアフィン変換を適用します。
        pub fn apply_range_affine_y(&mut self, range: impl RangeBounds<usize>, c: T, d: T) {
            self.apply_range_affine(range, 1.into(), 0.into(), c, d)
        }
        /// 指定された区間 `range` の `sum(xs[i] * ys[i])` を計算して返します。
        pub fn query_sum_xy(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_xy
        }
        /// 指定された区間 `range` の `sum(xs[i])` を計算して返します。
        pub fn query_sum_x(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_x
        }
        /// 指定された区間 `range` の `sum(ys[i])` を計算して返します。
        pub fn query_sum_y(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_y
        }
        /// 指定されたインデックス `p` の `xs[p]` と `ys[p]` の値を更新します。
        pub fn set(&mut self, p: usize, x: T, y: T) {
            self.segtree.set(p, TwoSequenceData::unit(x, y));
        }
        /// 指定されたインデックス `p` の `xs[p]` の値を更新します。
        pub fn set_x(&mut self, p: usize, x: T) {
            let (_, y) = self.get(p);
            self.set(p, x, y);
        }
        /// 指定されたインデックス `p` の `ys[p]` の値を更新します。
        pub fn set_y(&mut self, p: usize, y: T) {
            let (x, _) = self.get(p);
            self.set(p, x, y);
        }
        /// 指定されたインデックス `p` の `xs[p]` と `ys[p]` の値を取得します。
        pub fn get(&mut self, p: usize) -> (T, T) {
            let data = self.segtree.get(p);
            (data.sum_x, data.sum_y)
        }
        /// セグメント木の現在の状態を `(Vec<T>, Vec<T>)` として返します。
        pub fn to_vec(&mut self) -> (Vec<T>, Vec<T>) {
            (0..self.len).map(|i| self.get(i)).unzip()
        }
    }
}
