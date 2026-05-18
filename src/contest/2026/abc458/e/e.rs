// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        x1: usize,
        x2: usize,
        x3: usize,
    }
    use ac_library::ModInt998244353 as Mint;
    let sum = x1 + x2 + x3;

    let comb = Comb::<Mint>::new(2 * sum + 2);

    let ans = (0..x1)
        .map(|k| {
            // dbg!(k);
            // 1 が隣り合ってる場所が k 個ある
            let sub = if x2 < x1 - 1 - k {
                Mint::new(0)
            } else {
                let rem2 = x2 - (x1 - 1 - k);
                let factor0 = comb.comb(x1 - 1, k);
                let factor1 = comb.h(x1 + 1 - k, rem2);
                let factor2 = comb.h(rem2, x3);
                // dbg!(rem2);
                // dbg!(factor1);
                // dbg!(factor2);
                factor0 * factor1 * factor2
            };
            // dbg!(sub);
            sub
        })
        .sum::<Mint>();
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
use mod_combinatorics::*;
pub mod mod_combinatorics {
    use ac_library::modint::ModIntBase;
    #[derive(Clone, Debug)]
    pub struct Comb<Mint: ModIntBase> {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }
    impl<Mint: ModIntBase> Comb<Mint> {
        /// 階乗とその逆元を `max_val` まで前計算する。
        /// 計算量: O(max_val)
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
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[k] * self.invfac[n - k]
            }
        }

        pub fn h(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            if n == 0 {
                0.into()
            } else {
                self.comb(n + k - 1, k)
            }
        }
        pub fn perm(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[n - k]
            }
        }
        pub fn factorial(&self, n: usize) -> Mint {
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            self.fac[n]
        }
        pub fn inv_factorial(&self, n: usize) -> Mint {
            assert!(
                n < self.invfac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.invfac.len() - 1
            );
            self.invfac[n]
        }
    }
}
