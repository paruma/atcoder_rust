// 問題文と制約は読みましたか？
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        t: Chars,
        ss: [Chars; n],
        nq : usize,
        qs: [(Usize1, Usize1); nq],
    }

    // 正解かどうかに変換

    let mut xss = ss
        .iter()
        .map(|s| (0..k).map(|i| t[i] == s[i]).collect_vec())
        .collect_vec();
    let space = {
        let mut xss = xss.clone();
        let mut space = xss.clone();

        for &(i, j) in &qs {
            xss[i][j] = !xss[i][j];
            space.push(xss[i].clone());
        }

        space
    };
    let cc = CoordinateCompression::new(&space);

    let mut bag = FenwickTreeDenseMultiset::new(cc.space_size());

    for xs in &xss {
        bag.insert(cc.compress(xs));
    }

    for &(i, j) in &qs {
        // dbg!(&bag);
        bag.remove1(cc.compress(&xss[i]));
        xss[i][j] = !xss[i][j];
        bag.insert(cc.compress(&xss[i]));

        let cnt = bag.count_in_range(cc.compress(&xss[i])..);
        // dbg!(cnt);
        // 全問不正解は誰も通過できない
        if cnt <= m && xss[i].iter().copied().any(|p| p) {
            println!("Yes");
        } else {
            println!("No");
        }
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
use coordinate_compression::*;
#[allow(clippy::module_inception)]
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;
    #[derive(Debug, Clone)]
    pub struct CoordinateCompression<T> {
        pub space: Vec<T>,
    }
    impl<T: Ord + Clone> CoordinateCompression<T> {
        /// 与えられた要素から座標圧縮空間を構築する。
        /// # 計算量
        /// O(N log N) (N = |space|)
        pub fn new(space: &[T]) -> Self {
            let space = space.iter().cloned().sorted().dedup().collect_vec();
            Self { space }
        }
        /// 与えられた値を座標圧縮したインデックスを返す。
        /// 値が空間に存在しない場合はパニックする。
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress(&self, x: &T) -> usize {
            self.space.binary_search(&x).unwrap()
        }
        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_floor(&self, x: &T) -> usize {
            self.space.upper_bound(&x) - 1
        }
        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_ceil(&self, x: &T) -> usize {
            self.space.lower_bound(&x)
        }
        /// 与えられた各要素を座標圧縮した結果を返す。
        /// # 計算量
        /// O(M log N) (M = |xs|, N = space_size)
        pub fn compress_vec(&self, xs: &[T]) -> Vec<usize> {
            xs.iter().map(|x| self.compress(x)).collect_vec()
        }

        /// 座標圧縮されたインデックスから元の値を復元する。
        /// # 計算量
        /// O(1)
        pub fn decompress(&self, i: usize) -> T {
            self.space[i].clone()
        }
        /// 座標圧縮後の空間の大きさ（要素数）を返す。
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
    }
}
use fenwick_tree_dense_multiset::*;
pub mod fenwick_tree_dense_multiset {
    use std::ops::{Bound, RangeBounds};
    /// Fenwick Tree を基盤としたマルチセット。
    /// 要素は `0` から `size - 1` までの `usize` 値に限定されます。
    /// BTreeMultiSet と違って、任意の値を挿入することはできませんが、そのかわりk番目の値が k に依らず $O(\log N)$ で取得できます。
    #[derive(Clone)]
    pub struct FenwickTreeDenseMultiset {
        ft: InternalFenwickTree,
        length: usize,
        set_length: usize,
    }
    impl std::fmt::Debug for FenwickTreeDenseMultiset {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("FenwickTreeDenseMultiset")
                .field("length", &self.length)
                .field("set_length", &self.set_length)
                .field("counts", &self.counts_vec())
                .finish()
        }
    }
    impl FenwickTreeDenseMultiset {
        /// 指定された範囲 `[0, size)` の値を管理する空のマルチセットを作成します。
        /// # 計算量
        /// $O(N)$ ($N$ は `size`)
        pub fn new(size: usize) -> Self {
            Self {
                ft: InternalFenwickTree::new(size),
                length: 0,
                set_length: 0,
            }
        }
        /// 指定した値を追加します。
        /// # Panics
        /// `value >= size` の場合にパニックします。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn insert(&mut self, value: usize) {
            self.insert_many(value, 1);
        }
        /// 指定した値を `count` 個追加します。
        /// # Panics
        /// `value >= size` の場合にパニックします。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn insert_many(&mut self, value: usize, count: usize) {
            if count == 0 {
                return;
            }
            if self.count(value) == 0 {
                self.set_length += 1;
            }
            self.ft.add(value, count as i64);
            self.length += count;
        }
        /// 要素を1つ削除します。
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn remove1(&mut self, value: usize) -> bool {
            if self.count(value) > 0 {
                self.ft.add(value, -1);
                self.length -= 1;
                if self.count(value) == 0 {
                    self.set_length -= 1;
                }
                true
            } else {
                false
            }
        }
        /// 要素を最大 `count` 個削除します。
        /// 実際に削除した個数を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn remove_up_to(&mut self, value: usize, count: usize) -> usize {
            let current = self.count(value);
            let removed = current.min(count);
            if removed > 0 {
                self.ft.add(value, -(removed as i64));
                self.length -= removed;
                if current == removed {
                    self.set_length -= 1;
                }
            }
            removed
        }
        /// 指定した要素をすべて削除します。
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn remove_all(&mut self, value: usize) -> bool {
            let current = self.count(value);
            if current > 0 {
                self.ft.add(value, -(current as i64));
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
        /// $O(\log N)$ ($N$ は `size`)
        pub fn pop_min(&mut self) -> Option<usize> {
            let val = self.min()?;
            self.remove1(val);
            Some(val)
        }
        /// 最大の要素を1つ取り出して削除します。
        /// 空の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn pop_max(&mut self) -> Option<usize> {
            let val = self.max()?;
            self.remove1(val);
            Some(val)
        }
        /// マルチセットの全要素を削除し、空にします。
        /// # 計算量
        /// $O(N)$ ($N$ は `size`)
        pub fn clear(&mut self) {
            self.ft.clear();
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
        /// # 計算量
        /// $O(1)$
        pub fn count(&self, value: usize) -> usize {
            self.ft.get(value) as usize
        }
        /// 指定した要素が含まれているかを返します。
        /// # 計算量
        /// $O(1)$
        pub fn contains(&self, value: usize) -> bool {
            self.count(value) > 0
        }
        /// 重複を考慮して、$n$ 番目に小さい要素を返します（0-indexed）。
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn nth_min(&self, n: usize) -> Option<usize> {
            let idx = self.ft.max_right(0, |&s| s <= n as i64);
            if idx < self.ft.len() { Some(idx) } else { None }
        }
        /// 重複を考慮して、$n$ 番目に大きい要素を返します（0-indexed）。
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn nth_max(&self, n: usize) -> Option<usize> {
            let length = self.length;
            if n < length {
                let target_prefix_sum = (length - 1 - n) as i64;
                Some(self.ft.max_right(0, |&s| s <= target_prefix_sum))
            } else {
                None
            }
        }
        /// 最小の要素を返します。
        /// 空の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn min(&self) -> Option<usize> {
            let idx = self.ft.max_right(0, |&s| s == 0);
            if idx < self.ft.len() { Some(idx) } else { None }
        }
        /// 最大の要素を返します。
        /// 空の場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn max(&self) -> Option<usize> {
            let idx = self.ft.min_left(self.ft.len(), |&s| s == 0);
            if idx > 0 { Some(idx - 1) } else { None }
        }
        fn resolve_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
            let n = self.ft.len();
            let l = match range.start_bound() {
                Bound::Included(&l) => l,
                Bound::Excluded(&l) => l + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(&r) => r + 1,
                Bound::Excluded(&r) => r,
                Bound::Unbounded => n,
            };
            assert!(
                l <= r && r <= n,
                "FenwickTreeDenseMultiset::resolve_range: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                n
            );
            (l, r)
        }
        /// 指定された範囲内での最小の要素を返します。
        /// 範囲内に要素がない場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn min_in_range<R: RangeBounds<usize>>(&self, range: R) -> Option<usize> {
            let (l, r) = self.resolve_range(range);
            let idx = self.ft.max_right(l, |&s| s == 0);
            if idx < r { Some(idx) } else { None }
        }
        /// 指定された範囲内での最大の要素を返します。
        /// 範囲内に要素がない場合は `None` を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn max_in_range<R: RangeBounds<usize>>(&self, range: R) -> Option<usize> {
            let (l, r) = self.resolve_range(range);
            let idx = self.ft.min_left(r, |&s| s == 0);
            if idx > l { Some(idx - 1) } else { None }
        }
        /// 指定された範囲内で、重複を考慮して $n$ 番目に小さい要素を返す（0-indexed）。
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn nth_min_in_range<R: RangeBounds<usize>>(&self, n: usize, range: R) -> Option<usize> {
            let (l, r) = self.resolve_range(range);
            let idx = self.ft.max_right(l, |&s| s <= n as i64);
            if idx < r { Some(idx) } else { None }
        }
        /// 指定された範囲内で、重複を考慮して $n$ 番目に大きい要素を返す（0-indexed）。
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn nth_max_in_range<R: RangeBounds<usize>>(&self, n: usize, range: R) -> Option<usize> {
            let (l, r) = self.resolve_range(range);
            let range_count = self.ft.range_sum(l..r) as usize;
            if n < range_count {
                let target_prefix_sum = (range_count - 1 - n) as i64;
                Some(self.ft.max_right(l, |&s| s <= target_prefix_sum))
            } else {
                None
            }
        }
        /// 指定された範囲内の要素数（重複を含む）を返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn count_in_range<R: RangeBounds<usize>>(&self, range: R) -> usize {
            let (l, r) = self.resolve_range(range);
            if l >= r {
                return 0;
            }
            self.ft.range_sum(l..r) as usize
        }
        /// 指定した範囲内に要素が含まれているかを返します。
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn contains_in_range<R: RangeBounds<usize>>(&self, range: R) -> bool {
            self.count_in_range(range) > 0
        }
        /// 各要素の出現回数を Vec で返します。
        /// # 計算量
        /// $O(N)$ ($N$ は `size`)
        pub fn counts_vec(&self) -> Vec<usize> {
            self.ft.vals.iter().map(|&v| v as usize).collect()
        }
    }
    /// Fenwick Tree の基本操作を提供する補助構造体。
    /// `range_sum_fenwick_tree.rs` に準拠した実装。
    #[derive(Clone)]
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
        fn clear(&mut self) {
            self.ary.fill(0);
            self.vals.fill(0);
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
