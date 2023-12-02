use std::io::stdin;

struct TestCase {
    x1: i64,
    y1: i64,
    d1: char,
    x2: i64,
    y2: i64,
    d2: char,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CollisionTime {
    Ever,
    Never,
    Time(i64),
}

// 衝突時間の2倍を返す
fn collision_time(pos1: i64, vel1: i64, pos2: i64, vel2: i64) -> CollisionTime {
    let pos_diff = pos1 - pos2;
    let vel_diff = vel1 - vel2;
    if pos_diff == 0 {
        if vel_diff == 0 {
            return CollisionTime::Ever;
        }
        return CollisionTime::Time(0);
    }
    if vel_diff == 0 {
        // pos_diff != 0
        return CollisionTime::Never;
    }

    let time = -pos_diff * 2 / vel_diff;
    if time < 0 {
        return CollisionTime::Never;
    }
    CollisionTime::Time(time)
}

fn to_x(ch: char) -> i64 {
    match ch {
        'R' => 1,
        'L' => -1,
        _ => 0,
    }
}

fn to_y(ch: char) -> i64 {
    match ch {
        'U' => 1,
        'D' => -1,
        _ => 0,
    }
}

impl TestCase {
    fn solve(&self) -> bool {
        // 衝突時刻が同じか判定する
        let col_time_x = collision_time(self.x1, to_x(self.d1), self.x2, to_x(self.d2));
        let col_time_y = collision_time(self.y1, to_y(self.d1), self.y2, to_y(self.d2));
        // dbg!(col_time_x);
        // dbg!(col_time_y);
        match (col_time_x, col_time_y){
            (CollisionTime::Never, _) => false,
            (_, CollisionTime::Never) =>  false,
            (CollisionTime::Ever, CollisionTime::Ever) => true,
            (CollisionTime::Ever, CollisionTime::Time(_)) => true,
            (CollisionTime::Time(_), CollisionTime::Ever) => true,
            (CollisionTime::Time(tx), CollisionTime::Time(ty)) => tx==ty,
        }
    }
}

struct Problem {
    n_cases: usize,
    test_cases: Vec<TestCase>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let n_cases = r.read_usize_1();
        let test_cases = (0..n_cases)
            .map(|_| {
                let (x1, y1, d1) = r.read_any_3::<i64, i64, char>();
                let (x2, y2, d2) = r.read_any_3::<i64, i64, char>();
                TestCase { x1, y1, d1, x2, y2, d2 }
            })
            .collect();
        Problem { n_cases, test_cases }
    }
    fn solve(&self) -> Answer {
        let ans = self.test_cases.iter().map(|test_case| test_case.solve()).collect();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<bool>,
}

impl Answer {
    fn print(&self) {
        for &row in &self.ans {
            let msg = if row { "Yes" } else { "No" };
            println!("{}", msg);
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
