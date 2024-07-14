//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    s: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            s: Bytes,
        }
        Problem { s }
    }
    fn solve(&self) -> Answer {
        /*
        s の相異なる部分文字列の数を求める問題
        s="aaab" の場合
        a0 a1 a2 b3 という形で添え字をつける
        sの suffix をソートすると以下のようになる
        a0 a1 a2 b3
        a1 a2 b3
        a2 b3
        b3
        [a0, a1] と [a1, a2] や [a0] と [a1] のダブルカウントを防ぐために
        [a0, a1, a2, b3] と [a1, a2, b3] の LCP長 を引くというのを繰り返すと、
        相異なる部分文字列の数が得られる。
         */
        let n = self.s.len();
        let sa = suffix_array_arbitrary(&self.s);
        let lcp = lcp_array_arbitrary(&self.s, &sa);

        let ans = n * (n + 1) / 2 - lcp.iter().sum::<usize>();
        let ans = ans as i64;
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
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

use ac_library::{lcp_array_arbitrary, suffix_array_arbitrary};
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
