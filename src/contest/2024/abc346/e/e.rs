#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    t: i64, // 1 or 2
    pos: Usize1,
    color: usize,
}
struct Problem {
    h: usize,
    w: usize,
    nq: usize,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            nq: usize,
            qs: [Query; nq],
        }
        Problem { h, w, nq, qs }
    }
    fn solve(&self) -> Answer {
        let h = self.h;
        let w = self.w;

        let mut cnt_col_list: Vec<i64> = vec![0_i64; w];
        let mut cnt_col_sum = 0; // cnt_col_list で 非ゼロの数
        let mut cnt_row_list: Vec<i64> = vec![0_i64; h];
        let mut cnt_row_sum = 0;

        let color_max = self.qs.iter().map(|q| q.color).max().unwrap();

        let mut ans = vec![0_i64; color_max + 1];
        ans[0] = (w as i64) * (h as i64);

        for q in self.qs.iter().rev() {
            if q.t == 1 {
                // 行

                if cnt_row_list[q.pos] == 0 && q.color != 0 {
                    let addition = (w as i64) - cnt_col_sum;
                    ans[q.color] += addition;
                    ans[0] -= addition;
                }
                if cnt_row_list[q.pos] == 0 {
                    cnt_row_sum += 1;
                }
                cnt_row_list[q.pos] += 1;
            } else if q.t == 2 {
                //列
                if cnt_col_list[q.pos] == 0 && q.color != 0 {
                    let addition = (h as i64) - cnt_row_sum;
                    ans[q.color] += addition;
                    ans[0] -= addition;
                }
                if cnt_col_list[q.pos] == 0 {
                    cnt_col_sum += 1;
                }
                cnt_col_list[q.pos] += 1;
            }
            // dbg!(&cnt_col_list);
            // dbg!(&cnt_row_list);
            // dbg!(&cnt_col_sum);
            // dbg!(&cnt_row_sum);
        }

        let ans = ans
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, x)| *x > 0)
            .collect_vec();

        Answer { ans }
    }
    fn solve_naive(&self) -> Answer {
        let mut grid = vec![vec![0; self.w]; self.h];
        for &q in &self.qs {
            if q.t == 1 {
                // 行
                for x in 0..self.w {
                    grid[q.pos][x] = q.color;
                }
            } else if q.t == 2 {
                //列
                for y in 0..self.h {
                    grid[y][q.pos] = q.color;
                }
            }
        }

        let color_max = self.qs.iter().map(|q| q.color).max().unwrap();
        let mut ans = vec![0_i64; color_max + 1];

        for y in 0..self.h {
            for x in 0..self.w {
                ans[grid[y][x]] += 1;
            }
        }

        let ans = ans
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, x)| *x > 0)
            .collect_vec();

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<(usize, i64)>,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans.len());
        for (c, x) in self.ans.iter() {
            println!("{} {}", c, x);
        }
    }
}

fn main() {
    let p = Problem::read();
    p.solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
use num_traits::MulAddAssign;
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
