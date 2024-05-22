use std::io::stdin;

struct ElectoralDistrict {
    x: i64, // 高橋派
    y: i64, // 青木派
    z: i64, // 勝った場合に得られる議席数
}
struct Problem {
    n: usize,
    district: Vec<ElectoralDistrict>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let n = r.read_usize_1();
        let district = (0..n)
            .map(|_| {
                let (x, y, z) = r.read_i64_3();
                ElectoralDistrict { x, y, z }
            })
            .collect_vec();
        Problem { n, district }
    }
    fn solve(&self) -> Answer {
        let Problem { n, district } = self;
        let n = *n;
        //必要総議席数
        let max_sheets = 100_010;
        // dp[i][j]:  [0, i) の選挙区で j議席以上得るのに必要な鞍替え人数
        let mut dp = vec![vec![Inf; max_sheets]; n + 1];
        // 選挙区の数0の場合について初期化
        dp[0][0] = Fin(0);

        for i in 0..n {
            // 選挙区iを使う
            let d = &district[i];
            if d.x >= d.y {
                // 高橋派がもとから多い
                for j in 0..max_sheets {
                    // この選挙区は鞍替えしなくても勝つ
                    if d.z as usize > j {
                        dp[i + 1][j] = ExtInt::min(Fin(0), dp[i + 1][j])
                    } else {
                        dp[i + 1][j] = ExtInt::min(dp[i][j - d.z as usize], dp[i + 1][j])
                    }
                }
            } else {
                // 鞍替えをする
                let kuragae = (d.x + d.y) / 2 + 1 - d.x;
                for j in 0..max_sheets {
                    // usizeの引き算注意
                    // j-z からとってくる
                    if d.z as usize > j {
                        dp[i + 1][j] = ExtInt::min(Fin(kuragae), dp[i + 1][j]);
                    } else {
                        dp[i + 1][j] =
                            ExtInt::min(dp[i][j - d.z as usize] + Fin(kuragae), dp[i + 1][j]);
                    }
                    // この選挙区をあきらめる。
                    dp[i + 1][j] = ExtInt::min(dp[i][j], dp[i + 1][j])
                }
            }
        }

        let need = district.iter().map(|e| e.z).sum::<i64>() / 2 + 1;
        let ans = dp[n][need as usize].get_fin();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let Problem { n, district } = self;
        let n = *n;
        //必要総議席数
        let max_sheets = 100_010;
        // dp[i][j]:  [0, i) の選挙区で j議席以上得るのに必要な鞍替え人数
        let mut dp = vec![vec![Inf; max_sheets]; n + 1];
        // 選挙区の数0の場合について初期化
        dp[0][0] = Fin(0);

        for i in 0..n {
            // 選挙区iを使う
            let d = &district[i];
            // この選挙区で高橋派が勝つのに必要な鞍替えの数
            let kuragae = if d.x >= d.y {
                0
            } else {
                (d.x + d.y) / 2 + 1 - d.x
            };

            for j in 0..max_sheets {
                // dp[i+1][j] を求める
                // dp[i+1][j] = i番目までの選挙区でj議席以上獲得するのに必要な鞍替えの人数

                // この選挙区で高橋派が「勝つ」場合に必要な今までの鞍替えの数
                let case_win = if d.z as usize > j {
                    //この選挙区だけで勝てば良い
                    Fin(kuragae)
                } else {
                    dp[i][j - d.z as usize] + Fin(kuragae)
                };
                // // この選挙区で高橋派が「負ける」場合に必要な今までの鞍替えの数
                let case_lose = dp[i][j];

                dp[i + 1][j] = ExtInt::min(case_win, case_lose);
            }
        }

        let need = district.iter().map(|e| e.z).sum::<i64>() / 2 + 1;
        let ans = dp[n][need as usize].get_fin();
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // dp[i][z]: [0, i)の選挙区まで見たときの、z議席以上獲得するのに必要な最小鞍替え数
        // 議席数の合計値
        let sum_district = self.district.iter().map(|d| d.z).sum::<i64>() as usize;

        let mut dp = vec![vec![Inf; sum_district + 1]; self.n + 1];
        dp[0][0] = Fin(0);

        for (i, d) in self.district.iter().enumerate() {
            for z in 0..=sum_district {
                // この選挙区で勝つ
                let case_win = {
                    let majority = (d.x + d.y) / 2 + 1; //過半数
                    let kuragae = i64::max(majority - d.x, 0);
                    let prev = if z >= d.z as usize {
                        dp[i][z - d.z as usize]
                    } else {
                        dp[i][0]
                    };
                    prev + Fin(kuragae)
                };

                // この選挙区で勝たない
                let case_lose = dp[i][z];

                dp[i + 1][z] = ExtInt::min(case_win, case_lose);
            }
        }

        let ans = dp[self.n][sum_district / 2 + 1].get_fin();
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
    Problem::read(ProconReader::new(stdin().lock()))
        .solve3()
        .print();
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

use tropical::ExtInt::{self, *};
pub mod tropical {
    use std::{cmp::Ordering, ops::Add};
    use ExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtInt {
        Inf,
        Fin(i64),
    }
    impl ExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `ExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                Inf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_inf(self) -> bool {
            matches!(self, Inf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                Inf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Fin(a),
                None => Inf,
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for ExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for ExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
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
            buf.trim()
                .split(' ')
                .map(|s| s.parse::<T>().unwrap())
                .collect::<Vec<T>>()
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
