use std::io::stdin;

struct Problem {
    n_people: usize,
    te_list: Vec<u8>,
    nq: usize,
    qs: Vec<Query>,
}

struct Query {
    left: usize,
    right: usize, // [left, right]
}

// 勝った人を返す（引き分けはD）としてもよかった
fn janken(mine: u8, yours: u8) -> i64 {
    // 勝ち: 1
    // 引き分け:0
    // 負け: -1

    if mine == yours {
        return 0;
    } else if mine == b'G' {
        return if yours == b'C' { 1 } else { -1 };
    } else if mine == b'C' {
        return if yours == b'P' { 1 } else { -1 };
    } else if mine == b'P' {
        return if yours == b'G' { 1 } else { -1 };
    }
    unreachable!()
}

#[allow(clippy::collapsible_else_if)]
fn ooninzu_janken(cnt_g: i64, cnt_c: i64, cnt_p: i64) -> Option<u8> {
    let cnt_sorted =
        izip!(b"GCP", [cnt_g, cnt_c, cnt_p]).sorted_by_key(|(_, cnt)| *cnt).collect_vec();
    if cnt_sorted[0].1 == 0 && cnt_sorted[1].1 == 0 {
        return Some(*cnt_sorted[2].0);
    }
    if cnt_sorted[0].1 == 0 {
        if cnt_sorted[1].1 < cnt_sorted[2].1 {
            return Some(*cnt_sorted[1].0);
        } else {
            if janken(*cnt_sorted[1].0, *cnt_sorted[2].0) == 1 {
                return Some(*cnt_sorted[1].0);
            } else {
                return Some(*cnt_sorted[2].0);
            }
        }
    }
    if cnt_sorted[0].1 == cnt_sorted[1].1 && cnt_sorted[1].1 == cnt_sorted[2].1 {
        return None;
    } else if cnt_sorted[0].1 == cnt_sorted[1].1 {
        if janken(*cnt_sorted[0].0, *cnt_sorted[1].0) == 1 {
            return Some(*cnt_sorted[0].0);
        } else {
            return Some(*cnt_sorted[1].0);
        }
    } else {
        return Some(*cnt_sorted[0].0);
    }
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let n_people = r.read_usize_1();
        let te_list = r.read_bytes().iter().filter(|ch| **ch != b' ').copied().collect_vec();
        let nq = r.read_usize_1();
        let qs = (0..nq)
            .map(|_| {
                let (left, right) = r.read_usize_2();
                Query { left: left - 1, right: right - 1 }
            })
            .collect_vec();
        Problem { n_people, te_list, nq, qs }
    }
    fn solve(&self) -> Answer {
        let Problem { n_people, te_list, nq, qs } = self;
        let ind_g = te_list.iter().map(|ch| if *ch == b'G' { 1 } else { 0 }).collect_vec();
        let ind_c = te_list.iter().map(|ch| if *ch == b'C' { 1 } else { 0 }).collect_vec();
        let ind_p = te_list.iter().map(|ch| if *ch == b'P' { 1 } else { 0 }).collect_vec();
        let cumsum_g = CumSum::new(&ind_g);
        let cumsum_c = CumSum::new(&ind_c);
        let cumsum_p = CumSum::new(&ind_p);
        let x = false;

        let ans = qs
            .iter()
            .map(|q| {
                // 勝てる手のリストを返す
                let cnt_g = cumsum_g.get_interval_sum(q.left, q.right + 1);
                let cnt_c = cumsum_c.get_interval_sum(q.left, q.right + 1);
                let cnt_p = cumsum_p.get_interval_sum(q.left, q.right + 1);

                // グーチョキパー全部検証する
                // 最小値が0
                b"GCP"
                    .iter()
                    .filter(move |te| match **te {
                        b'G' => ooninzu_janken(cnt_g + 1, cnt_c, cnt_p) == Some(b'G'),
                        b'C' => ooninzu_janken(cnt_g, cnt_c + 1, cnt_p) == Some(b'C'),
                        b'P' => ooninzu_janken(cnt_g, cnt_c, cnt_p + 1) == Some(b'P'),
                        _ => panic!(),
                    })
                    .copied()
                    .collect_vec()
            })
            .collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<Vec<u8>>,
}

