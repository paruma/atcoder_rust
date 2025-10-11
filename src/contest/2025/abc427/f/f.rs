fn solve0(xs: &[i64], m: i64) -> (Vec<i64>, Vec<i64>) {
    let n = xs.len();

    // ここ遅そう。間に合う？
    // dp0, dp1 って分けたほうが早そう
    let mut dp: Vec<Vec<i64>> = vec![vec![]; 2];

    dp[0] = vec![0];
    dp[1] = vec![xs[0]];

    for i in 2..=n {
        let mut ndp = dp.clone();
        ndp[0] = chain!(&dp[0], &dp[1]).copied().collect_vec();

        ndp[1] = dp[0]
            .iter()
            .copied()
            .map(|x| (x + xs[i - 1]) % m)
            .collect_vec();

        dp = ndp;
    }

    // dbg!(&dp[0]); // 最後の値使ってない
    // dbg!(&dp[1]); // 最後の値使った
    (dp[0].clone(), dp[1].clone())
}

fn solve1(cnts0: &HashMap<i64, usize>, cnts1: &HashMap<i64, usize>, m: i64) -> i64 {
    cnts0
        .iter()
        .map(|(&k, &cnt)| {
            let other = (m - k) % m;
            let other_cnt = cnts1.get(&other).copied().unwrap_or(0) as i64;
            (cnt as i64) * other_cnt
        })
        .sum::<i64>()
}

fn solve(n: usize, m: i64, xs: &[i64]) -> i64 {
    if n == 1 {
        return if xs[0] == 0 { 2 } else { 1 };
    }

    let xs0 = xs[..n / 2].to_vec();
    let xs1 = xs[n / 2..].iter().copied().rev().collect_vec();

    let (ys00, ys01) = solve0(&xs0, m);
    let (ys10, ys11) = solve0(&xs1, m);

    // 末尾0の方は vec にしていいかも
    let ys00_cnts = ys00.iter().copied().counts();
    let ys01_cnts = ys01.iter().copied().counts();
    let ys10_cnts = ys10.iter().copied().counts();
    let ys11_cnts = ys11.iter().copied().counts();

    // 最後が0: 末尾を使ってない
    // 最後が1: 末尾を使った
    let term1 = solve1(&ys00_cnts, &ys10_cnts, m);
    let term2 = solve1(&ys01_cnts, &ys10_cnts, m);
    let term3 = solve1(&ys00_cnts, &ys11_cnts, m);

    term1 + term2 + term3
}
fn main() {
    input! {
        n: usize,
        m: i64,
        xs: [i64; n],
    }

    let ans: i64 = solve(n, m, &xs);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};
    #[test]
    fn test() {
        solve0(&[1, 2, 4, 8], 10000);
    }

    #[test]
    fn test_problem() {
        let n = 30;
        let mut dp = vec![[0_i128; 2]; n + 1];

        dp[1][0] = 1;
        dp[1][1] = 1;

        for i in 2..=n {
            dp[i][0] = dp[i - 1][0] + dp[i - 1][1];
            dp[i][1] = dp[i - 1][0];
        }

        // dbg!(&dp);
        // dbg!(dp[n][0] + dp[n][1]);

        assert_eq!(1 + 1, 2);
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.gen_range(1..=10);
        let xs = (0..n).map(|_| rng.gen_range(0..10)).collect_vec();

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
        // let mut rng = SmallRng::from_entropy();
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
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use {
    itertools::{chain, iproduct, izip, Itertools},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng},
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
