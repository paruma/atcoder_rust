// 区間をセットで管理するテク
#[fastout]
fn main() {
    input! {
        n: i64,
        q: usize,
        lrs: [(i64, i64); q],
    }

    let mut set = RangeSet::new();
    set.insert_range(1, n + 1);

    for &(l, r) in &lrs {
        set.remove_range(l, r + 1);
        let ans = set.len();
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
    #[derive(Debug, Clone, PartialEq, Eq)]
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
        /// 空の `RangeSet2` を作成する。
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
