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

struct Problem {
    n: i64,
    mat: Vec<Vec<i64>>,
}

struct Answer {
    mat: Vec<Vec<i64>>,
}

impl Problem {
    fn read() -> Problem {
        let n = read_i64_1();
        let mat = (0..n)
            .map(|_| {
                read_line()
                    .bytes()
                    .map(|c| if c == b'1' { 1_i64 } else { 0_i64 })
                    .collect_vec()
            })
            .collect_vec();
        Problem { n, mat }
    }
    fn solve(&self) -> Answer {
        let mut ans_mat = self.mat.clone();
        let n = self.n as usize;

        // 上の辺
        for x in 1..=n - 1 {
            ans_mat[0][x] = self.mat[0][x - 1];
        }

        // 右の辺
        for y in 1..=n - 1 {
            ans_mat[y][n - 1] = self.mat[y - 1][n - 1];
        }
        // 下の辺
        for x in 0..=n - 2 {
            ans_mat[n - 1][x] = self.mat[n - 1][x + 1];
        }

        // 左の辺
        for y in 0..=n - 2 {
            ans_mat[y][0] = self.mat[y + 1][0];
        }
        Answer { mat: ans_mat }
    }
}

impl Answer {
    fn print(self) {
        self.mat
            .iter()
            .for_each(|row| println!("{}", row.iter().join("")));
        // println!(self.ans)
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
