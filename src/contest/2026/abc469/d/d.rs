// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abs: [(Usize1, Usize1); m],
    }

    let abs = abs.iter().copied().unique().collect_vec();
    let m = abs.len();

    let adj = abs
        .iter()
        .copied()
        .fold(vec![vec![]; n], |mut acc, (a, b)| {
            acc[a].push(b);
            acc[b].push(a);
            acc
        });

    let mut cnts = vec![0; n]; // [0, m] の値を取る
    let mut cnts_cnts = vec![0; (m + 1).max(n + 1)];
    cnts_cnts[0] = n;

    for a in abs.iter().copied().flat_map(|(a, b)| [a, b]) {
        cnts_cnts[cnts[a]] -= 1;
        cnts[a] += 1;
        cnts_cnts[cnts[a]] += 1;
    }

    // dbg!(&cnts);
    // dbg!(&cnts_cnts);
    let mut ans = 0;

    for x in 0..n {
        for &a in &adj[x] {
            cnts_cnts[cnts[a]] -= 1;
            cnts[a] -= 1;
            cnts_cnts[cnts[a]] += 1;

            cnts_cnts[cnts[x]] -= 1;
            cnts[x] -= 1;
            cnts_cnts[cnts[x]] += 1;
        }
        // なんかする

        // dbg!(&cnts);
        // dbg!(&cnts_cnts);

        // dbg!(adj[x].len());
        ans += cnts_cnts[m - adj[x].len()];
        if cnts[x] == m - adj[x].len() {
            ans -= 1;
        }

        for &a in &adj[x] {
            cnts_cnts[cnts[a]] -= 1;
            cnts[a] += 1;
            cnts_cnts[cnts[a]] += 1;

            cnts_cnts[cnts[x]] -= 1;
            cnts[x] += 1;
            cnts_cnts[cnts[x]] += 1;
        }
        //
    }

    println!("{}", ans / 2);
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
