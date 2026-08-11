// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [usize; n],
    }

    let max = xs.iter().copied().max().unwrap();
    let mut poss = vec![vec![]; max + 1];

    for (i, x) in xs.iter().copied().enumerate() {
        poss[x].push(i);
    }

    let range_set = RangeSet::new();

    let mut cnts = vec![0; max + 2];
    cnts[0] = n * (n + 1) / 2;

    for mex in 1..=max + 1 {
        // 0 から mex - 1 まではある
        // 

        //
    }

    let ans = (1..=max + 1).map(|mex| cnts[mex]).sum::<i64>();
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
use range_set::*;
#[allow(clippy::module_inception)]
pub mod range_set {
    use std::collections::BTreeMap;
    /// 整数の集合を隣り合わない半開区間の直和で管理するデータ構造。
    /// # 機能
    /// - 区間内の整数の追加 (`insert_range`)
    /// - 区間内の整数の削除 (`remove_range`)
    /// - 点が区間集合に含まれるかの判定 (`contains`)
    /// - 区間が完全にカバーされているかの判定 (`covers`)
    /// - 全区間の長さの合計 (`len`)
    /// - x 以上で集合に含まれない最小値 (`min_exclusive_geq`、いわゆる mex)
    /// - x 以下で集合に含まれない最大値 (`max_exclusive_leq`)
    /// - など
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct RangeSet {
        map: BTreeMap<i64, i64>,
        total_length: i64,
    }
    impl Default for RangeSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl RangeSet {
        /// 空の `RangeSet` を作成する。
        pub fn new() -> Self {
            Self {
                map: BTreeMap::new(),
                total_length: 0,
            }
        }
        /// 区間 `[l, r)` の各値を集合に追加する。
        /// # 計算量
        /// amortized O(log N)
        pub fn insert_range(&mut self, l: i64, r: i64) {
            assert!(l <= r);
            if r == l {
                return;
            }
            let mut start = l;
            let mut end = r;
            let mut to_remove = Vec::new();
            let mut removed_len = 0;
            for (&l_i, &r_i) in self.map.range(..=r).rev().take_while(|&(_, r_i)| l <= *r_i) {
                start = start.min(l_i);
                end = end.max(r_i);
                to_remove.push(l_i);
                removed_len += r_i - l_i;
            }
            for l_i in to_remove {
                self.map.remove(&l_i);
            }
            let added_len = end - start;
            self.total_length += added_len - removed_len;
            self.map.insert(start, end);
        }
        /// 区間 `[l, r)` の各値を集合から削除する。
        /// # 計算量
        /// amortized O(log N)
        pub fn remove_range(&mut self, l: i64, r: i64) {
            assert!(l <= r);
            if r == l {
                return;
            }
            let mut to_add = Vec::new();
            let mut to_remove = Vec::new();
            let mut len_change = 0;
            for (&l_i, &r_i) in self.map.range(..r).rev().take_while(|&(_, r_i)| l < *r_i) {
                to_remove.push(l_i);
                len_change -= r_i - l_i;
                if l_i < l {
                    to_add.push((l_i, l));
                    len_change += l - l_i;
                }
                if r < r_i {
                    to_add.push((r, r_i));
                    len_change += r_i - r;
                }
            }
            for l_i in to_remove {
                self.map.remove(&l_i);
            }
            for (l_add, r_add) in to_add {
                self.map.insert(l_add, r_add);
            }
            self.total_length += len_change;
        }
        /// 集合に `x` を追加する。
        /// # 計算量
        /// amortized O(log N)
        pub fn insert(&mut self, x: i64) {
            self.insert_range(x, x + 1);
        }
        /// 集合から `x` を削除する。
        /// # 計算量
        /// amortized O(log N)
        pub fn remove(&mut self, x: i64) {
            self.remove_range(x, x + 1);
        }
        /// 集合が `x` を含んでいるかを返す。
        /// # 計算量
        /// O(log N)
        pub fn contains(&self, x: i64) -> bool {
            self.find_range(x).is_some()
        }
        /// 集合が区間 `[l, r)` を含んでいるかを返す。
        /// # 計算量
        /// O(log N)
        pub fn covers(&self, l: i64, r: i64) -> bool {
            assert!(l <= r);
            if r == l {
                return true;
            }
            if let Some((_start, end)) = self.find_range(l) {
                r <= end
            } else {
                false
            }
        }
        /// 集合が空かどうかを返す。
        /// # 計算量
        /// O(1)
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
        /// 集合の要素数を返す。
        /// # 計算量
        /// O(1)
        pub fn len(&self) -> i64 {
            self.total_length
        }
        /// 集合に含まれる最小値を返す。
        /// 集合が空の場合は `None` を返す。
        /// # 計算量
        /// O(1)
        pub fn min(&self) -> Option<i64> {
            self.map.keys().next().copied()
        }
        /// 集合に含まれる最大値を返す。
        /// 集合が空の場合は `None` を返す。
        /// # 計算量
        /// O(1)
        pub fn max(&self) -> Option<i64> {
            self.map.iter().next_back().map(|(_, &r)| r - 1)
        }
        /// 指定した区間 `[l, r)` と集合が共通部分を持たない（重ならない）かを返す。
        /// # 計算量
        /// O(log N)
        pub fn is_disjoint(&self, l: i64, r: i64) -> bool {
            if l >= r {
                return true;
            }
            if let Some((_, &r_i)) = self.map.range(..r).next_back() {
                r_i <= l
            } else {
                true
            }
        }
        /// 集合全体が指定した区間 `[l, r)` に完全に含まれているかを返す。
        /// # 計算量
        /// O(1)
        pub fn is_covered_by(&self, l: i64, r: i64) -> bool {
            if self.is_empty() {
                return true;
            }
            if l >= r {
                return false;
            }
            if let Some(min_val) = self.min() {
                if min_val < l {
                    return false;
                }
            }
            if let Some(max_val) = self.max() {
                if max_val >= r {
                    return false;
                }
            }
            true
        }
        /// x 以上で self に入っていない値の最小値を返す (いわゆる mex)
        /// # 計算量
        /// O(log N)
        pub fn min_exclusive_geq(&self, x: i64) -> i64 {
            if let Some((_, r)) = self.find_range(x) {
                r
            } else {
                x
            }
        }
        /// x 以下で self に入っていない値の最大値を返す
        /// # 計算量
        /// O(log N)
        pub fn max_exclusive_leq(&self, x: i64) -> i64 {
            if let Some((l, _)) = self.find_range(x) {
                l - 1
            } else {
                x
            }
        }
        /// x 以上で集合に含まれる最小の値を返す。
        /// 集合に含まれる値が存在しない場合は None を返す。
        /// # 計算量
        /// O(log N)
        pub fn min_inclusive_geq(&self, x: i64) -> Option<i64> {
            if self.contains(x) {
                return Some(x);
            }
            self.map.range(x..).next().map(|(&l, _)| l)
        }
        /// x 以下で集合に含まれる最大の値を返す。
        /// 集合に含まれる値が存在しない場合は None を返す。
        /// # 計算量
        /// O(log N)
        pub fn max_inclusive_leq(&self, x: i64) -> Option<i64> {
            if self.contains(x) {
                return Some(x);
            }
            self.map.range(..x).last().map(|(_, &r)| r - 1)
        }
        /// `x` が含まれる区間 `[l, r)` を検索し、`Some((l, r))` で返す。
        /// `x` を含む区間が見つからない場合は `None` を返す。
        fn find_range(&self, x: i64) -> Option<(i64, i64)> {
            if let Some((&l, &r)) = self.map.range(..=x).last() {
                if x < r { Some((l, r)) } else { None }
            } else {
                None
            }
        }
        /// 管理しているすべての区間 `[l, r)` のイテレータを返す。
        #[cfg(test)]
        pub(crate) fn ranges(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
            self.map.iter().map(|(&l, &r)| (l, r))
        }
    }
}
