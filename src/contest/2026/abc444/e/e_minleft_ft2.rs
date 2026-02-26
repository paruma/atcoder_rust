// r を 1ずつうごかす (min_left)
// 多重集合を FenwickTree で管理 (FenwickTreeSparseMultiset)

fn main() {
    input! {
        n: usize,
        d: i64,
        xs: [i64; n],
    }

    let mut bag = FenwickTreeSparseMultiset::new(&xs);

    let mut l = 0_usize;
    let mut sum = 0;
    for r in 0..n {
        // l を伸ばす (l<=r まで)
        // xs[l..=r] の要素のうち、開区間 (xs[r] - d, xs[r] + d) に含まれている個数 (0であってほしい)
        loop {
            if bag.contains_in_range(xs[r] - d + 1..xs[r] + d) {
                bag.remove1(xs[l]);
                l += 1;
            } else {
                break;
            }
        }
        let sub = r - l + 1;
        sum += sub;

        bag.insert(xs[r]);
    }

    println!("{}", sum);
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
use fenwick_tree_sparse_multiset::*;
pub mod fenwick_tree_sparse_multiset {
    use std::ops::RangeBounds;
    /// Fenwick Tree を基盤とした座標圧縮付きマルチセット。
    /// BTreeMultiSet と違って、`entries`` として指定した値以外を挿入することはできませんが、そのかわりk番目の値が k に依らず $O(\log N)$ で取得できます。
    #[derive(Clone, Debug)]
    pub struct FenwickTreeSparseMultiset {
        ft: InternalFenwickTree,
        cc: CoordinateCompression,
        length: usize,
        set_length: usize,
    }
    impl FenwickTreeSparseMultiset {
        /// 構築時に指定された `entries` に基づいて座標圧縮空間を構築し、空のマルチセットを作成します。
        /// # 計算量
        /// $O(N \log N)$ ($N$ は `entries.len()`)
        pub fn new(entries: &[i64]) -> Self {
            let cc = CoordinateCompression::new(entries);
            let size = cc.space_size();
            Self {
                ft: InternalFenwickTree::new(size),
                cc,
                length: 0,
                set_length: 0,
            }
        }
        /// 指定した値を追加します。
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn insert(&mut self, value: i64) {
            self.insert_many(value, 1);
        }
        /// 指定した値を `count` 個追加します。
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn insert_many(&mut self, value: i64, count: usize) {
            if count == 0 {
                return;
            }
            let idx = self.cc.compress(value);
            if self.ft.get(idx) == 0 {
                self.set_length += 1;
            }
            self.ft.add(idx, count as i64);
            self.length += count;
        }
        /// 要素を1つ削除します。
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返します。
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn remove1(&mut self, value: i64) -> bool {
            let idx = self.cc.compress(value);
            if self.ft.get(idx) > 0 {
                self.ft.add(idx, -1);
                self.length -= 1;
                if self.ft.get(idx) == 0 {
                    self.set_length -= 1;
                }
                true
            } else {
                false
            }
        }
        /// 要素を最大 `count` 個削除します。
        /// 実際に削除した個数を返します。
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn remove_up_to(&mut self, value: i64, count: usize) -> usize {
            let idx = self.cc.compress(value);
            let current = self.ft.get(idx) as usize;
            let removed = current.min(count);
            if removed > 0 {
                self.ft.add(idx, -(removed as i64));
                self.length -= removed;
                if current == removed {
                    self.set_length -= 1;
                }
            }
            removed
        }
        /// 指定した要素をすべて削除します。
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返します。
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn remove_all(&mut self, value: i64) -> bool {
            let idx = self.cc.compress(value);
            let current = self.ft.get(idx) as usize;
            if current > 0 {
                self.ft.add(idx, -(current as i64));
                self.length -= current;
                self.set_length -= 1;
                true
            } else {
                false
            }
        }
        /// 最小の要素を1つ取り出して削除します。
        /// 空の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn pop_min(&mut self) -> Option<i64> {
            let val = self.min()?;
            self.remove1(val);
            Some(val)
        }
        /// 最大の要素を1つ取り出して削除します。
        /// 空の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn pop_max(&mut self) -> Option<i64> {
            let val = self.max()?;
            self.remove1(val);
            Some(val)
        }
        /// マルチセットの全要素を削除し、空にします。
        /// # 計算量
        /// $O(N)$ ($N$ は一意な要素数)
        pub fn clear(&mut self) {
            self.ft = InternalFenwickTree::new(self.cc.space_size());
            self.length = 0;
            self.set_length = 0;
        }
        /// マルチセットに含まれる全要素数（重複を含む）を返します。
        /// # 計算量
        /// $O(1)$
        pub fn len(&self) -> usize {
            self.length
        }
        /// マルチセットに含まれるユニークな要素の種類数を返します。
        /// # 計算量
        /// $O(1)$
        pub fn set_len(&self) -> usize {
            self.set_length
        }
        /// マルチセットが空かどうかを返します。
        /// # 計算量
        /// $O(1)$
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
        /// 指定した要素の個数を返します。
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        /// # 計算量
        /// $O(1)$
        pub fn count(&self, value: i64) -> usize {
            let idx = self.cc.compress(value);
            self.ft.get(idx) as usize
        }
        /// 指定した要素が含まれているかを返します。
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        /// # 計算量
        /// $O(1)$
        pub fn contains(&self, value: i64) -> bool {
            self.count(value) > 0
        }
        /// 重複を考慮して、$n$ 番目に小さい要素を返します（0-indexed）。
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn nth_min(&self, n: usize) -> Option<i64> {
            let idx = self.ft.max_right(0, |&s| s <= n as i64);
            if idx < self.ft.len() {
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }
        /// 重複を考慮して、$n$ 番目に大きい要素を返します（0-indexed）。
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn nth_max(&self, n: usize) -> Option<i64> {
            let length = self.length;
            if n < length {
                let target_prefix_sum = (length - 1 - n) as i64;
                let idx = self.ft.max_right(0, |&s| s <= target_prefix_sum);
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }
        /// 最小の要素を返します。
        /// 空の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn min(&self) -> Option<i64> {
            let idx = self.ft.max_right(0, |&s| s == 0);
            if idx < self.ft.len() {
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }
        /// 最大の要素を返します。
        /// 空の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn max(&self) -> Option<i64> {
            let idx = self.ft.min_left(self.ft.len(), |&s| s == 0);
            if idx > 0 {
                Some(self.cc.decompress(idx - 1))
            } else {
                None
            }
        }
        /// 指定された範囲内での最小の要素を返します。
        /// 範囲内に要素がない場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn min_in_range<R: RangeBounds<i64>>(&self, range: R) -> Option<i64> {
            let r_idx = self.cc.compress_range(range);
            let idx = self.ft.max_right(r_idx.start, |&s| s == 0);
            if idx < r_idx.end {
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }
        /// 指定された範囲内での最大の要素を返します。
        /// 範囲内に要素がない場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn max_in_range<R: RangeBounds<i64>>(&self, range: R) -> Option<i64> {
            let r_idx = self.cc.compress_range(range);
            let idx = self.ft.min_left(r_idx.end, |&s| s == 0);
            if idx > r_idx.start {
                Some(self.cc.decompress(idx - 1))
            } else {
                None
            }
        }
        /// 指定された範囲内で、重複を考慮して $n$ 番目に小さい要素を返す（0-indexed）。
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn nth_min_in_range<R: RangeBounds<i64>>(&self, n: usize, range: R) -> Option<i64> {
            let r_idx = self.cc.compress_range(range);
            let idx = self.ft.max_right(r_idx.start, |&s| s <= n as i64);
            if idx < r_idx.end {
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }
        /// 指定された範囲内で、重複を考慮して $n$ 番目に大きい要素を返す（0-indexed）。
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn nth_max_in_range<R: RangeBounds<i64>>(&self, n: usize, range: R) -> Option<i64> {
            let r_idx = self.cc.compress_range(range);
            let range_count = self.ft.range_sum(r_idx.start..r_idx.end) as usize;
            if n < range_count {
                let target_prefix_sum = (range_count - 1 - n) as i64;
                let idx = self.ft.max_right(r_idx.start, |&s| s <= target_prefix_sum);
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }
        /// 指定した範囲内に要素が含まれているかを返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn contains_in_range<R: RangeBounds<i64>>(&self, range: R) -> bool {
            let r_idx = self.cc.compress_range(range);
            if r_idx.start >= r_idx.end {
                return false;
            }
            self.ft.range_sum(r_idx.start..r_idx.end) > 0
        }
    }
    /// 座標圧縮構造体。
    #[derive(Debug, Clone)]
    struct CoordinateCompression {
        space: Vec<i64>,
    }
    impl CoordinateCompression {
        fn new(space: &[i64]) -> Self {
            let mut space = space.to_vec();
            space.sort_unstable();
            space.dedup();
            Self { space }
        }
        fn compress(&self, x: i64) -> usize {
            self.space
                .binary_search(&x)
                .expect("Value not in coordinate compression space")
        }
        fn decompress(&self, i: usize) -> i64 {
            self.space[i]
        }
        fn space_size(&self) -> usize {
            self.space.len()
        }
        fn compress_range(&self, range: impl RangeBounds<i64>) -> std::ops::Range<usize> {
            use std::ops::Bound::*;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => self.space.binary_search(&x).unwrap_or_else(|e| e),
                Excluded(&x) => match self.space.binary_search(&x) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                },
            };
            let end = match range.end_bound() {
                Unbounded => self.space.len(),
                Included(&x) => match self.space.binary_search(&x) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                },
                Excluded(&x) => self.space.binary_search(&x).unwrap_or_else(|e| e),
            };
            begin..end
        }
    }
    /// Fenwick Tree の基本操作を提供する補助構造体。
    /// `range_sum_fenwick_tree.rs` に準拠した実装。
    #[derive(Clone, Debug)]
    struct InternalFenwickTree {
        n: usize,
        ary: Vec<i64>,
        vals: Vec<i64>,
    }
    impl InternalFenwickTree {
        fn new(n: usize) -> Self {
            Self {
                n,
                ary: vec![0; n],
                vals: vec![0; n],
            }
        }
        fn prefix_sum(&self, mut idx: usize) -> i64 {
            let mut sum = 0;
            while idx > 0 {
                sum += self.ary[idx - 1];
                idx &= idx - 1;
            }
            sum
        }
        fn add(&mut self, mut idx: usize, val: i64) {
            assert!(idx < self.n);
            self.vals[idx] += val;
            idx += 1;
            while idx <= self.n {
                self.ary[idx - 1] += val;
                idx += idx & idx.wrapping_neg();
            }
        }
        fn range_sum(&self, range: std::ops::Range<usize>) -> i64 {
            let l = range.start;
            let r = range.end;
            assert!(l <= r && r <= self.n);
            self.prefix_sum(r) - self.prefix_sum(l)
        }
        fn get(&self, idx: usize) -> i64 {
            assert!(idx < self.n);
            self.vals[idx]
        }
        fn max_right<F: FnMut(&i64) -> bool>(&self, l: usize, mut f: F) -> usize {
            assert!(l <= self.n);
            assert!(f(&0));
            let val_l = self.prefix_sum(l);
            let mut r = 0;
            let mut current_val = 0;
            let mut k = if self.n == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - self.n.leading_zeros())
            };
            while k > 0 {
                if r + k <= self.n {
                    let next_val = current_val + self.ary[r + k - 1];
                    if r + k <= l || f(&(next_val - val_l)) {
                        r += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            r
        }
        fn min_left<F: FnMut(&i64) -> bool>(&self, r: usize, mut f: F) -> usize {
            assert!(r <= self.n);
            assert!(f(&0));
            let val_r = self.prefix_sum(r);
            if f(&val_r) {
                return 0;
            }
            let mut idx = 0;
            let mut current_val = 0;
            let mut k = if self.n == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - self.n.leading_zeros())
            };
            while k > 0 {
                if idx + k <= r {
                    let next_val = current_val + self.ary[idx + k - 1];
                    if !f(&(val_r - next_val)) {
                        idx += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            idx + 1
        }
        fn len(&self) -> usize {
            self.n
        }
    }
}
