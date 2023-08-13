use std::io::stdin;

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

struct Problem {
    n_sellers: usize,
    n_buyers: usize,
    seller_hope_list: Vec<i64>,
    buyer_hope_list: Vec<i64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Person {
    Seller { price: i64 },
    Buyer { price: i64 },
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (n_sellers, n_buyers) = r.read_usize_2();
        let seller_hope_list = r.read_vec_i64();
        let buyer_hope_list = r.read_vec_i64();
        Problem { n_sellers, n_buyers, seller_hope_list, buyer_hope_list }
    }
    fn solve(&self) -> Answer {
        let mut people: Vec<Person> = vec![];
        for &seller_hope in &self.seller_hope_list {
            people.push(Person::Seller { price: seller_hope });
        }
        for &buyer_hope in &self.buyer_hope_list {
            // ここバグってる
            // Seller ではなく、Buyer がただしい
            // （なんかACしてしまった）
            people.push(Person::Seller { price: buyer_hope + 1 });
        }
        people.sort_by_key(|&person| match person {
            Person::Seller { price } => price,
            Person::Buyer { price } => price,
        });
        let mut n_cnt_sellers = 0;
        let mut n_cnt_buyers = self.n_buyers;
        for person in people {
            match person {
                Person::Seller { price } => {
                    n_cnt_sellers += 1;
                }
                Person::Buyer { price } => {
                    n_cnt_buyers -= 1;
                }
            }
            if n_cnt_sellers == n_cnt_buyers {
                let ans = match person {
                    Person::Seller { price } => price,
                    Person::Buyer { price } => price,
                };
                return Answer { ans };
            }
        }
        panic!();
    }

    // 二分探索解法
    fn solve2(&self) -> Answer {
        let count_sellers = |price: i64| {
            // price 円なら売る人数
            // 希望以上の値段なら売る
            self.seller_hope_list.iter().filter(|price_lb| price >= **price_lb).count()
        };

        let count_buyers = |price: i64| {
            // price 円なら買う人数
            // 希望以下の値段なら買う
            self.buyer_hope_list.iter().filter(|price_ub| price <= **price_ub).count()
        };

        // min{price | count_sellers(price) >= count_buyers(price)} を求める。
        // 売りたい人数 >= 買いたい人数か？
        let pred = |price: i64| count_sellers(price) >= count_buyers(price);
        let ans = bin_search(2_000_000_000, 0, pred);
        Answer { ans }
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
    Problem::read(ProconReader::new(stdin().lock())).solve2().print();
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
