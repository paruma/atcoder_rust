use itertools::Itertools;
#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io;

    pub fn read_line() -> String {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().to_string()
    }

    pub fn read_vec_i64() -> Vec<i64> {
        let buf = read_line();
        buf.trim()
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    pub fn read_vec_str() -> Vec<String> {
        let buf = read_line();
        buf.trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn read_i64_1() -> i64 {
        let buf = read_line();
        buf.parse::<i64>().unwrap()
    }

    pub fn read_i64_2() -> (i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1])
    }

    pub fn read_i64_3() -> (i64, i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1], ns[2])
    }

    pub fn read_i64_4() -> (i64, i64, i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1], ns[2], ns[3])
    }
}

use scan_iter::*;
pub mod scan_iter {
    #[derive(Clone)]
    pub struct Scanl<I, B, F> {
        iter: I,
        state: Option<B>,
        f: F,
    }
    impl<I, B, F> Scanl<I, B, F> {
        fn new(iter: I, init: B, f: F) -> Scanl<I, B, F> {
            Scanl {
                iter,
                state: Some(init),
                f,
            }
        }
    }
    impl<I, B, F> Iterator for Scanl<I, B, F>
    where
        B: Clone + Copy,
        I: Iterator,
        F: FnMut(&mut B, I::Item) -> B,
    {
        type Item = B;
        #[inline]
        fn next(&mut self) -> Option<B> {
            let retval = self.state?;
            let a_opt = self.iter.next();
            self.state = self
                .state
                .and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));
            Some(retval)
        }
    }
    pub trait IteratorExtScanLeft: Iterator + Sized {
        fn scanl<B, F>(self, init: B, f: F) -> Scanl<Self, B, F>
        where
            Self: Sized,
            F: FnMut(&mut B, Self::Item) -> B,
        {
            Scanl::new(self, init, f)
        }
    }
    impl<T: Iterator> IteratorExtScanLeft for T {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Medicine {
    len: i64,     //飲む日数
    per_day: i64, // 1日に飲む数
}

struct Problem {
    n: i64, // 薬の数
    k: i64,
    medicines: Vec<Medicine>,
}

struct Answer {
    ans: i64,
}

impl Problem {
    fn read() -> Problem {
        let (n, k) = read_i64_2();
        let medicines = (0..n)
            .map(|_| {
                let (len, per_day) = read_i64_2();
                Medicine { len, per_day }
            })
            .collect_vec();
        Problem { n, k, medicines }
    }
    /*
    len, per_day
    ___ 
    6 3 0 
    4 2 3  ← 3日目以降は5になる
    2 5 5 
    1 9 10
        19
    

    1 9 19
    2 5 10
    4 2 5
    6 3 3
        0
    2日目までは10錠飲む
    4日目までは5錠飲む

    k = 20
    k = 19

    4日目: 5錠
    5日目: 3錠
    6日目: 3錠
    7日目: 0錠

    */

    fn solve(&self) -> Answer {
        // 一旦ソート
        //
        //let sorted_medicine = self.medicines.sort_by_key(|&x| x.len);
        // len の降順
        let sorted_medicine = self
            .medicines
            .iter()
            .sorted_by_key(|x| -x.len)
            .collect_vec();
        let cumsum = sorted_medicine
            .iter()
            .map(|x| x.per_day)
            .scanl(0, |acc, x| *acc + x)
            .collect_vec();
        let find_result = cumsum.iter().enumerate().find(|(_i, sum)| **sum > self.k);

        let ans = match find_result {
            Some((i, _)) => sorted_medicine[i - 1].len + 1,
            None => 1,
        };
        Answer { ans }
    }
}

impl Answer {
    fn print(self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve0(k: i64) -> i64 {
        let p = Problem {
            n: 6,
            k,
            medicines: [(6, 3), (2, 5), (1, 9), (4, 2)]
                .map(|(len, per_day)| Medicine { len, per_day })
                .to_vec(),
        };
        p.solve().ans
    }

    #[test]
    fn test() {
        assert_eq!(solve0(19), 1);
        assert_eq!(solve0(18), 2);
        assert_eq!(solve0(3), 5);
        assert_eq!(solve0(0), 7);
    }
}
