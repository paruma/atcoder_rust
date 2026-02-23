// #[fastout]
fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            a_s: [usize; n],
            b_s: [usize; n],
        }

        // 仕入れた日付を入れておく
        let mut eggs: BTreeMultiSet<usize> = BTreeMultiSet::new();

        for i in 0..n {
            for _ in 0..a_s[i] {
                eggs.insert(i);
            }

            for _ in 0..b_s[i] {
                let min = *eggs.min().unwrap();
                eggs.remove1(&min);
            }

            if i >= d {
                eggs.remove_all(&(i - d));
            }
        }

        let ans = eggs.len();
        println!("{}", ans);
    }
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
use btree_multiset::*;
#[allow(clippy::module_inception)]
pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{BTreeMap, btree_map::Range},
        ops::RangeBounds,
    };
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BTreeMultiSet<T> {
        map: BTreeMap<T, usize>,
        length: usize,
    }
    impl<T> Default for BTreeMultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T> BTreeMultiSet<T> {
        pub const fn new() -> BTreeMultiSet<T> {
            BTreeMultiSet {
                map: BTreeMap::new(),
                length: 0,
            }
        }
        pub fn range<R>(&self, range: R) -> Range<'_, T, usize>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.map.range(range)
        }
        /// 内部の BTreeMap のイテレータを返す。
        /// 要素とその個数のペア `(&T, &usize)` を巡回する。
        pub fn iter(&self) -> std::collections::btree_map::Iter<'_, T, usize> {
            self.map.iter()
        }
        /// 最小の要素を返す。
        /// 空の場合は `None` を返す。計算量は $O(\log K)$ ($K$ は要素の種類数)。
        pub fn min(&self) -> Option<&T>
        where
            T: Ord,
        {
            self.map.first_key_value().map(|(k, _)| k)
        }
        /// 最大の要素を返す。
        /// 空の場合は `None` を返す。計算量は $O(\log K)$ ($K$ は要素の種類数)。
        pub fn max(&self) -> Option<&T>
        where
            T: Ord,
        {
            self.map.last_key_value().map(|(k, _)| k)
        }
        /// 重複を考慮して、$n$ 番目に小さい要素を返す（0-indexed）。
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_min(&self, n: usize) -> Option<&T>
        where
            T: Ord,
        {
            let mut sum = 0;
            for (val, &cnt) in self.iter() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }
        /// 重複を考慮して、$n$ 番目に大きい要素を返す（0-indexed）。
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_max(&self, n: usize) -> Option<&T>
        where
            T: Ord,
        {
            let mut sum = 0;
            for (val, &cnt) in self.iter().rev() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }
        /// 指定した範囲内での最小の要素を返す。
        /// 範囲内に要素がない場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn min_in_range<R>(&self, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.range(range).next().map(|(k, _)| k)
        }
        /// 指定した範囲内での最大の要素を返す。
        /// 範囲内に要素がない場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn max_in_range<R>(&self, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.range(range).next_back().map(|(k, _)| k)
        }
        /// 指定した範囲内で、重複を考慮して $n$ 番目に小さい要素を返す（0-indexed）。
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は範囲内で走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_min_in_range<R>(&self, n: usize, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            let mut sum = 0;
            for (val, &cnt) in self.range(range) {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }
        /// 指定した範囲内で、重複を考慮して $n$ 番目に大きい要素を返す（0-indexed）。
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は範囲内で走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_max_in_range<R>(&self, n: usize, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            let mut sum = 0;
            for (val, &cnt) in self.range(range).rev() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }
        pub fn insert(&mut self, value: T)
        where
            T: Ord,
        {
            *self.map.entry(value).or_insert(0) += 1;
            self.length += 1;
        }
        pub fn remove1<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if let Some(cnt) = self.map.get_mut(value) {
                *cnt -= 1;
                if *cnt == 0 {
                    self.map.remove(value);
                }
                self.length -= 1;
                return true;
            }
            false
        }
        pub fn remove_all<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if let Some(cnt) = self.map.get(value) {
                self.length -= cnt;
                self.map.remove(value);
                return true;
            }
            false
        }
        pub fn len(&self) -> usize {
            self.length
        }
        pub fn set_len(&self) -> usize {
            self.map.len()
        }
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
        pub fn count<Q>(&self, value: &Q) -> usize
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            self.map.get(value).copied().unwrap_or(0)
        }
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            self.map.contains_key(value)
        }
    }
    impl<T: Ord> FromIterator<T> for BTreeMultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> BTreeMultiSet<T> {
            let mut set = BTreeMultiSet::new();
            for x in iter {
                set.insert(x);
            }
            set
        }
    }
}
