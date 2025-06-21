fn main() {
    input! {
        n: usize,
        q: usize,
        xs: [Usize1; q],
    }

    // false: 白, true: 黒
    let mut board = vec![false; n];

    let white = false;
    let black = true;

    let mut ans: i64 = 0; // 連続した黒の数

    for x in xs {
        if board[x] == white {
            let next_cnt_black = {
                let cnt1 = (x != 0 && board[x - 1] == black) as i64;
                let cnt2 = (x != n - 1 && board[x + 1] == black) as i64;
                cnt1 + cnt2
            };
            match next_cnt_black {
                0 => {
                    ans += 1;
                }
                1 => {}
                2 => {
                    ans -= 1;
                }
                _ => panic!(),
            }

            board[x] = black;
        } else if board[x] == black {
            let next_cnt_black = {
                let cnt1 = (x != 0 && board[x - 1] == black) as i64;
                let cnt2 = (x != n - 1 && board[x + 1] == black) as i64;
                cnt1 + cnt2
            };

            match next_cnt_black {
                0 => {
                    ans -= 1;
                }
                1 => {}
                2 => {
                    ans += 1;
                }
                _ => panic!(),
            }
            board[x] = white;
        }

        println!("{}", ans);
    }
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
