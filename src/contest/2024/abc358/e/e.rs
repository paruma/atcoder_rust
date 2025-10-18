//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    k: usize,
    cs: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            k: usize,
            cs: [usize; 26],
        }
        Problem { k, cs }
    }
    fn solve(&self) -> Answer {
        // 解法: 挿入DP
        use ac_library::ModInt998244353 as Mint;
        let k = self.k;
        let cs = &self.cs;
        // dp[i][x]: 0-origin (i-1)番目までのアルファベットを使ってx文字作る
        let mut dp = vec![vec![Mint::new(0); k + 1]; 26 + 1];
        dp[0][0] = Mint::new(1);

        let comb = Comb::new(k);

        for i in 0..26 {
            for x in 0..=k {
                // addition: i文字目のアルファベットを何文字使うか
                for addition in 0..=cs[i] {
                    if x + addition <= k {
                        // 長さ x に addition 個の文字を挿入する場合の数は comb(x + addition, addition) 通り
                        let tmp = dp[i][x] * comb.comb(x + addition, addition);
                        dp[i + 1][x + addition] += tmp;
                    }
                }
            }
        }
        let ans = (1..=k).map(|i| dp[26][i]).sum::<Mint>().val() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // FFT(畳み込み) を使った解法
        // 例えばA, B, Cの3種類のアルファベットで6文字を作る場合、
        // Aを使う数をa, Bを使う数をb, Cを使う数をc としたとき
        // a + b + c = 6 が成り立つ
        // 作れる文字は 6!/(a!b!c!) 通り。
        // 和が一定といえば畳み込み(多項式のかけ算) である。

        use ac_library::ModInt998244353 as Mint;
        let k = self.k;
        let cs = &self.cs;
        let c_max = cs.iter().copied().max().unwrap();
        // dp[i][x]: 0-origin (i-1)番目までのアルファベットを使ってx文字作る

        let comb = Comb::new(usize::max(k, c_max));
        let inv_fac_list = (0..=c_max).map(|i| comb.inv_factorial(i)).collect_vec();

        // mod 998244353 での畳み込みは長さ 2^23 ≒ 8×10^6 までできる。
        // 今回の制約であれば問題ない (長さはたかだか 2.6×10^4 にしかならない)
        let convolution_result = (0..26)
            .map(|i| &inv_fac_list[0..=cs[i]])
            .fold(vec![Mint::new(1)], |acc, x| {
                ac_library::convolution(&acc, x)
            });

        // 畳み込みの結果は長さが足りないケースがあるので注意(足りない分は0として扱う)
        let ans = (1..=k)
            .map(|l| comb.factorial(l) * convolution_result.get(l).unwrap_or(&Mint::new(0)))
            .sum::<Mint>()
            .val() as i64;

        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
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
    Problem::read().solve2().print();
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

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
use mod_combinatorics::*;
pub mod mod_combinatorics {
    use ac_library::ModInt998244353 as Mint;
    pub struct Comb {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }
    impl Comb {
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