impl Answer {
    fn print(&self) {
        for row in &self.ans {
            if row.is_empty() {
                println!("{}", -1);
            } else {
                println!("{}", row.iter().map(|ch| *ch as char).join(" "));
            }
        }
    }
}

fn main() {
    Problem::read(ProconReader::new(stdin().lock())).solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(ProconReader::new(input.as_bytes())).solve();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_problem() {
        let _input = "
3
4
        "
        .trim();
        // check(_input, Answer { ans: 7 });
    }
}

// ====== snippet ======

pub mod cumsum {
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }

    impl CumSum {
        pub fn new(xs: &Vec<i64>) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }

        pub fn get_interval_sum(&self, begin: usize, end: usize) -> i64 {
            // [begin, end) の間で総和を求める
            self.cumsum[end] - self.cumsum[begin]
        }
    }
}

use cumsum::CumSum;
use itertools::{izip, zip, Itertools};
#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io::BufRead;

    pub trait IProconReader {
        fn read_line(&mut self) -> String;

        fn read_bytes(&mut self) -> Vec<u8> {
            self.read_line().as_bytes().to_vec()
        }

        fn read_any_1<T>(&mut self) -> T
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.parse::<T>().unwrap()
        }

        fn read_any_2<T0, T1>(&mut self) -> (T0, T1)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            (a0, a1)
        }

        fn read_any_3<T0, T1, T2>(&mut self) -> (T0, T1, T2)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
            T2: std::str::FromStr,
            T2::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            let a2 = splitted[2].parse::<T2>().unwrap();
            (a0, a1, a2)
        }

        fn read_any_4<T0, T1, T2, T3>(&mut self) -> (T0, T1, T2, T3)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
            T2: std::str::FromStr,
            T2::Err: std::fmt::Debug,
            T3: std::str::FromStr,
            T3::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            let a2 = splitted[2].parse::<T2>().unwrap();
            let a3 = splitted[3].parse::<T3>().unwrap();
            (a0, a1, a2, a3)
        }
        fn read_vec_any<T>(&mut self) -> Vec<T>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.trim().split(' ').map(|s| s.parse::<T>().unwrap()).collect::<Vec<T>>()
        }

        fn read_vec_i64(&mut self) -> Vec<i64> {
            self.read_vec_any::<i64>()
        }

        fn read_vec_usize(&mut self) -> Vec<usize> {
            self.read_vec_any::<usize>()
        }

        fn read_vec_str(&mut self) -> Vec<String> {
            self.read_vec_any::<String>()
        }

        fn read_i64_1(&mut self) -> i64 {
            self.read_any_1::<i64>()
        }

        fn read_i64_2(&mut self) -> (i64, i64) {
            self.read_any_2::<i64, i64>()
        }

        fn read_i64_3(&mut self) -> (i64, i64, i64) {
            self.read_any_3::<i64, i64, i64>()
        }

        fn read_i64_4(&mut self) -> (i64, i64, i64, i64) {
            self.read_any_4::<i64, i64, i64, i64>()
        }

        fn read_usize_1(&mut self) -> usize {
            self.read_any_1::<usize>()
        }

        fn read_usize_2(&mut self) -> (usize, usize) {
            self.read_any_2::<usize, usize>()
        }

        fn read_usize_3(&mut self) -> (usize, usize, usize) {
            self.read_any_3::<usize, usize, usize>()
        }

        fn read_usize_4(&mut self) -> (usize, usize, usize, usize) {
            self.read_any_4::<usize, usize, usize, usize>()
        }
    }

    pub struct ProconReader<R: BufRead> {
        buf_read: R,
    }

    impl<R: BufRead> ProconReader<R> {
        pub fn new(buf_read: R) -> ProconReader<R> {
            ProconReader { buf_read }
        }
    }

    impl<R: BufRead> IProconReader for ProconReader<R> {
        fn read_line(&mut self) -> String {
            let mut buffer = String::new();
            self.buf_read.read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        }
    }
}
