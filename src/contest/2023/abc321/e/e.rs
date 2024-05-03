use std::{cmp::min, io::stdin};

struct TestCase {
    n: i64,
    x: i64,
    k: i64,
}
struct Problem {
    n: usize,
    test_case_list: Vec<TestCase>,
}

fn pow2(i: i64) -> i64 {
    assert!(i >= 0);
    assert!(i <= 63);
    i64::pow(2, i as u32)
}

fn parent(i: i64) -> i64 {
    assert!(i >= 2);
    i / 2
}

fn brother(i: i64) -> i64 {
    assert!(i >= 2);
    i ^ 1
}

fn n_descendants(i: i64, k: i64, n: i64) -> i64 {
    // iの子孫で距離がkのもの (k=0なら自分も含む)

    // iの子孫で距離がkのもの: [i * 2^k, (i+1) * 2^k)
    // [i * 2^k, (i+1) * 2^k) と [1, n] = [1, n+1) の共通部分の数を求める
    // n<=10^18
    // i * 2^k >= 10^18 + 1 のときは 0返す
    // log_2(i) + k >= log_2(10^18 + 1) のときは 0を返す
    // log_2(10^18 + 1) = 59.79....
    // log_2(i) + k >= 60 だったら 0を返しておけば良い。
    // k=55
    // log_2(i) = 5.1
    // k>= 60 であれば↑を満たす
    if i.ilog2() as i64 + k >= 60 {
        0
    } else {
        let begin = i * pow2(k);
        if begin > n {
            0
        } else {
            let end = min((i + 1) * pow2(k), n + 1);
            end - begin
        }
    }
}

#[allow(clippy::if_same_then_else)]
fn n_not_descendants(i: i64, k: i64, n: i64) -> i64 {
    // iの子孫以外で距離がkのもの
    if i == 1 {
        // ルート
        0
    } else if k == 0 {
        0
    } else if k == 1 {
        1 //親だけ
    } else {
        n_descendants(brother(i), k - 2, n) + n_not_descendants(parent(i), k - 1, n)
    }
}

impl TestCase {
    fn solve(&self) -> i64 {
        // 解法1: 子孫とそれ以外で場合分け。それ以外の部分は再帰になる
        let n = self.n; // 頂点の数
        let x = self.x; // x から距離 k の頂点の数を求めたい。
        let k = self.k;

        n_descendants(x, k, n) + n_not_descendants(x, k, n)
    }
    #[allow(clippy::if_same_then_else)]
    fn solve2(&self) -> i64 {
        // 解法2: 親の方を辿っていって、各頂点に対して親と兄弟でカウントをする
        // FIXME: RE が発生している
        let n = self.n; // 頂点の数
        let x = self.x; // x から距離 k の頂点の数を求めたい。
        let k = self.k;

        let s: i64 = std::iter::successors(Some((x, k)), |&(i, d)| {
            if i == 1 {
                None
            } else if parent(i) == 1 {
                None
            } else if d == 0 {
                None
            } else {
                Some((parent(i), d - 1))
            }
            //xの高さとiの高さから距離の計算をしても良かったかも
        })
        .map(|(i, d)| {
            //i の親と兄弟で距離がkのものを計算
            // 個々の部分は別の関数に切り出した方がいい。kという使わない変数が環境にあるので紛らわしい。
            if i == 1 {
                0
            } else if d == 0 {
                0
            } else if d == 1 {
                1
            } else {
                n_descendants(brother(i), d - 2, n)
            }
        })
        .sum();

        n_descendants(x, k, n) + s
    }

    #[allow(clippy::comparison_chain)]
    fn solve3(&self) -> i64 {
        // 方針3: 親の子孫 - 自分の子孫 を繰り返す
        let path_to_root: Vec<(i64, i64)> =
            std::iter::successors(
                Some(self.x),
                |&i| {
                    if i == 1 {
                        None
                    } else {
                        Some(parent(i))
                    }
                },
            )
            .tuple_windows()
            .collect_vec();

        let sum = path_to_root
            .iter()
            .copied()
            .enumerate()
            .map(|(i, (child, current))| {
                // current の子孫で距離が dist のもの - child の子孫で距離が dist -1 のもの
                let dist = self.k - i as i64 - 1;
                if dist < 0 {
                    0
                } else if dist == 0 {
                    1
                } else {
                    n_descendants(current, dist, self.n) - n_descendants(child, dist - 1, self.n)
                }
            })
            .sum::<i64>();
        n_descendants(self.x, self.k, self.n) + sum
    }
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let n = r.read_usize_1();
        let test_case_list = (0..n)
            .map(|_| {
                let (n, x, k) = r.read_i64_3();
                TestCase { n, x, k }
            })
            .collect_vec();
        Problem { n, test_case_list }
    }
    fn solve(&self) -> Answer {
        let Problem { n, test_case_list } = self;

        let ans = test_case_list.iter().map(|t| t.solve3()).collect_vec();
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
    Problem::read(ProconReader::new(stdin().lock()))
        .solve()
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
    fn test_brother() {
        assert_eq!(brother(4), 5);
        assert_eq!(brother(5), 4);

        assert_eq!(brother(6), 7);
        assert_eq!(brother(7), 6);
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
