//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xss: Vec<Vec<char>>,
}

fn print(xss: &Vec<Vec<char>>) {
    for l in xss {
        print_chars(l);
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xss: [Chars; n],
        }
        Problem { n, xss }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let xss = &self.xss;
        let xss0 = xss.clone();
        let xss1 = rotate_right(&xss0);
        let xss2 = rotate_right(&xss1);
        let xss3 = rotate_right(&xss2);

        let xss_rotates = [&xss0, &xss1, &xss2, &xss3];

        let mut ans = vec![vec!['o'; n]; n];

        for i in 0..n / 2 {
            let xss_r = xss_rotates[(i + 1) % 4];
            for x in i..n - i {
                let y1 = i;
                ans[y1][x] = xss_r[y1][x];

                let y2 = n - i - 1;
                ans[y2][x] = xss_r[y2][x];
            }

            for y in i..n - i {
                let x1 = i;
                ans[y][x1] = xss_r[y][x1];

                let x2 = n - i - 1;
                ans[y][x2] = xss_r[y][x2];
            }
        }

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
    ans: Vec<Vec<char>>,
}

impl Answer {
    fn print(&self) {
        for l in &self.ans {
            print_chars(l);
        }
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
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
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
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
use array_2d_transformation::*;
#[allow(clippy::module_inception)]
pub mod array_2d_transformation {
    pub fn rotate_right<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[x][h - 1 - y] = *v;
            }
        }
        table_after
    }
    pub fn rotate_left<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[w - 1 - x][y] = *v;
            }
        }
        table_after
    }
    pub fn rotate_180_deg<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[h - 1 - y][w - 1 - x] = *v;
            }
        }
        table_after
    }
    pub fn transpose<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[x][y] = *v;
            }
        }
        table_after
    }
    pub fn reflect_x_axis<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[h - 1 - y][x] = *v;
            }
        }
        table_after
    }
    pub fn reflect_y_axis<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[y][w - 1 - x] = *v;
            }
        }
        table_after
    }
}
