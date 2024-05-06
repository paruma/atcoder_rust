//#[derive_readable]
#[derive(Debug)]
struct Problem {
    xss: [[i64; 3]; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Board {
    board: [[u8; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            board: [[b'W'; 3]; 3],
        }
    }

    // 縦・横・斜めのいずれかが揃っている
    fn has_line(&self, color: u8) -> bool {
        (0..3).any(|x| (0..3).all(|y| self.board[y][x] == color))
            || (0..3).any(|y| (0..3).all(|x| self.board[y][x] == color))
            || (0..3).all(|i| self.board[i][i] == color)
            || (0..3).all(|i| self.board[i][2 - i] == color)
    }

    fn count_non_white(&self) -> usize {
        iproduct!(0..3, 0..3)
            .filter(|(x, y)| self.board[*y][*x] != b'W')
            .count()
    }

    fn has_white(&self) -> bool {
        iproduct!(0..3, 0..3).any(|(x, y)| self.board[y][x] == b'W')
    }

    fn calc_point(&self, point_map: &[[i64; 3]; 3], color: u8) -> i64 {
        iproduct!(0..3, 0..3)
            .map(|(x, y)| point_map[y][x] * (self.board[y][x] == color) as i64)
            .sum()
    }

    fn judge(&self, point_map: &[[i64; 3]; 3]) -> u8 {
        // W: 白（決着ついてない）
        // R: 赤（高橋くん勝ち）
        // B: 青（青木くん勝ち）

        if self.has_line(b'R') {
            b'R'
        } else if self.has_line(b'B') {
            b'B'
        } else if self.has_white() {
            return b'W';
        } else if self.calc_point(point_map, b'R') > self.calc_point(point_map, b'B') {
            b'R'
        } else {
            b'B'
        }
    }

    fn at(&self, x: usize, y: usize) -> u8 {
        self.board[y][x]
    }

    fn write(&self, x: usize, y: usize, color: u8) -> Self {
        let mut board = self.board;
        board[y][x] = color;
        Board { board }
    }

    fn to_pretty_string(&self) -> String {
        let mut ans = String::new();
        for y in 0..3 {
            for x in 0..3 {
                ans.push(self.board[y][x] as char);
            }
            ans.push('\n');
        }
        ans
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            xss: [[i64;3]; 3],
        }
        let mut xss_arr = [[0; 3]; 3];
        for y in 0..3 {
            for x in 0..3 {
                xss_arr[y][x] = xss[y][x];
            }
        }
        Problem { xss: xss_arr }
    }
    fn solve(&self) -> Answer {
        // W: 白
        // R: 赤 (高橋くん)
        // B: 青 (青木くん)

        struct Rec {
            point_map: [[i64; 3]; 3],
        }

        impl Rec {
            fn new(point_map: [[i64; 3]; 3]) -> Self {
                Self { point_map }
            }

            // 最適行動をしたときに勝つ方を求める
            fn rec(&self, board: &Board) -> u8 {
                let judge_result = board.judge(&self.point_map);
                if judge_result != b'W' {
                    return judge_result;
                }

                let tern_cnt = board.count_non_white();
                let player = [b'R', b'B'][tern_cnt % 2];

                let can_win = iproduct!(0..3, 0..3)
                    .filter(|(y, x)| board.at(*x, *y) == b'W')
                    .map(|(y, x)| {
                        let next_board = board.write(x, y, player);
                        self.rec(&next_board)
                    })
                    .any(|c| c == player);

                if can_win {
                    player
                } else {
                    match player {
                        b'R' => b'B',
                        b'B' => b'R',
                        _ => panic!(),
                    }
                }
            }
        }
        let ans = Rec::new(self.xss).rec(&Board::new());
        let ans = match ans {
            b'R' => "Takahashi",
            b'B' => "Aoki",
            _ => panic!(),
        };
        let ans = ans.to_string();
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
    ans: String,
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
