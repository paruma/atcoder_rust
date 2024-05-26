#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Condition {
    a: Usize1,
    b: Usize1,
    c: Usize1,
}
#[derive(Debug)]
struct Problem {
    len: usize,
    n_conds: usize,
    conds: Vec<Condition>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            len: usize,
            n_conds: usize,
            conds: [Condition; n_conds],
        }
        Problem {
            len,
            n_conds,
            conds,
        }
    }
    fn solve(&self) -> Answer {
        // 方針
        // C で group_by して C の小さい方から P_A と P_B を決定していく。

        use ac_library::ModInt998244353 as Mint;
        // c で group_by してソートする
        let conds = self.conds.iter().copied().into_group_map_by(|x| x.c);
        let comb = Comb::new(self.len);
        let mut cnt_used = 0;
        let mut visited = vec![false; self.len];

        // 辺に現れる頂点
        let cnt1 = conds
            .iter()
            .sorted_by_key(|(c, _)| **c)
            .map(|(c, sub_conds)| {
                // sub_conds でグラフを構成したとき、そのグラフがスターになっているケースを考える。
                // スターのルート（中心）に c を割り当てる。
                // star_root は基本1種類だが、sub_conds.len() == 1 のときは2種類になることもある。
                let star_root_cand = [sub_conds[0].a, sub_conds[0].b]
                    .into_iter()
                    .filter(|&v| sub_conds.iter().copied().all(|x| x.a == v || x.b == v))
                    .collect_vec();

                let sub_ans = star_root_cand
                    .iter()
                    .copied()
                    .filter(|&star_root| !visited[star_root])
                    .map(|_| {
                        let cnt_unvisited = sub_conds
                            .iter()
                            .copied()
                            .flat_map(|cond| [cond.a, cond.b])
                            .unique()
                            .filter(|&x| !visited[x])
                            .count();

                        if *c < cnt_used || cnt_unvisited == 0 {
                            Mint::new(0)
                        } else {
                            comb.perm(c - cnt_used, cnt_unvisited - 1)
                        }
                    })
                    .sum::<Mint>();

                for &cond in sub_conds {
                    if !visited[cond.a] {
                        visited[cond.a] = true;
                        cnt_used += 1;
                    }
                    if !visited[cond.b] {
                        visited[cond.b] = true;
                        cnt_used += 1;
                    }
                }

                sub_ans
            })
            .product::<Mint>();
        // 辺に現れない頂点
        let cnt2 = comb.factorial(self.len - cnt_used);
        let ans = (cnt1 * cnt2).val() as i64;
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
    Problem::read().solve().print();
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

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
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
    }
}
