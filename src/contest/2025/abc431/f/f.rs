// #[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        xs: [Usize1; n],
    }

    use ac_library::ModInt998244353 as Mint;
    let comb = Comb::<Mint>::new(n + 1);

    let max = xs.iter().copied().max().unwrap();
    let mut cnts = FenwickTree::new(max + 1, 0_i64);
    for &x in &xs {
        cnts.add(x, 1);
    }
    let mut ans = Mint::new(1);

    for x in 0..=max {
        // [x-d, x)
        let begin = x.saturating_sub(d);
        let end = x;
        // 挿入可能場所
        let cnt_pos = cnts.sum(begin..end);

        // 挿入したい数
        let cnt_insert = cnts.sum(x..x + 1);
        ans *= comb.comb((cnt_pos + cnt_insert) as usize, cnt_insert as usize);
    }

    let ans: i64 = ans.val() as i64;
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

use ac_library::FenwickTree;
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
use mod_combinatorics::*;
pub mod mod_combinatorics {
    use ac_library::modint::ModIntBase;
    pub struct Comb<Mint: ModIntBase> {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }
    impl<Mint: ModIntBase> Comb<Mint> {
        pub fn new(max_val: usize) -> Self {
            let mut inv = vec![Mint::new(0); max_val + 1];
            let mut fac = vec![Mint::new(0); max_val + 1];
            let mut invfac = vec![Mint::new(0); max_val + 1];
            fac[0] = 1.into();
            fac[1] = 1.into();
            invfac[0] = 1.into();
            invfac[1] = 1.into();
            inv[1] = 1.into();
            let modulus = Mint::modulus() as usize;
            for i in 2..=max_val {
                inv[i] = -inv[modulus % i] * Mint::new(modulus / i);
                fac[i] = fac[i - 1] * Mint::new(i);
                invfac[i] = invfac[i - 1] * inv[i];
            }
            Self { fac, invfac }
        }
        pub fn comb(&self, n: usize, k: usize) -> Mint {
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[k] * self.invfac[n - k]
            }
        }
        pub fn perm(&self, n: usize, k: usize) -> Mint {
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[n - k]
            }
        }
        pub fn factorial(&self, n: usize) -> Mint {
            self.fac[n]
        }
        pub fn inv_factorial(&self, n: usize) -> Mint {
            self.invfac[n]
        }
    }
}
use counts_vec::*;
#[allow(clippy::module_inception)]
pub mod counts_vec {
    pub trait IteratorCountsVec: Iterator<Item = usize> + Sized {
        fn counts_vec(self, size: usize) -> Vec<i64> {
            let mut counts = vec![0; size];
            self.for_each(|item| counts[item] += 1);
            counts
        }
    }
    impl<T: Iterator<Item = usize>> IteratorCountsVec for T {}
}
