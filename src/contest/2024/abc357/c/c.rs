//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
}

fn rec(board: &mut [Vec<bool>], level: usize, y: usize, x: usize) {
    if level == 0 {
        board[y][x] = true;
        return;
    }

    // level=1 だと 1
    // level=2 だと 3
    // level=3 だと 9
    let sub_size = usize::pow(3, level as u32 - 1);

    for block_y in 0..3 {
        for block_x in 0..3 {
            if block_y == 1 && block_x == 1 {
                continue;
            }
            rec(
                board,
                level - 1,
                y + block_y * sub_size,
                x + block_x * sub_size,
            );
        }
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
        }
        Problem { n }
    }
    fn solve(&self) -> Answer {
        // 再帰を使う
        let n = self.n;
        let size = usize::pow(3, n as u32);
        let mut board = vec![vec![false; size]; size];
        rec(&mut board, n, 0, 0);
        Answer { ans: board }
    }

    fn solve2(&self) -> Answer {
        // 3進数展開 を考える
        let n = self.n;

        // multi_cartesian_product で空積をするとバグるので回避する。
        if n == 0 {
            return Answer {
                ans: vec![vec![true]],
            };
        }

        let ans = (0..n)
            .map(|_| (0..3))
            .multi_cartesian_product()
            .map(|ternary_y| {
                (0..n)
                    .map(|_| (0..3))
                    .multi_cartesian_product()
                    .map(|ternary_x| {
                        let is_white =
                            izip!(&ternary_y, &ternary_x).any(|(y, x)| (*x, *y) == (1, 1));
                        !is_white
                    })
                    .collect_vec()
            })
            .collect_vec();
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // forループでboard を成長させる
        // 約数列挙を素因数分解から求めるときと同じノリ。

        let mut board = vec![vec![true]];
        for _ in 0..self.n {
            // x:  bbb
            // y:  b.b
            // x:  bbb
            // (b: board)
            let x = board
                .iter()
                .map(|r| [r.clone(), r.clone(), r.clone()].concat())
                .collect_vec();
            let y = board
                .iter()
                .map(|r| [r.clone(), vec![false; board.len()], r.clone()].concat())
                .collect_vec();

            board = [x.clone(), y.clone(), x.clone()].concat();
        }
        Answer { ans: board }
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
    ans: Vec<Vec<bool>>,
}

impl Answer {
    fn print(&self) {
        for row in &self.ans {
            let msg = row
                .iter()
                .copied()
                .map(|p| if p { b'#' } else { b'.' })
                .collect_vec();
            print_bytes(&msg);
        }
    }
}

fn main() {
    Problem::read().solve3().print();
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
