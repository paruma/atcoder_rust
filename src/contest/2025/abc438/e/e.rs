// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        fs: [Usize1; n],
        tbs: [(usize, Usize1); q],
    }

    let gs = (0..n).map(|i| (i + 1) as i64).collect_vec();

    let doubling = DoublingWithSum::new(&fs, &gs, 1_000_000_000);

    let ans = tbs
        .iter()
        .copied()
        .map(|(t, b)| doubling.eval(t, b).1)
        .collect_vec();
    print_vec(&ans);
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
use doubling_with_sum::*;
#[allow(clippy::module_inception)]
pub mod doubling_with_sum {
    pub struct DoublingWithSum {
        n: usize,
        log: usize,
        dp_f: Vec<Vec<usize>>,
        dp_g: Vec<Vec<i64>>,
    }
    impl DoublingWithSum {
        /// doubling 前処理の構築をする
        /// * `k` は 合成回数の最大値 (`k>=1`)
        /// * `f[x]` は `x` の遷移先
        /// * `g[x]` は `x→f[x]` の辺重み
        /// # 計算量
        /// `n = f.len()` としたとき、`O(n log k)`
        pub fn new(f: &[usize], g: &[i64], k: usize) -> DoublingWithSum {
            let n = f.len();
            let log = (usize::BITS - k.leading_zeros()) as usize;
            let mut dp_f = vec![vec![0; n]; log];
            let mut dp_g = vec![vec![0; n]; log];
            if k >= 1 {
                dp_f[0] = f.to_vec();
                dp_g[0] = g.to_vec();
            }
            for i in 1..log {
                for x in 0..n {
                    let fp = &dp_f[i - 1];
                    let gp = &dp_g[i - 1];
                    dp_g[i][x] = gp[x] + gp[fp[x]];
                    dp_f[i][x] = fp[fp[x]];
                }
            }
            DoublingWithSum { n, log, dp_f, dp_g }
        }
        /// `f` の `k` 回合成を `f^k` とする。
        /// `(f^k)(x)` と `x → f(x) → ... → (f^k)(x)` のパス重みを求める。
        /// # 計算量
        /// O(log k)
        pub fn eval(&self, k: usize, x: usize) -> (usize, i64) {
            assert!((0..self.n).contains(&x));
            assert!(k < (1 << self.log));
            if k == 0 {
                return (x, 0);
            }
            let k_bits = (usize::BITS - k.leading_zeros()) as usize;
            self.dp_f
                .iter()
                .zip(self.dp_g.iter())
                .enumerate()
                .take(k_bits)
                .filter(|(i, _)| (k >> i) & 1 == 1)
                .map(|(_, (fp, gp))| (fp, gp))
                .fold((x, 0), |(idx, val), (fp, gp)| (fp[idx], val + gp[idx]))
        }
    }
}
