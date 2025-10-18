#[derive_readable]
#[derive(Debug)]
struct Problem {
    s: Bytes,
    t: Bytes,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        // コンテスト中に書いた実装。t[0], t[1], t[2] が現れる場所を調べる。
        let s = &self.s;
        let t = &self.t; // 長さ3

        let pos_t0 = s
            .iter()
            .copied()
            .position(|ch| ch == t[0].to_ascii_lowercase());
        if pos_t0.is_none() {
            return Answer { ans: false };
        }
        let pos_t0 = pos_t0.unwrap();
        if pos_t0 + 1 >= s.len() {
            return Answer { ans: false };
        }
        let pos_t1 = s[pos_t0 + 1..]
            .iter()
            .copied()
            .position(|ch| ch == t[1].to_ascii_lowercase())
            .map(|x| x + pos_t0 + 1);
        if pos_t1.is_none() {
            return Answer { ans: false };
        }
        let pos_t1 = pos_t1.unwrap();
        if t[2] == b'X' {
            return Answer { ans: true };
        }
        if pos_t1 + 1 >= s.len() {
            return Answer { ans: false };
        }
        let pos_t2 = s[pos_t1 + 1..]
            .iter()
            .copied()
            .position(|ch| ch == t[2].to_ascii_lowercase());

        let ans = pos_t2.is_some();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // solve のリファクタリング
        let s = &self.s;
        let t = &self.t.to_ascii_lowercase(); // 長さ3

        // 末尾が x の場合は x を除いた状態で考える
        let t_len = if t[2] == b'x' { 2 } else { 3 };
        let mut current_s = s.as_slice(); // 見た分は消していく。
        for i in 0..t_len {
            let pos = current_s.iter().copied().position(|ch| ch == t[i]);
            if pos.is_none() {
                return Answer { ans: false };
            }
            let pos = pos.unwrap();
            current_s = &current_s[pos + 1..];
        }

        Answer { ans: true }
    }

    fn solve3(&self) -> Answer {
        // solve2 のリファクタリング
        // T ではなく S を 1 ずつ動かしていく
        let s = &self.s;
        let t = &self.t.to_ascii_lowercase(); // 長さ3

        // 末尾が x の場合は x を除いた状態で考える
        let t_len = if t[2] == b'x' { 2 } else { 3 };

        let mut t_itr = t[0..t_len].iter().copied().peekable();

        for si in s {
            if t_itr.peek() == Some(si) {
                t_itr.next();
            }
        }
        let ans = t_itr.peek().is_none();

        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // 正規表現を使ったもの
        let s = String::from_utf8(self.s.clone()).unwrap();
        let t = &self
            .t
            .iter()
            .copied()
            .map(|x| x.to_ascii_lowercase() as char)
            .collect_vec();

        let ans = if t[2] == 'x' {
            let pattern = format!("^.*{}.*{}.*$", t[0], t[1]);
            let re = Regex::new(&pattern).unwrap();
            re.is_match(&s)
        } else {
            let pattern = format!("^.*{}.*{}.*{}.*$", t[0], t[1], t[2]);
            let re = Regex::new(&pattern).unwrap();
            re.is_match(&s)
        };

        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let s = &self.s;
        let t = &self
            .t
            .iter()
            .copied()
            .map(|x| x.to_ascii_lowercase())
            .collect_vec(); // 長さ3

        // t.to_ascii_lowercase(); がある

        //空港コードのリスト
        let set = {
            let list1 = s.iter().copied().combinations(3).collect_vec();
            let list2 = s
                .iter()
                .copied()
                .combinations(2)
                .map(|mut xs| {
                    xs.push(b'x');
                    xs
                })
                .collect_vec();

            chain!(list1, list2).collect::<HashSet<_>>()
        };
        let ans = set.contains(t);
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
    Problem::read().solve3().print();
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
        assert_eq!(p.solve2(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        let mut rng = SmallRng::from_os_rng();
        let s = (0..10).map(|_| rng.random_range(b'a'..=b'z')).collect_vec();
        let t = (0..3).map(|_| rng.random_range(b'A'..=b'Z')).collect_vec();
        println!("{:?}", String::from_utf8(s.clone()).unwrap());
        println!("{:?}", String::from_utf8(t.clone()).unwrap());
        let p = Problem { s, t };
        p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..10000 {
            let p = make_random_problem();
            check(&p);
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
use regex::Regex;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::format;

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
