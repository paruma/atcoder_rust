#[derive(Clone, Debug, PartialEq, Eq)]
enum Query {
    ServerToPc { p: usize },
    Append { p: usize, s: Vec<char> }, // これ大丈夫？作って MLE とかしない？
    PcToServer { p: usize },
}
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let qs = (0..q)
        .map(|_| {
            input! {
                t: usize
            }
            if t == 1 {
                input! {
                    p: Usize1,
                }
                Query::ServerToPc { p }
            } else if t == 2 {
                input! {
                    p: Usize1,
                    s: Chars,
                }
                Query::Append { p, s }
            } else {
                input! {
                    p: Usize1,
                }
                Query::PcToServer { p }
            }
        })
        .collect_vec();

    // server は n 番目の PC として扱う
    // k 個クエリを見た状態
    fn rec(k: usize, p: usize, n: usize, qs: &[Query]) -> Vec<char> {
        if k == 0 {
            return vec![];
        }
        let query = &qs[k - 1];
        match query {
            Query::ServerToPc { p: qp } => {
                if p == *qp {
                    rec(k - 1, n, n, qs)
                } else {
                    rec(k - 1, p, n, qs)
                }
            }
            Query::Append { p: qp, s } => {
                if p == *qp {
                    let mut prev = rec(k - 1, p, n, qs);
                    prev.append(&mut s.clone());
                    prev
                } else {
                    rec(k - 1, p, n, qs)
                }
            }
            Query::PcToServer { p: qp } => {
                // server の番号
                if p == n {
                    rec(k - 1, *qp, n, qs)
                } else {
                    rec(k - 1, p, n, qs)
                }
            }
        }
    }

    let ans: Vec<char> = rec(q, n, n, &qs);
    print_chars(&ans);
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
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
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
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
