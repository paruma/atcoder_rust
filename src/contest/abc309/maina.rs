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

struct Problem {
    a: i64,
    b: i64,
}

struct Answer {
    ans: bool,
}

impl Problem {
    fn read() -> Problem {
        let (a, b) = read_i64_2();
        Problem { a, b }
    }
    fn solve(&self) -> Answer {
        let a = self.a - 1;
        let b = self.b - 1;

        //行が一致していない
        if a / 3 != b / 3 {
            return Answer { ans: false };
        }
        // 列の差が1
        if b % 3 - a % 3 == 1 {
            return Answer { ans: true };
        }
        Answer { ans: false }
    }
}

impl Answer {
    fn print(self) {
        let msg = if self.ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
