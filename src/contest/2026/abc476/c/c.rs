// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [i64; n],
    }
    let mut bag: TopKMultiset<i64, 3> = TopKMultiset::new();

    for i in 0..n {
        bag.insert(xs[i]);
        if i >= 2 {
            let ans = bag.nth(2).unwrap();
            println!("{}", ans);
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
use topk_multiset::*;
#[allow(clippy::module_inception)]
pub mod topk_multiset {
    use std::fmt;
    /// 値が大きい方から最大 K 個を保持するマルチセット（同一値の重複あり）。
    /// ヒープを使用せず、スタック上の固定長配列で動作する。
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TopKMultiset<T, const K: usize> {
        buf: [T; K],
        len: usize,
    }
    /// 値が大きい方から最大 2 個を保持するマルチセット。
    pub type Top2Multiset<T> = TopKMultiset<T, 2>;
    /// 値が大きい方から最大 3 個を保持するマルチセット。
    pub type Top3Multiset<T> = TopKMultiset<T, 3>;
    /// 値が大きい方から最大 4 個を保持するマルチセット。
    pub type Top4Multiset<T> = TopKMultiset<T, 4>;
    /// 値が大きい方から最大 5 個を保持するマルチセット。
    pub type Top5Multiset<T> = TopKMultiset<T, 5>;
    impl<T, const K: usize> TopKMultiset<T, K>
    where
        T: Ord + Copy + Default,
    {
        /// 空の TopKMultiset を作成する。
        /// 計算量は $O(K)$。
        pub fn new() -> Self {
            Self {
                buf: [T::default(); K],
                len: 0,
            }
        }
        /// 要素を 1 つだけ含む TopKMultiset を作成する。
        /// 計算量は $O(K)$。
        pub fn unit(value: T) -> Self {
            Self::new().inserted(value)
        }
        /// 要素を 1 つ追加する。
        /// 計算量は $O(K)$。
        pub fn insert(&mut self, value: T) {
            let pos = self.buf[..self.len]
                .iter()
                .position(|&x| value >= x)
                .unwrap_or(self.len);
            if self.len < K {
                self.len += 1;
            } else if pos == K {
                return;
            }
            self.buf.copy_within(pos..self.len - 1, pos + 1);
            self.buf[pos] = value;
        }
        /// 要素を 1 つ追加した新しい TopKMultiset を返す。
        /// 計算量は $O(K)$。
        #[must_use]
        pub fn inserted(self, value: T) -> Self {
            let mut result = self;
            result.insert(value);
            result
        }
        /// other の全要素を追加する。
        /// 計算量は $O(K^2)$。
        pub fn merge(&mut self, other: Self) {
            for x in other.iter() {
                self.insert(x);
            }
        }
        /// other の全要素を追加した新しい TopKMultiset を返す。
        /// 計算量は $O(K^2)$。
        #[must_use]
        pub fn merged(self, other: Self) -> Self {
            let mut result = self;
            result.merge(other);
            result
        }
        /// i 番目に大きい要素を返す（0-indexed）。
        /// i >= len の場合は None を返す。計算量は $O(1)$。
        pub fn nth(&self, i: usize) -> Option<T> {
            if i < self.len {
                Some(self.buf[i])
            } else {
                None
            }
        }
        /// 保持している最大の要素を返す。
        /// `nth(0)` と同じ。計算量は $O(1)$。
        pub fn max(&self) -> Option<T> {
            self.nth(0)
        }
        /// 保持している要素数を返す。
        /// 計算量は $O(1)$。
        pub fn len(&self) -> usize {
            self.len
        }
        /// 空かどうかを返す。
        /// 計算量は $O(1)$。
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
        /// 保持している要素のイテレータを返す（T 降順）。
        pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
            self.buf[..self.len].iter().copied()
        }
    }
    impl<T, const K: usize> Default for TopKMultiset<T, K>
    where
        T: Ord + Copy + Default,
    {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T, const K: usize> FromIterator<T> for TopKMultiset<T, K>
    where
        T: Ord + Copy + Default,
    {
        /// イテレータの各要素を順に insert した結果と等価。
        /// 計算量は $O(NK)$（N はイテレータの要素数）。
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut result = Self::new();
            for x in iter {
                result.insert(x);
            }
            result
        }
    }
    impl<T: Copy, const K: usize> IntoIterator for TopKMultiset<T, K> {
        type Item = T;
        type IntoIter = std::iter::Take<std::array::IntoIter<T, K>>;
        fn into_iter(self) -> Self::IntoIter {
            self.buf.into_iter().take(self.len)
        }
    }
    impl<T: fmt::Debug, const K: usize> fmt::Debug for TopKMultiset<T, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for (i, x) in self.buf[..self.len].iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", x)?;
            }
            write!(f, "}}")
        }
    }
}
