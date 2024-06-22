//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    m: usize,
    n: usize,
    xs: Vec<usize>,
}

fn to_1idx(xs: &[usize]) -> Vec<usize> {
    xs.iter().copied().map(|x| x + 1).collect_vec()
}

impl Problem {
    fn read() -> Problem {
        input! {
            m: usize,
            n: usize,
            xs: [Usize1; m],
        }
        Problem { m, n, xs }
    }
    fn solve(&self) -> Answer {
        use ac_library::ModInt998244353 as Mint;
        let m = self.m;
        let n = self.n;
        let xs = &self.xs;
        let mut dp = vec![vec![Mint::new(0); 1 << m]; n + 1];
        dp[0][(1 << m) - 1] = Mint::new(1);

        for i in 0..n {
            // n
            for s in 0..(1 << m) {
                // 2^m
                for x in 0..m {
                    // m
                    // x not in s だったら、制約違反
                    if (s >> x) & 1 == 1 {
                        let tmp = dp[i][s];
                        let next_state = (s & !(1 << x)) // m
                            | xs.iter()
                                .copied()
                                .enumerate()
                                .filter(|(_, x0)| *x0 == x)
                                .fold(0, |acc, (i, _)| acc | (1 << i));
                        dp[i + 1][next_state] += tmp;
                    }
                }
            }
        }
        let ans = dp[n].iter().sum::<Mint>();
        let ans = ans.val() as i64;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let m = self.m;
        let n = self.n;
        let xs = &self.xs;

        let ans = std::iter::repeat((0..m).collect_vec())
            .take(n)
            .multi_cartesian_product()
            .filter(|ys| {
                //dbg!(ys);
                for l in 0..n {
                    for r in l + 1..n {
                        if ys[l] != ys[r] {
                            continue;
                        }

                        if !ys[l..=r].contains(&xs[ys[l]]) {
                            return false;
                        }
                    }
                }
                true
                // (0..=n)
                //     .tuple_combinations()
                //     .filter(|(begin, end)| end - begin > 1)
                //     .filter(|(begin, end)| {
                //         dbg!(begin, end);
                //         ys[*begin] == ys[*end - 1]
                //     })
                //     .all(|(begin, end)| ys[begin..end].contains(&xs[begin]))
            })
            .count() as i64;
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
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};
    #[test]
    fn test_problem2() {
        let m = 4;
        let n = 9;
        let xs = [1, 2, 3, 0];
        let ans = std::iter::repeat((0..m).collect_vec())
            .take(n)
            .multi_cartesian_product()
            .filter(|ys| {
                //dbg!(ys);
                for l in 0..n {
                    for r in l + 1..n {
                        if ys[l] != ys[r] {
                            continue;
                        }

                        if !ys[l..=r].contains(&xs[ys[l]]) {
                            return false;
                        }
                    }
                }
                true
            })
            .collect_vec();

        dbg!((0..m)
            .map(|i| ans.iter().filter(|xs| xs[0] == i).count())
            .collect_vec());
        dbg!(ans.len());
    }
    #[test]
    fn test_problem() {
        //dbg!(ys);
        let n = 4;
        let ys = [0, 1, 2, 0];
        let xs = [1, 0, 1];
        dbg!((0..n)
            .tuple_combinations()
            .filter(|(begin, end)| end - begin > 1)
            .filter(|(begin, end)| ys[*begin] == ys[*end - 1])
            .all(|(begin, end)| {
                //
                let ans = ys[begin..end].contains(&xs[begin]);
                if !ans {
                    dbg!(begin);
                    dbg!(end);
                }
                ans
            }));
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
