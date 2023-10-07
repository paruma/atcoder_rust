struct Problem {
    n_players: usize,
    n_problems: usize,
    problem_point_list: Vec<i64>,
    table: Vec<Vec<u8>>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_players: usize,
            n_problems: usize,
            problem_point_list: [i64; n_problems],
            table: [Bytes; n_players],
        }
        Problem { n_players, n_problems, problem_point_list, table }
    }
    fn solve(&self) -> Answer {
        let Problem { n_players, n_problems, problem_point_list, table } = self;
        // 各プレイヤーの点数を計算しておく
        let player_point_list = (0..*n_players)
            .map(|player_i| {
                table[player_i]
                    .iter()
                    .enumerate()
                    .filter_map(|(problem_i, ch)| {
                        (*ch == b'o').then_some(problem_point_list[problem_i] + player_i as i64 + 1)
                    })
                    .sum::<i64>()
            })
            .collect_vec();

        let problems_point_list_sorted = problem_point_list
            .iter()
            .copied()
            .enumerate()
            .sorted_by_key(|(_i, p)| Reverse(*p))
            .collect_vec();

        let ans = (0..*n_players)
            .map(|i| {
                // 何点必要か計算する
                let max_point_without_me = (0..*n_players)
                    .filter(|&j| j != i)
                    .map(|j| player_point_list[j])
                    .max()
                    .unwrap();
                let need_point = max(0, max_point_without_me - player_point_list[i] + 1);
                if need_point <= 0 {
                    return 0;
                }

                // 何問解けばいいですか？
                let mut cnt = 0;
                let mut addtional_point = 0;

                for &(problem_idx, point) in &problems_point_list_sorted {
                    if table[i][problem_idx] == b'x' {
                        cnt += 1;
                        addtional_point += point;
                        if addtional_point >= need_point {
                            return cnt;
                        }
                    }
                }
                panic!();

                // return cnt;
            })
            .collect_vec();

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
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
        assert_eq!(1 + 1, 2);
    }
}

use std::cmp::{max, Reverse};

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
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
