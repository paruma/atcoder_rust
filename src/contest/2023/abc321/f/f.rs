use core::panic;
use std::io::stdin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Query {
    Add(i64),
    Erase(i64),
}
struct Problem {
    nq: usize,
    sum: i64,
    qs: Vec<Query>,
}

struct Dp {
    dp: Vec<Vec<RR>>,
}
impl Dp {
    fn new(n: usize, sum: i64) -> Dp {
        Dp { dp: vec![vec![RR::zero(); sum as usize + 1]; n + 1] }
    }

    fn at(&self, i: usize, sum: i64) -> RR {
        if sum < 0 {
            RR::zero()
        } else {
            self.dp[i][sum as usize]
        }
    }

    fn at_mut(&mut self, i: usize, sum: i64) -> &mut RR {
        &mut self.dp[i][sum as usize]
    }
}
impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (nq, sum) = r.read_i64_2();
        let nq = nq as usize;
        let qs = std::iter::repeat_with(|| {
            let line = r.read_vec_str();
            let v: i64 = line[1].parse().unwrap();
            match line[0].as_str() {
                "+" => Query::Add(v),
                "-" => Query::Erase(v),
                _ => panic!(),
            }
        })
        .take(nq)
        .collect_vec();
        Problem { nq, sum, qs }
    }
    fn solve(&self) -> Answer {
        let Problem { nq, sum, qs } = self;

        let mut dp = Dp::new(*nq, *sum);
        *dp.at_mut(0, 0) = RR::one();
        for (i, &query) in qs.iter().enumerate() {
            match query {
                Query::Add(v) => {
                    for s in 0..=*sum {
                        *dp.at_mut(i + 1, s) = dp.at(i, s - v) + dp.at(i, s);
                    }
                }
                Query::Erase(v) => {
                    for s in 0..=*sum {
                        *dp.at_mut(i + 1, s) = dp.at(i, s) - dp.at(i + 1, s - v);
                    }
                }
            }
        }
        let ans = (0..*nq).map(|i| dp.at(i + 1, *sum).rep()).collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        for &x in &self.ans {
            println!("{}", x);
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

use num::{One, Zero};
use rr::*;
pub mod rr {
    pub const MOD: i64 = 998_244_353;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RR {
        rep: i64,
    }
    impl RR {
        pub fn new(x: i64) -> RR {
            RR { rep: x.rem_euclid(MOD) }
        }
        pub fn rep(self) -> i64 {
            self.rep
        }
    }
    impl num_traits::Zero for RR {
        fn zero() -> Self {
            RR::new(0)
        }
        fn is_zero(&self) -> bool {
            self.rep == 0
        }
    }
    impl num_traits::One for RR {
        fn one() -> Self {
            RR::new(1)
        }
    }
    macro_rules ! bi_ops_impl {($ std_ops : ident , $ fn : ident , $ op : tt ) => {impl std :: ops ::$ std_ops for RR {type Output = Self ; fn $ fn (self , rhs : Self ) -> Self :: Output {RR :: new (self . rep $ op rhs . rep ) } } } ; }
    bi_ops_impl ! (Add , add , + );
    bi_ops_impl ! (Sub , sub , - );
    bi_ops_impl ! (Mul , mul , * );
    macro_rules ! bi_ops_assign_impl {($ std_ops_assign : ident , $ fn_assign : ident , $ op : tt ) => {impl std :: ops ::$ std_ops_assign for RR {fn $ fn_assign (& mut self , rhs : Self ) {* self = * self $ op rhs } } } ; }
    bi_ops_assign_impl ! (AddAssign , add_assign , + );
    bi_ops_assign_impl ! (SubAssign , sub_assign , - );
    bi_ops_assign_impl ! (MulAssign , mul_assign , * );
    impl std::ops::Neg for RR {
        type Output = Self;
        fn neg(self) -> Self::Output {
            RR::new(-self.rep)
        }
    }
}

use itertools::Itertools;
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
