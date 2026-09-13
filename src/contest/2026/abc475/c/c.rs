// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Usize1,
        len: i64,
        mut xs: [i64; n -1],
    }

    let ps = prefix_sum(&xs);

    let ans = (0..n)
        .tuple_combinations_with_replacement()
        .filter(|&(l, r)| {
            if !(l..=r).contains(&s) {
                return false;
            }

            // s → l → r
            let len1 = (ps[s] - ps[l]) + ps[r] - ps[l];

            // s → r → l
            let len2 = (ps[r] - ps[s]) + ps[r] - ps[l];

            len1.min(len2) <= len
        })
        .map(|(l, r)| r - l + 1)
        .max()
        .unwrap();

    println!("{}", ans);
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
use cumsum::*;
#[allow(clippy::module_inception)]
pub mod cumsum {
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }
    use std::ops::{Bound, Range, RangeBounds};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }
    impl CumSum {
        /// # 計算量
        /// O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1,
            };
            begin..end
        }
        /// 区間 `[begin, end)` の要素の和を計算します。
        /// # 計算量
        /// O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        /// 区間 `[0, end)` での和を計算します。
        /// # 計算量
        /// O(1)
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        /// 区間 `[begin, n)` の要素の和を計算します。（`n` は元の配列の長さ）
        /// # 計算量
        /// O(1)
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
        /// `f(sum(l..r))` が `true` となる最大の `r in [l, n]` を見つける。
        /// `n` は元の配列の長さ。
        /// `f` は単調でなければならない。
        /// `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        /// # Panics
        /// `l > n` の場合にパニックする。
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(l <= n);
            assert!(f(0), "f(0) must be true");
            if f(self.range_sum(l..n)) {
                return n;
            }
            let mut ok = l;
            let mut ng = n + 1;
            while ng - ok > 1 {
                let mid = ok + (ng - ok) / 2;
                if f(self.range_sum(l..mid)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
        /// `f(sum(l..r))` が `true` となる最小の `l in [0, r]` を見つける。
        /// `f` は単調でなければならない。
        /// `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        /// `r > n` の場合にパニックする。
        /// # 計算量
        /// O(log r)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(r <= n);
            assert!(f(0), "f(0) must be true");
            if f(self.range_sum(0..r)) {
                return 0;
            }
            let mut ok = r;
            let mut ng = 0;
            while ok - ng > 1 {
                let mid = ng + (ok - ng) / 2;
                if f(self.range_sum(mid..r)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
    }
}

use tuple_combinations_with_replacement::*;
#[allow(clippy::module_inception)]
pub mod tuple_combinations_with_replacement {
    use std::iter::{Fuse, FusedIterator};
    use std::marker::PhantomData;
    /// 重複を許して固定長タプルを列挙するイテレータである。
    #[derive(Clone, Debug)]
    #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
    pub struct TupleCombinationsWithReplacement<I, T>
    where
        I: Iterator,
        T: HasCombinationWithReplacement<I>,
    {
        iter: T::Combination,
        _marker: PhantomData<I>,
    }
    pub trait HasCombinationWithReplacement<I>: Sized {
        type Combination: From<I> + Iterator<Item = Self>;
    }
    pub trait IteratorTupleCombinationsWithReplacement: Iterator + Sized {
        /// 重複を許す組合せを、タプルとして列挙する。
        /// 入力中での位置が同じ要素を複数回選べる。出力順は入力順に対する辞書順である。
        fn tuple_combinations_with_replacement<T>(self) -> TupleCombinationsWithReplacement<Self, T>
        where
            Self: Clone,
            Self::Item: Clone,
            T: HasCombinationWithReplacement<Self>,
        {
            TupleCombinationsWithReplacement {
                iter: T::Combination::from(self),
                _marker: PhantomData,
            }
        }
    }
    impl<T: Iterator> IteratorTupleCombinationsWithReplacement for T {}
    impl<I, T> Iterator for TupleCombinationsWithReplacement<I, T>
    where
        I: Iterator,
        T: HasCombinationWithReplacement<I>,
    {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
        fn count(self) -> usize {
            self.iter.count()
        }
        fn fold<B, F>(self, init: B, f: F) -> B
        where
            F: FnMut(B, Self::Item) -> B,
        {
            self.iter.fold(init, f)
        }
    }
    impl<I, T> FusedIterator for TupleCombinationsWithReplacement<I, T>
    where
        I: FusedIterator,
        T: HasCombinationWithReplacement<I>,
    {
    }
    #[derive(Clone, Debug)]
    pub struct Tuple1CombinationWithReplacement<I> {
        iter: I,
    }
    impl<I> From<I> for Tuple1CombinationWithReplacement<I> {
        fn from(iter: I) -> Self {
            Self { iter }
        }
    }
    impl<I: Iterator> Iterator for Tuple1CombinationWithReplacement<I> {
        type Item = (I::Item,);
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|x| (x,))
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
        fn count(self) -> usize {
            self.iter.count()
        }
    }
    impl<I: Iterator> HasCombinationWithReplacement<I> for (I::Item,) {
        type Combination = Tuple1CombinationWithReplacement<I>;
    }
    macro_rules ! impl_tuple_combination_with_replacement {($ combination : ident $ previous : ident ; $ ($ item : ident ) * ) => {# [derive (Clone , Debug ) ] pub struct $ combination < I : Iterator > {item : Option < I :: Item >, iter : I , combination : $ previous < I >, } impl < I : Iterator + Clone > From < I > for $ combination < I > {fn from (mut iter : I ) -> Self {let combination = iter . clone () . into () ; let item = iter . next () ; Self {item , iter , combination , } } } impl < I : Iterator + Clone > From < I > for $ combination < Fuse < I >> {fn from (iter : I ) -> Self {Self :: from (iter . fuse () ) } } impl < I , A > Iterator for $ combination < I > where I : Iterator < Item = A > + Clone , A : Clone , {type Item = (A , $ (replace_ident ! ($ item , A ) ) ,* ) ; fn next (& mut self ) -> Option < Self :: Item > {if let Some (($ ($ item , ) * ) ) = self . combination . next () {let head = self . item . clone () . unwrap () ; Some ((head , $ ($ item ) ,* ) ) } else {let next_combination = self . iter . clone () . into () ; self . item = self . iter . next () ; self . item . clone () . and_then (| head | {self . combination = next_combination ; self . combination . next () . map (| ($ ($ item , ) * ) | (head , $ ($ item ) ,* ) ) } ) } } } impl < I , A > HasCombinationWithReplacement < I > for (A , $ (replace_ident ! ($ item , A ) ) ,* ) where I : Iterator < Item = A > + Clone , I :: Item : Clone , {type Combination = $ combination < Fuse < I >>; } } ; }
    macro_rules! replace_ident {
        ($ _ident : ident , $ replacement : ty ) => {
            $replacement
        };
    }
    impl_tuple_combination_with_replacement ! (Tuple2CombinationWithReplacement Tuple1CombinationWithReplacement ; a );
    impl_tuple_combination_with_replacement ! (Tuple3CombinationWithReplacement Tuple2CombinationWithReplacement ; a b );
    impl_tuple_combination_with_replacement ! (Tuple4CombinationWithReplacement Tuple3CombinationWithReplacement ; a b c );
    impl_tuple_combination_with_replacement ! (Tuple5CombinationWithReplacement Tuple4CombinationWithReplacement ; a b c d );
    impl_tuple_combination_with_replacement ! (Tuple6CombinationWithReplacement Tuple5CombinationWithReplacement ; a b c d e );
    impl_tuple_combination_with_replacement ! (Tuple7CombinationWithReplacement Tuple6CombinationWithReplacement ; a b c d e f );
    impl_tuple_combination_with_replacement ! (Tuple8CombinationWithReplacement Tuple7CombinationWithReplacement ; a b c d e f g );
    impl_tuple_combination_with_replacement ! (Tuple9CombinationWithReplacement Tuple8CombinationWithReplacement ; a b c d e f g h );
    impl_tuple_combination_with_replacement ! (Tuple10CombinationWithReplacement Tuple9CombinationWithReplacement ; a b c d e f g h i );
    impl_tuple_combination_with_replacement ! (Tuple11CombinationWithReplacement Tuple10CombinationWithReplacement ; a b c d e f g h i j );
    impl_tuple_combination_with_replacement ! (Tuple12CombinationWithReplacement Tuple11CombinationWithReplacement ; a b c d e f g h i j k );
}
