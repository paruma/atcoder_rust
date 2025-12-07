// 区間をセットで管理するテク
#[fastout]
fn main() {
    input! {
        n: i64,
        q: usize,
        lrs: [(i64, i64); q],
    }

    let mut set = RangeSet::new(n); // [1,n] だけ入ってる

    for &(l, r) in &lrs {
        set.remove_range(l, r);
        let ans = set.len();
        println!("{}", ans);
        // dbg!(seg.to_vec());
    }

    // dbg!(cnts);

    // dbg!(coord);
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

use range_set::RangeSet;
// 参考: 要素の追加・削除と mex を対数時間で処理するよ - えびちゃんの日記
// https://rsk0315.hatenablog.com/entry/2020/10/11/125049
pub mod range_set {
    use std::collections::BTreeSet;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RangeSet {
        set: BTreeSet<(i64, i64)>, // 閉区間を管理する
        count: usize,
    }

    impl RangeSet {
        pub fn new(n: i64) -> RangeSet {
            RangeSet {
                set: vec![(i64::MIN, i64::MIN), (1, n), (i64::MAX, i64::MAX)] // 番兵
                    .into_iter()
                    .collect(),
                count: n as usize,
            }
        }

        pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
            self.set
                .iter()
                .copied()
                .filter(|&(l, r)| (l, r) != (i64::MIN, i64::MIN) && (l, r) != (i64::MAX, i64::MAX)) // 番兵は除く
                .flat_map(|(left, right)| left..=right)
        }

        pub fn insert(&mut self, x: i64) -> bool {
            if self.contains(x) {
                return false;
            }

            // 番兵がいるので unwrap 可能。
            let &(prev_l, prev_r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            let &(next_l, next_r) = self.set.range((x + 1, x + 1)..).min().unwrap();

            // 以下の4パターンがある ('x' が insert する値。"[ ]" が既存の区間 )
            // [ ]x[ ]
            // [ ]x  [ ]
            // [ ]  x[ ]
            // [ ]  x  [ ]

            if prev_r + 1 == x && x == next_l - 1 {
                self.set.remove(&(prev_l, prev_r));
                self.set.remove(&(next_l, next_r));
                self.set.insert((prev_l, next_r));
            } else if prev_r + 1 == x {
                self.set.remove(&(prev_l, prev_r));
                self.set.insert((prev_l, x));
            } else if x == next_l - 1 {
                self.set.remove(&(next_l, next_r));
                self.set.insert((x, next_r));
            } else {
                self.set.insert((x, x));
            }

            self.count += 1;

            true
        }

        pub fn remove_range(&mut self, left: i64, right: i64) {
            // 管理している区間を右から左に見ていく
            let (mut sl, mut sr) = *self.set.range(..(right + 1, right + 1)).max().unwrap();
            // (sl, sr) を処理する
            let mut to_insert: Vec<(i64, i64)> = vec![];
            loop {
                if sr < left {
                    break;
                }
                self.set.remove(&(sl, sr));
                self.count -= (sr - sl + 1) as usize;

                if left <= sl && right < sr {
                    to_insert.push((right + 1, sr));
                } else if sl < left && right < sr {
                    to_insert.push((sl, left - 1));
                    to_insert.push((right + 1, sr));
                } else if sl < left && sr <= right {
                    to_insert.push((sl, left - 1));
                }

                (sl, sr) = *self.set.range(..(sl, sl)).max().unwrap();
            }
            for (sl, sr) in to_insert {
                self.set.insert((sl, sr));
                self.count += (sr - sl + 1) as usize;
            }
        }

        pub fn remove(&mut self, x: i64) -> bool {
            if !self.contains(x) {
                return false;
            }

            let &(current_l, current_r) = self.set.range(..(x + 1, x + 1)).max().unwrap();

            // 削除のパターンは以下の4通り
            //  [x]
            // → (消滅)
            //
            //  [x    ]
            // →  [   ]
            //
            //  [    x]
            //→ [   ]
            //
            //  [  x  ]
            // →[ ] [ ]

            if current_l == x && x == current_r {
                self.set.remove(&(current_l, current_r));
            } else if current_l == x {
                self.set.remove(&(current_l, current_r));
                self.set.insert((x + 1, current_r));
            } else if x == current_r {
                self.set.remove(&(current_l, current_r));
                self.set.insert((current_l, x - 1));
            } else {
                self.set.remove(&(current_l, current_r));
                self.set.insert((current_l, x - 1));
                self.set.insert((x + 1, current_r));
            }

            self.count -= 1;
            true
        }

        pub fn len(&self) -> usize {
            self.count
        }

        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        pub fn contains(&self, x: i64) -> bool {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            (l..=r).contains(&x)
        }

        /// x 以上で self に入っていない値の最小値を返す (いわゆる mex)
        pub fn min_exclusive_geq(&self, x: i64) -> i64 {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            if (l..=r).contains(&x) { r + 1 } else { x }
        }
        /// x 以下で self に入っていない値の最大値を返す
        pub fn max_exclusive_leq(&self, x: i64) -> i64 {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            if (l..=r).contains(&x) { l - 1 } else { x }
        }
    }
}
