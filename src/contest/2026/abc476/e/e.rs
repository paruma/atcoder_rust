// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [Usize1; n],
        lrs: [(Usize1, Usize1); m],
    }

    let mut inv_xs = inv_of_permutation(&xs);

    let mut xs_min = RangeMinSegtree::from_slice(&xs);
    let mut xs_max = RangeMaxSegtree::from_slice(&xs);

    for (l, r) in lrs {
        let min = xs_min.range_min(l..=r);
        let max = xs_max.range_max(l..=r);

        let min_idx = inv_xs[min];
        let max_idx = inv_xs[max];

        xs_min.set(min_idx, max);
        xs_min.set(max_idx, min);

        xs_max.set(min_idx, max);
        xs_max.set(max_idx, min);

        inv_xs.swap(min, max);
    }

    let ans = xs_min.to_vec().iter().copied().map(|x| x + 1).collect_vec();
    print_vec_1line(&ans);
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
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
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
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
use symmetric_group::*;
#[allow(clippy::module_inception)]
pub mod symmetric_group {
    /// 置換を巡回置換の積で表したときの巡回置換のリストを返す。
    /// 例: `make_cycles(&[1, 0, 3, 2]) == vec![vec![0, 1], vec![2, 3]]`
    /// # 計算量
    /// O(N)
    pub fn make_cycles(ps: &[usize]) -> Vec<Vec<usize>> {
        let n = ps.len();
        let mut visited = vec![false; n];
        let mut cycles = vec![];
        for init in 0..n {
            if visited[init] {
                continue;
            }
            let mut cycle = vec![];
            let mut cur = init;
            while !visited[cur] {
                cycle.push(cur);
                visited[cur] = true;
                cur = ps[cur];
            }
            cycles.push(cycle);
        }
        cycles
    }
    /// 置換の積を計算する
    /// # 計算量
    /// O(N)
    pub fn mul_of_permutation(ps: &[usize], mut qs: Vec<usize>) -> Vec<usize> {
        for q in qs.iter_mut() {
            *q = ps[*q];
        }
        qs
    }
    /// 置換の `k` 乗を計算する
    /// # 計算量
    /// O(N)
    pub fn pow_of_permutation(ps: &[usize], k: u64) -> Vec<usize> {
        let n = ps.len();
        let mut ret = vec![0; n];
        let cycles = make_cycles(ps);
        for cycle in cycles {
            let len = cycle.len() as u64;
            let k = (k % len) as usize;
            for (i, &x) in cycle.iter().enumerate() {
                ret[x] = cycle[(i + k) % cycle.len()];
            }
        }
        ret
    }
    /// 逆置換を計算する
    /// # 計算量
    /// O(N)
    pub fn inv_of_permutation(ps: &[usize]) -> Vec<usize> {
        let n = ps.len();
        let mut ret = vec![0; n];
        for (i, &p) in ps.iter().enumerate() {
            ret[p] = i;
        }
        ret
    }
    /// 転倒数を計算する
    /// # 計算量
    /// O(N log N)
    pub fn inversion_number(ps: &[usize]) -> i64 {
        use ac_library::FenwickTree;
        let n = ps.len();
        let mut ft = FenwickTree::new(n, 0_i64);
        let mut ans = 0;
        for &p in ps {
            ans += ft.sum(p + 1..n);
            ft.add(p, 1);
        }
        ans
    }
}

