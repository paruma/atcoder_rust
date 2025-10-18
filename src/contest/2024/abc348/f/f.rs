// #[derive_readable]
#[derive(Debug)]
struct Problem {
    n: usize,
    m: usize,
    xss: Vec<Vec<i32>>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            m: usize,
            xss: [[i32; m]; n],

        }
        Problem { n, m, xss }
    }
    fn solve(&self) -> Answer {
        let ans = 0;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        // u16 だと 1068ms
        // i16 だと 1222ms
        // u32 だと  973ms
        // i32 だと 1006ms
        // u64 だと 1261ms
        // i64 だと 1489ms
        // 16bit や 64 bit より 32 bit のほうが速そう
        // signed より unsigned のほうが速そう

        // self.xss.tuple_combinations() でよかった
        let ans = (0..self.n)
            .tuple_combinations()
            .filter(|(i, j)| {
                izip!(&self.xss[*i], &self.xss[*j])
                    .filter(|(x, y)| x == y)
                    .count()
                    % 2
                    == 1
            })
            .count();
        Answer { ans }
    }

    fn solve_naive2(&self) -> Answer {
        // 添え字アクセスするとTLEする
        let ans = (0..self.n)
            .tuple_combinations()
            .filter(|(i, j)| {
                let xs1 = &self.xss[*i];
                let xs2 = &self.xss[*j];
                (0..self.m).filter(|k| xs1[*k] == xs2[*k]).count() % 2 == 1
            })
            .count();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: usize,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve_naive().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_os_rng();
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

use itertools::izip;
// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
