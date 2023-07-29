use std::{cmp::Reverse, io::stdin};

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

        fn read_usize_1(&mut self) -> usize {
            self.read_any_1::<usize>()
        }

        fn read_usize_2(&mut self) -> (usize, usize) {
            self.read_any_2::<usize, usize>()
        }

        fn read_usize_3(&mut self) -> (usize, usize, usize) {
            self.read_any_3::<usize, usize, usize>()
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
            Scanl { iter, state: Some(init), f }
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
            self.state = self.state.and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));
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

/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok);
        assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Product {
    PullTabCan { happiness: i64 },
    RegularCan { happiness: i64 },
    CanOpener { cnt: i64 },
}

struct Problem {
    n_products: usize,
    n_choices: usize,
    products: Vec<Product>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (n_products, n_choices) = r.read_usize_2();
        let products = (0..n_products)
            .map(|_| {
                let (t, x) = r.read_i64_2();
                match t {
                    0 => Product::PullTabCan { happiness: x },
                    1 => Product::RegularCan { happiness: x },
                    2 => Product::CanOpener { cnt: x },
                    _ => panic!(),
                }
            })
            .collect_vec();
        Problem { n_products, n_choices, products }
    }
    fn solve(&self) -> Answer {
        let mut openners = self
            .products
            .iter()
            .filter_map(|&product| match product {
                Product::PullTabCan { happiness } => None,
                Product::RegularCan { happiness } => None,
                Product::CanOpener { cnt } => Some(cnt),
            })
            .collect_vec();
        let mut pull_tab_cans = self
            .products
            .iter()
            .filter_map(|&product| match product {
                Product::PullTabCan { happiness } => Some(happiness),
                Product::RegularCan { happiness } => None,
                Product::CanOpener { cnt } => None,
            })
            .collect_vec();
        let mut regular_cans = self
            .products
            .iter()
            .filter_map(|&product| match product {
                Product::PullTabCan { happiness } => None,
                Product::RegularCan { happiness } => Some(happiness),
                Product::CanOpener { cnt } => None,
            })
            .collect_vec();
        openners.sort_by_key(|x| Reverse(*x));
        pull_tab_cans.sort_by_key(|x| Reverse(*x));
        regular_cans.sort_by_key(|x| Reverse(*x));
        let openners_cumsum = openners.iter().scanl(0, |acc, x| *acc + *x).collect_vec();
        let pull_tab_can_cumsum = pull_tab_cans.iter().scanl(0, |acc, x| *acc + *x).collect_vec();
        let regular_can_cumsum = regular_cans.iter().scanl(0, |acc, x| *acc + *x).collect_vec();
        let mut max_happiness = 0;
        for n_cnt_openners in 0..=openners.len() {
            if n_cnt_openners > self.n_choices {
                break;
            }
            // n_cnt_openners: 使った缶切りの数

            // 開ける回数
            let can_open_cnt = openners_cumsum[n_cnt_openners];
            let n_remain_cans = self.n_choices - n_cnt_openners;
            // 缶切り使わないケースが怖い
            if pull_tab_cans.len() == 0 {
                max_happiness = i64::max(
                    max_happiness,
                    regular_can_cumsum
                        [usize::min(can_open_cnt as usize, regular_can_cumsum.len() - 1)],
                );
            } else {
                let x = pull_tab_cans[0];
                // xより大きいのを缶切り使う方からもってくる
                let prod = |i: i64| {
                    if i >= regular_cans.len() as i64 {
                        false
                    } else {
                        dbg!(regular_cans[i as usize]);
                        regular_cans[i as usize] >= x
                    }
                };
                // y番目までの缶切り使う缶を使う。y=-1 だったら使わない。
                let y = bin_search(-1, n_remain_cans as i64 + 20, prod);
                assert!(-1 <= y && y <= n_remain_cans as i64);
                dbg!(y);
                max_happiness = i64::max(
                    max_happiness,
                    regular_can_cumsum[(y + 1) as usize]
                        + pull_tab_can_cumsum[usize::min(
                            n_remain_cans - (y + 1) as usize,
                            pull_tab_can_cumsum.len() - 1,
                        )],
                );
            }
            dbg!(max_happiness);
        }

        Answer { ans: max_happiness }
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
    Problem::read(ProconReader::new(stdin().lock())).solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(ProconReader::new(input.as_bytes())).solve();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_problem() {
        let input = "
3
4
        "
        .trim();
        // check(input, Answer { ans: 7 });
    }
}
