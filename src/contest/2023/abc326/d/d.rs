/*
解法
N=5の場合、1行あたり候補が20通りしかないことが実験からわかる。あとは頑張って全探索する。
20通りを計算で求めるとこう。

`ABC..` の並べ替えは 5P3 = 60 通り
このうち、.以外の最左文字がAである場合は全体の1/3である(BやCも同様。)
よって60/3=20通りと求まる。

[実装途中のバグ]
n とするところを 3としてしまった。

[その他]
check_line の cnts の計算で Itertools の counts を使ったら TLE になった。
Vec を使って実装をしたら AC した（HashMap を使っているから定数倍で遅いっぽい。）
*/

//#[derive_readable]
struct Problem {
    n: usize,
    row: Vec<u8>,
    col: Vec<u8>,
}

// xs に A, B, C がちょうど1回ずつ現れる && . 以外で最初に現れる文字が head
fn check_line(xs: &[u8], head: u8) -> bool {
    let cnts = {
        let mut cnts = vec![0; 3];
        for ch in xs.iter().copied().filter(|ch| *ch != b'.') {
            cnts[ch as usize - b'A' as usize] += 1;
        }
        cnts
    };
    if cnts.iter().copied().any(|x| x != 1) {
        return false;
    }

    let head_pos = xs.iter().position(|&x| x == head).unwrap();
    let other_pos = xs.iter().position(|&x| ![head, b'.'].contains(&x)).unwrap();
    head_pos < other_pos
}

// ABCが1回ずつ現れる確認は省略する
fn check_line_small(xs: &[u8], head: u8) -> bool {
    let head_pos = xs.iter().position(|&x| x == head).unwrap();
    let other_pos = xs.iter().position(|&x| ![head, b'.'].contains(&x)).unwrap();
    head_pos < other_pos
}

struct Board {
    board: Vec<Vec<u8>>,
}

impl Board {
    fn new(board: &[Vec<u8>]) -> Board {
        Board {
            board: board.to_vec(),
        }
    }

    fn size(&self) -> usize {
        self.board.len()
    }

    fn row(&self, y: usize) -> Vec<u8> {
        self.board[y].clone()
    }

    fn col(&self, x: usize) -> Vec<u8> {
        (0..self.size()).map(|y| self.board[y][x]).collect_vec()
    }

    fn pretty_string(&self) -> String {
        let mut ans = String::new();
        let n = self.size();
        for y in 0..n {
            for x in 0..n {
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
            n: usize,
            row: Bytes,
            col: Bytes,
        }
        Problem { n, row, col }
    }

    fn judge(&self, board: Board) -> bool {
        (0..self.n).all(|y| check_line_small(&board.row(y), self.row[y]))
            && (0..self.n).all(|x| check_line(&board.col(x), self.col[x]))
    }

    fn solve(&self) -> Answer {
        // let n = self.n;
        let row = &self.row;
        // let col = &self.col;

        // template[0] は 先頭が A でABCが1つずつ現れる1行全体
        let template = b"ABC"
            .iter()
            .copied()
            .map(|ch| {
                std::iter::repeat(b".ABC") // {'.', 'A', 'B', 'C'}^n
                    .take(self.n)
                    .copied()
                    .multi_cartesian_product()
                    .filter(|xs| check_line(xs, ch))
                    .collect_vec()
            })
            .collect_vec();

        let ans = row
            .iter()
            .copied()
            .map(|ch| template[(ch - b'A') as usize].to_vec()) // 各行の候補 (N=5 の場合は20個)
            .multi_cartesian_product() // 1行目の候補×2行目の候補×... という直積を取る
            .find(|rows| {
                let board = Board::new(rows);
                self.judge(board)
            });

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<Vec<Vec<u8>>>,
}

impl Answer {
    fn print(&self) {
        if let Some(board) = &self.ans {
            println!("Yes");
            for row in board {
                println!("{}", std::str::from_utf8(row).unwrap());
            }
        } else {
            println!("No");
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

    fn check(xs: &[u8], head: u8) -> bool {
        let cnts = xs.iter().copied().counts();
        if !b"ABC"
            .iter()
            .copied()
            .all(|ch| cnts.get(&ch).unwrap_or(&0) == &1)
        {
            return false;
        }

        let head_pos = xs.iter().position(|&x| x == head).unwrap();
        let other_pos = xs.iter().position(|&x| ![head, b'.'].contains(&x)).unwrap();
        head_pos < other_pos
    }

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
        let cnt = std::iter::repeat(b".ABC")
            .take(5)
            .copied()
            .multi_cartesian_product()
            .filter(|xs| check(xs, b'A'))
            .count();
        //dbg!(cnt);
        let board = [b"AC..B", b".BA.C", b"C.BA.", b"BA.C.", b"..CBA"]
            .iter()
            .copied()
            .map(|xs| xs.to_vec())
            .collect_vec();
        let p = Problem {
            n: 5,
            row: b"ABCBC".to_vec(),
            col: b"ACAAB".to_vec(),
        };

        let board = Board::new(&board);
        assert!(p.judge(board));
    }
}

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
