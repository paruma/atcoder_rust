// 問題文と制約は読みましたか？
// #[fastout]

fn nc2(n: i64) -> i64 {
    n * (n - 1) / 2
}

fn main() {
    input! {
        n: usize,
        d: usize,
        sts: [(Usize1, Usize1); n],
    }
    let max = sts.iter().copied().flat_map(|(s, t)| [s, t]).max().unwrap();
    let mut imos = RangeAddImos::new(max + 1);

    for (s, t) in sts {
        if t >= d - 1 && s < t - d + 1 {
            imos.range_add(s..t - d + 1, 1);
        }
    }

    let cnts = imos.to_vec();
    let ans = cnts.iter().copied().map(|cnt| nc2(cnt)).sum::<i64>();
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
use range_add_imos::*;
#[allow(clippy::module_inception)]
pub mod range_add_imos {
    /// いもす法（差分配列）を用いて、配列に対する区間加算クエリを効率的に処理するデータ構造です。
    /// 最終的な配列の状態を一度に計算する場合（オフライン処理）に特に有用です。
    /// 各区間加算操作はO(1)で、最終的な配列を構築するのにO(N)かかります。
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RangeAddImos {
        n: usize,
        diff: Vec<i64>,
    }
    impl RangeAddImos {
        /// サイズ `n` の新しい `RangeAddImos` インスタンスを作成します。
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            Self {
                n,
                diff: vec![0; n + 1],
            }
        }
        /// 指定された `range` に `x` を加算します。
        /// # Panics
        /// 範囲が不正な場合にパニックします。
        /// # 計算量
        /// O(1)
        pub fn range_add(&mut self, range: impl std::ops::RangeBounds<usize>, x: i64) {
            let range = open_range_bounds(range, self.n);
            let l = range.start;
            let r = range.end;
            assert!(l <= r && r <= self.n);
            self.diff[l] += x;
            self.diff[r] -= x;
        }
        /// 差分配列から最終的な配列を構築します。
        /// # 計算量
        /// O(n)
        pub fn to_vec(mut self) -> Vec<i64> {
            if self.n == 0 {
                return Vec::new();
            }
            for i in 1..self.n {
                self.diff[i] += self.diff[i - 1];
            }
            self.diff.truncate(self.n);
            self.diff
        }
        /// 指定されたインデックス `p` に `x` を加算します。
        /// # Panics
        /// `p >= n` の場合にパニックする可能性があります。
        /// `range_add` の内部で範囲チェックが行われます。
        /// # 計算量
        /// O(1)
        pub fn add(&mut self, p: usize, x: i64) {
            self.range_add(p..(p + 1), x);
        }
        /// 配列スライスから`RangeAddImos`インスタンスを作成します。
        /// # 計算量
        /// O(n)
        pub fn from_slice(xs: &[i64]) -> Self {
            let n = xs.len();
            let mut diff = vec![0; n + 1];
            if n > 0 {
                diff[0] = xs[0];
                for i in 1..n {
                    diff[i] = xs[i] - xs[i - 1];
                }
            }
            Self { n, diff }
        }
    }
    fn open_range_bounds(
        range: impl std::ops::RangeBounds<usize>,
        len: usize,
    ) -> std::ops::Range<usize> {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(&x) => x,
            Excluded(&x) => x + 1,
        };
        let r = match range.end_bound() {
            Unbounded => len,
            Included(&x) => x + 1,
            Excluded(&x) => x,
        };
        l..r
    }
}
