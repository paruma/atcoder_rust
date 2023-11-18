use std::{collections::HashSet, io::stdin};

struct Problem {
    final_day: i64,
    n_fire: usize,
    day_list: Vec<i64>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (final_day, n_fire) = r.read_any_2::<i64, usize>();
        let day_list = r.read_vec_i64();
        Problem { final_day, n_fire, day_list }
    }
    fn solve(&self) -> Answer {
        let Problem { final_day, n_fire, day_list } = self;
        //day_list: 花火が上がる日
        let ans = (1..=*final_day)
            .map(|i| {
                // ここの変数名 i は day とかがいいかも
                let next_fire_idx = lower_bound(day_list, i);
                day_list[next_fire_idx] - i
            })
            .collect_vec();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // O(n) 解法
        let Problem { final_day, n_fire, day_list } = self;

        // next_fire_day_list[day] = day日以降で初めて花火が上がる日
        let mut next_fire_day_list = vec![-1; *final_day as usize + 1];
        let mut day_stack = day_list.iter().rev().collect_vec();
        for day in 1..=*final_day {
            let next_day = {
                if day > **day_stack.last().unwrap() {
                    day_stack.pop();
                }

                **day_stack.last().unwrap()
            };
            next_fire_day_list[day as usize] = next_day;
        }
        let ans = (1..=*final_day).map(|day| next_fire_day_list[day as usize] - day).collect_vec();

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // O(n) 解法 (自作 Stack を使用する)
        let Problem { final_day, n_fire, day_list } = self;

        // next_fire_day_list[day] = day日以降で初めて花火が上がる日
        let mut next_fire_day_list = vec![-1; *final_day as usize + 1];
        let mut day_stack = Stack::new();
        for day in day_list.iter().rev() {
            day_stack.push(*day);
        }
        // stack のトップにある: 現時点での次の花火の日
        for day in 1..=*final_day {
            let next_day = {
                if day > *day_stack.peek().unwrap() {
                    day_stack.pop();
                }

                *day_stack.peek().unwrap()
            };
            next_fire_day_list[day as usize] = next_day;
        }
        let ans = (1..=*final_day).map(|day| next_fire_day_list[day as usize] - day).collect_vec();

        Answer { ans }
    }

    fn solve3_2(&self) -> Answer {
        // O(n) 解法 (peekable を使う)

        let Problem { final_day, n_fire, day_list } = self;

        // next_fire_day_list[day] = day日以降で初めて花火が上がる日
        let mut next_fire_day_list = vec![-1; *final_day as usize + 1];
        let mut fire_day_iter = day_list.iter().copied().peekable();

        for day in 1..=*final_day {
            let next_day = {
                if day > *fire_day_iter.peek().unwrap() {
                    fire_day_iter.next();
                }

                *fire_day_iter.peek().unwrap()
            };
            next_fire_day_list[day as usize] = next_day;
        }
        let ans = (1..=*final_day).map(|day| next_fire_day_list[day as usize] - day).collect_vec();

        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // O(n) 解法 (DPぽい。後ろから見る)
        let Problem { final_day, n_fire, day_list } = self;

        // 変数名が微妙なので変える
        let n_days = *final_day as usize;
        let fire_day_list = day_list;

        //ついでに0オリジンにする
        let fire_day_set: HashSet<usize> =
            fire_day_list.iter().map(|day| (*day - 1) as usize).collect();

        let mut ans = vec![-1; n_days];
        ans[n_days - 1] = 0;

        for day in (0..n_days - 1).rev() {
            if fire_day_set.contains(&day) {
                ans[day] = 0;
            } else {
                ans[day] = ans[day + 1] + 1;
            }
        }

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
    }
}

fn main() {
    Problem::read(ProconReader::new(stdin().lock())).solve3_2().print();
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

use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    #[allow(clippy::needless_range_loop)]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
}

use mod_stack::*;
pub mod mod_stack {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Stack<T> {
        raw: Vec<T>,
    }
    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack { raw: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.last()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Stack<T> {
        fn default() -> Self {
            Self::new()
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
/// 指定された要素以上の値が現れる最初の位置を返す。
/// ## Arguments
/// * xs: 単調増加
///     * 単調増加でなくても、 `|i| key <= xs[i]` が単調ならOK
/// ## Return
/// `I = {i in 0..xs.len() | key > xs[i]}` としたとき、`min I` を返す。
/// ただし、`I` が空の場合は `xs.len()` を返す
/// 戻り値は、区間 `0..=xs.len()` の間で返る。
pub fn lower_bound<T: PartialOrd>(xs: &[T], key: T) -> usize {
    let pred = |i: i64| key <= xs[i as usize];
    bin_search(xs.len() as i64, -1_i64, pred) as usize
}

// 3 4 4 5 5
//   ↑   ↑

// 5 5 4 4 3
//     ↑   ↑

use itertools::Itertools;
#[allow(unused_imports)]
use myio::*;
use proconio::fastout;
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
