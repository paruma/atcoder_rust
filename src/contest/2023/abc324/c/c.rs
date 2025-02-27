//#[derive_readable]
struct Problem {
    recieved: Vec<u8>,
    cand_list: Vec<Vec<u8>>,
}

fn is_dist_leq1(str1: &[u8], str2: &[u8]) -> bool {
    if str1 == str2 {
        return true;
    }
    if usize::abs_diff(str1.len(), str2.len()) >= 2 {
        return false;
    }

    if str1.len() == str2.len() {
        return izip!(str1, str2).filter(|(c1, c2)| c1 != c2).count() <= 1;
    }

    if str1.len() == str2.len() + 1 {
        // str1 のほうが1長い
        let mut iter1 = str1.iter().peekable();
        let mut iter2 = str2.iter().peekable();

        let mut diff_cnt = 0;

        while iter1.peek().is_some() {
            if iter1.peek() == iter2.peek() {
                iter1.next();
                iter2.next();
            } else {
                iter1.next();
                diff_cnt += 1;
            }
        }

        return diff_cnt <= 1;

        // abacbc
        // aba bc

        // abc
        //  bc

        // abc
        // ab
    }
    if str1.len() + 1 == str2.len() {
        return is_dist_leq1(str2, str1);
    }

    panic!()
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
            recieved: Bytes,
            cand_list: [Bytes; n],
        }
        Problem { recieved, cand_list }
    }
    fn solve(&self) -> Answer {
        let ans = self
            .cand_list
            .iter()
            .enumerate()
            .filter(|(_i, cand)| is_dist_leq1(cand, &self.recieved))
            .map(|(i, _cand)| i + 1) //1オリジンにする
            .collect_vec();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // enumerate().filter(...).map(...) は positions で書ける
        let ans = self
            .cand_list
            .iter()
            .positions(|cand| is_dist_leq1(cand, &self.recieved))
            .map(|i| i + 1) //1オリジンにする
            .collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans.len());
        print_vec_1line(&self.ans);
    }
}

fn main() {
    Problem::read().solve2().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        // abacbc
        // aba bc

        // abc
        //  bc

        // abc
        // ab
        assert!(is_dist_leq1(b"abacbc", b"ababc"));
        assert!(is_dist_leq1(b"abc", b"bc"));
        assert!(is_dist_leq1(b"abc", b"ab"));
        assert!(is_dist_leq1(b"ab", b"abc"));
        assert!(is_dist_leq1(b"", b""));
        assert!(!is_dist_leq1(b"abc", b"c"));
        assert!(is_dist_leq1(b"abc", b"ac"));
        assert!(!is_dist_leq1(b"abc", b"ad"));

        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
use itertools::izip;
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
