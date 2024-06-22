//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [usize; n],
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        // 解法: 余事象を使う(三角不等式を満たさない場合の数を求める)
        // R + G + B = S を満たす (R, G, B) の図を3次元座標で表現するすると三角形になる
        // さらに(R, G, B) を3辺とする三角形ができるような(R, G, B)を図にすると、
        // ゼルダのトライフォースみたいな感じになる。
        // ゼルダのトライフォースで余事象（包除原理）をする

        let n = self.n;
        let xs = &self.xs;
        let sum = xs.iter().sum::<usize>();
        use ac_library::ModInt998244353 as Mint;

        // dp1[i][x] = xs[0..i] の各成分を赤・緑・青で塗ったときに、赤で塗った成分の和がxになる場合の数
        let dp1 = {
            let mut dp1 = vec![vec![Mint::new(0); sum + 1]; n + 1];

            dp1[0][0] = Mint::new(1);

            for i in 0..n {
                for s in 0..=sum {
                    let from_my_color = if s >= xs[i] {
                        dp1[i][s - xs[i]]
                    } else {
                        Mint::new(0)
                    };
                    let from_other_color = dp1[i][s] * 2;
                    dp1[i + 1][s] = from_my_color + from_other_color;
                }
            }
            dp1
        };

        // dp2[i][x] = xs[0..i] の各成分を赤・緑で塗ったときに、赤で塗った成分の和がxになる場合の数
        let dp2 = {
            let mut dp2 = vec![vec![Mint::new(0); sum + 1]; n + 1];

            dp2[0][0] = Mint::new(1);

            for i in 0..n {
                for s in 0..=sum {
                    let from_my_color = if s >= xs[i] {
                        dp2[i][s - xs[i]]
                    } else {
                        Mint::new(0)
                    };
                    let from_other_color = dp2[i][s];
                    dp2[i + 1][s] = from_my_color + from_other_color;
                }
            }
            dp2
        };
        // 3^n - 3 * sum(dp1[n][⌈S/2⌉..S]) (S が偶数の場合は重複分を除く)

        let ans = Mint::new(3).pow(n as u64)
            - dp1[n][num::integer::div_ceil(sum, 2)..]
                .iter()
                .sum::<Mint>()
                * 3
            + if sum % 2 == 0 {
                dp2[n][sum / 2] * 3
            } else {
                Mint::new(0)
            };
        let ans = ans.val() as i64;
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
        // let n = rng.gen_range(1..=10);
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
        // let mut rng = SmallRng::from_entropy();
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
use num::Integer;
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
