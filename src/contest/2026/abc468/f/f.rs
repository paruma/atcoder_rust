fn main() {
    input! {
        n: usize,
        xs: [Usize1; n],
    }

    // 貪欲単調増加列(left-to-right maxima, record hights)を除く
    let ys = {
        let mut ys = vec![];
        let mut cur_max = xs[0];

        for &x in &xs[1..] {
            if cur_max < x {
                cur_max = x;
            } else {
                ys.push(x)
            }
        }
        ys
    };

    let ans = (n - ys.len()) + lis_len(&ys) as usize;
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
use {
    ac_library::{Monoid, Segtree},
    num_traits::WrappingAdd,
    std::convert::Infallible,
};
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
use lis::*;
#[allow(clippy::module_inception)]
pub mod lis {
    use ac_library::{Max, Segtree};
    use itertools::Itertools;
    use superslice::Ext;
    /// 各要素を末尾とする最長増加部分列 (LIS) の長さを求める
    ///
    /// 戻り値の `i` 番目の要素は、`xs[i]` を末尾とする LIS の長さを表す。
    ///
    /// # 計算量
    /// O(N log N)
    pub fn lis_array<T: Ord>(xs: &[T]) -> Vec<i64> {
        let n = xs.len();
        if n == 0 {
            return vec![];
        }
        let sorted = xs.iter().sorted().dedup().collect_vec();
        let rank = xs
            .iter()
            .map(|x| sorted.binary_search(&x).unwrap())
            .collect_vec();
        let mut dp = vec![0; n];
        let mut seg = Segtree::<Max<i64>>::new(sorted.len());
        for (i, &x) in rank.iter().enumerate() {
            let prev_lis = seg.prod(..x);
            dp[i] = if prev_lis == i64::MIN {
                1
            } else {
                prev_lis + 1
            };
            if seg.get(x) < dp[i] {
                seg.set(x, dp[i]);
            }
        }
        dp
    }
    /// 最長増加部分列 (LIS) の長さを求める
    ///
    /// # 計算量
    /// O(N log N)
    pub fn lis_len<T: Ord>(xs: &[T]) -> i64 {
        let mut dp = vec![];
        for x in xs {
            let i = dp.lower_bound(&x);
            if i < dp.len() {
                dp[i] = x;
            } else {
                dp.push(x);
            }
        }
        dp.len() as i64
    }
}
