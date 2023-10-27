#[derive_readable]
struct Problem {
    a: i64,
    b: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem,
        }
        p
    }
    fn solve(&self) -> Answer {
        // mod p と mod 2 を両方計算 (p=998244353)
        use ac_library::ModInt998244353 as Mint998;
        type Mint2 = ModInt2;

        let a = self.a;
        let b = self.b;

        // (1) A^B の約数の個数を求める
        // (1-1) A を素因数分解する
        let fa = prime_factorize(a);
        // (1-2) Aの素因数分解から A^B の約数の個数を求める
        let n_divisors: Mint998 =
            fa.values().map(|&cnt| Mint998::new(cnt) * Mint998::new(b) + Mint998::new(1)).product();
        let n_divisors_mod2: Mint2 =
            fa.values().map(|&cnt| Mint2::new(cnt) * Mint2::new(b) + Mint2::new(1)).product();

        // (2) A^Bの約数の総積はAで何回割れるか求める
        // A^Bの約数の総積は(A^B)^(n_divisors/2) =A^((B * n_divisors)/2)
        // つまり、floor((B * n_divisors)/2) 回割れる
        // ↓こう書くとWA になる
        // let ans = x * RF::new(self.b) / RF::new(2);
        // (2-1) B * n_divisors を計算
        let ans0 = n_divisors * Mint998::new(b);
        let ans0_mod2 = n_divisors_mod2 * Mint2::new(b);
        // (2-2) floor((B * n_divisors)/2) を計算
        let ans = if ans0_mod2.val() == 0 {
            ans0 / Mint998::new(2)
        } else {
            (ans0 - Mint998::new(1)) / Mint998::new(2)
        };

        // let ans = (ans0 - Mint998::new(ans0_mod2.val())) / Mint998::new(2);
        let ans = ans.val() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // mod 2*p で計算
        type Mint = ModInt2times998;

        let a = self.a;
        let b = self.b;

        // (1) A^B の約数の個数を求める
        // (1-1) A を素因数分解する
        let fa = prime_factorize(a);
        // (1-2) Aの素因数分解から A^B の約数の個数を求める
        let n_divisors: Mint =
            fa.values().map(|&cnt| Mint::new(cnt) * Mint::new(b) + Mint::new(1)).product();

        // (2) A^Bの約数の総積はAで何回割れるか求める
        // A^Bの約数の総積は(A^B)^(n_divisors/2) =A^((B * n_divisors)/2)
        // つまり、floor((B * n_divisors)/2) 回割れる
        // ↓こう書くとWA になる
        // let ans = x * RF::new(self.b) / RF::new(2);
        // (2-1) B * n_divisors を計算
        let ans0 = n_divisors * Mint::new(b);
        // (2-2) floor((B * n_divisors)/2) を計算
        // ℤ/2pℤ → ℤ/pℤ; [x] ↦ [floor(x/2)] は well-defined なので、単にこう計算すれば良い。
        let ans = (ans0.val() / 2) as i64;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(Problem { a: 4, b: 1 }.solve().ans, 1);
        assert_eq!(1 + 1, 2);
    }
}

use std::{cell::RefCell, collections::HashMap, thread::LocalKey};

use ac_library::{ButterflyCache, Modulus, StaticModInt};
// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
use num_integer::{Integer, Roots};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======

pub fn prime_factorize(n: i64) -> HashMap<i64, i64> {
    assert!(n >= 1);
    let mut cnt_table: HashMap<i64, i64> = HashMap::new();
    let mut n = n;
    for i in 2..=n.sqrt() {
        if n.is_multiple_of(&i) {
            // n を i で割れるだけ割る
            let mut cnt = 0;
            while n.is_multiple_of(&i) {
                n /= i;
                cnt += 1;
            }
            cnt_table.insert(i, cnt);
        }
    }
    if n != 1 {
        cnt_table.insert(n, 1);
    }
    cnt_table
}

use static_mod_int::*;
pub mod static_mod_int {
    use ac_library::{ButterflyCache, Modulus, StaticModInt};
    use std::{cell::RefCell, thread::LocalKey};
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod2 {}
    impl Modulus for Mod2 {
        const VALUE: u32 = 2;
        const HINT_VALUE_IS_PRIME: bool = true;
        fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
            thread_local! {static BUTTERFLY_CACHE : RefCell < Option < ButterflyCache < Mod2 >>> = RefCell :: default () ; }
            &BUTTERFLY_CACHE
        }
    }
    pub type ModInt2 = StaticModInt<Mod2>;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod2times998 {}

    impl Modulus for Mod2times998 {
        const VALUE: u32 = 2 * 998244353;
        const HINT_VALUE_IS_PRIME: bool = false;

        fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
            thread_local! {
                static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod2times998>>> = RefCell::default();
            }
            &BUTTERFLY_CACHE
        }
    }
    pub type ModInt2times998 = StaticModInt<Mod2times998>;
}