use min_max_monoid::*;
use range_min_segtree::*;
#[allow(clippy::module_inception)]
pub mod min_max_monoid {
    use ac_library::Monoid;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    pub trait BoundedBelow {
        fn min_value() -> Self;
    }
    pub trait BoundedAbove {
        fn max_value() -> Self;
    }
    macro_rules ! impl_bounded {($ ($ ty : ty ) ,* ) => {$ (impl BoundedBelow for $ ty {# [inline ] fn min_value () -> Self {Self :: MIN } } impl BoundedAbove for $ ty {# [inline ] fn max_value () -> Self {Self :: MAX } } ) * } ; }
    impl_bounded!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
    impl<T: BoundedAbove> BoundedBelow for std::cmp::Reverse<T> {
        #[inline]
        fn min_value() -> Self {
            std::cmp::Reverse(T::max_value())
        }
    }
    impl<T: BoundedBelow> BoundedAbove for std::cmp::Reverse<T> {
        #[inline]
        fn max_value() -> Self {
            std::cmp::Reverse(T::min_value())
        }
    }
    macro_rules ! impl_bounded_tuples {($ head : ident ) => {} ; ($ head : ident , $ ($ tail : ident ) ,* ) => {impl <$ head , $ ($ tail ) ,*> BoundedBelow for ($ head , $ ($ tail ) ,* ) where $ head : BoundedBelow , $ ($ tail : BoundedBelow ) ,* {# [inline ] fn min_value () -> Self {($ head :: min_value () , $ ($ tail :: min_value () ) ,* ) } } impl <$ head , $ ($ tail ) ,*> BoundedAbove for ($ head , $ ($ tail ) ,* ) where $ head : BoundedAbove , $ ($ tail : BoundedAbove ) ,* {# [inline ] fn max_value () -> Self {($ head :: max_value () , $ ($ tail :: max_value () ) ,* ) } } impl_bounded_tuples ! ($ ($ tail ) ,* ) ; } ; () => {} ; }
    impl_bounded_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    /// 辞書式順序で最小の要素を管理するモノイド (単位元は最大値)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MinMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for MinMonoid<T>
    where
        T: BoundedAbove + Ord + Clone,
    {
        type S = T;
        fn identity() -> Self::S {
            T::max_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::min(a, b).clone()
        }
    }
    /// 辞書式順序で最大の要素を管理するモノイド (単位元は最小値)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MaxMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for MaxMonoid<T>
    where
        T: BoundedBelow + Ord + Clone,
    {
        type S = T;
        fn identity() -> Self::S {
            T::min_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::max(a, b).clone()
        }
    }
}
#[allow(clippy::module_inception)]
pub mod range_min_segtree {
    use super::MinMonoid;
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::ops::RangeBounds;
    /// ACL の Segtree を使用した区間最小セグメント木。
    /// 数値型 T に対して点更新・区間最小取得を行う。
    #[derive(Clone)]
    pub struct RangeMinSegtree<T>
    where
        MinMonoid<T>: Monoid<S = T>,
        T: Clone,
    {
        segtree: Segtree<MinMonoid<T>>,
        len: usize,
    }
    impl<T> RangeMinSegtree<T>
    where
        MinMonoid<T>: Monoid<S = T>,
        T: Copy + Ord,
    {
        /// 単位元で初期化されたセグメント木を構築する
        pub fn new(n: usize) -> Self {
            Self {
                segtree: Segtree::<MinMonoid<T>>::new(n),
                len: n,
            }
        }
        /// 配列からセグメント木を構築する
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: Segtree::<MinMonoid<T>>::from(xs.to_vec()),
                len,
            }
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        /// p 番目の要素を x に更新する
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }
        /// p 番目の要素を取得する
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p)
        }
        /// range の最小値を取得する
        pub fn range_min<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }
        /// 全要素の最小値を取得する
        pub fn all_min(&self) -> T {
            self.segtree.all_prod()
        }
        /// セグメント木上の二分探索。
        /// [l, r) の最小値 s について f(&s) が true となる最大の r を返す。
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.max_right(l, f)
        }
        /// セグメント木上の二分探索。
        /// [l, r) の最小値 s について f(&s) が true となる最小の l を返す。
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.min_left(r, f)
        }
        /// p 番目の要素を min(current, x) に更新する
        pub fn chmin(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, std::cmp::min(current, x));
        }
        /// 現在の状態を Vec として返す
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
use min_max_monoid::*;
use range_max_segtree::*;
#[allow(clippy::module_inception)]
pub mod range_max_segtree {
    use super::MaxMonoid;
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::ops::RangeBounds;
    /// ACL の Segtree を使用した区間最大セグメント木。
    /// 数値型 T に対して点更新・区間最大取得を行う。
    #[derive(Clone)]
    pub struct RangeMaxSegtree<T>
    where
        MaxMonoid<T>: Monoid<S = T>,
        T: Clone,
    {
        segtree: Segtree<MaxMonoid<T>>,
        len: usize,
    }
    impl<T> RangeMaxSegtree<T>
    where
        MaxMonoid<T>: Monoid<S = T>,
        T: Copy + Ord,
    {
        /// 単位元で初期化されたセグメント木を構築する
        pub fn new(n: usize) -> Self {
            Self {
                segtree: Segtree::<MaxMonoid<T>>::new(n),
                len: n,
            }
        }
        /// 配列からセグメント木を構築する
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: Segtree::<MaxMonoid<T>>::from(xs.to_vec()),
                len,
            }
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        /// p 番目の要素を x に更新する
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }
        /// p 番目の要素を取得する
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p)
        }
        /// range の最大値を取得する
        pub fn range_max<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }
        /// 全要素の最大値を取得する
        pub fn all_max(&self) -> T {
            self.segtree.all_prod()
        }
        /// セグメント木上の二分探索。
        /// [l, r) の最大値 s について f(&s) が true となる最大の r を返す。
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.max_right(l, f)
        }
        /// セグメント木上の二分探索。
        /// [l, r) の最大値 s について f(&s) が true となる最小の l を返す。
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.min_left(r, f)
        }
        /// p 番目の要素を max(current, x) に更新する
        pub fn chmax(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, std::cmp::max(current, x));
        }
        /// 現在の状態を Vec として返す
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
