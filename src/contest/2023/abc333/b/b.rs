#[derive_readable]
struct Problem {
    s: Bytes,
    t: Bytes,
}

fn len(s1: i64, s2: i64) -> i64 {
    // メモ: mod5 で差が {1, 4} か {2, 3} かって見ると見通しが良い
    // 隣り合ってたら1, そうでない場合は2
    if s1 == s2 {
        panic!()
    }
    // mod の引き算はちょっと書きにくい。足し算だと書きやすい。
    if (s1 + 1) % 5 == s2 || (s2 + 1) % 5 == s1 {
        1
    } else {
        2
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }

    fn solve(&self) -> Answer {
        let s = self
            .s
            .iter()
            .copied()
            .map(|ch| (ch - b'A') as i64)
            .collect_vec();
        let t = self
            .t
            .iter()
            .copied()
            .map(|ch| (ch - b'A') as i64)
            .collect_vec();

        let len1 = len(s[0], s[1]);
        let len2 = len(t[0], t[1]);

        let ans = len1 == len2;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        print_yesno(self.ans);
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
