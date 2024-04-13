#[derive_readable]
#[derive(Debug)]
struct Problem {
    x_pc: u32,
    y_pc: u32,
    c: u64,
}

fn to_bits(x: u64) -> Vec<u64> {
    (0..60).rev().map(|i| (x >> i) & 1).collect()
}

fn from_bits(x_bits: &[u64]) -> u64 {
    x_bits
        .iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, b)| b << i)
        .sum()
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let x_pc = self.x_pc;
        let y_pc = self.y_pc;
        let c = self.c;
        let c_pc = c.count_ones();

        // 最初から i32(i64) でよかった感じがする
        let x_and_y_pc_times2 = (x_pc + y_pc) as i32 - c_pc as i32;
        let ans = if x_and_y_pc_times2 % 2 == 0
            && 0 <= x_and_y_pc_times2
            && x_and_y_pc_times2 / 2 <= u32::min(x_pc, y_pc) as i32
            && x_and_y_pc_times2 / 2 <= (60 - c_pc) as i32
        {
            let x_and_y_pc = (x_and_y_pc_times2 / 2) as u32;
            // (x&y)&c=0 となるようにする
            let c_bits = to_bits(c);
            let mut x_bits = vec![0; 60];
            let mut y_bits = vec![0; 60];

            let c_bits_0_pos = c_bits.iter().copied().positions(|b| b == 0).collect_vec();
            let c_bits_1_pos = c_bits.iter().copied().positions(|b| b == 1).collect_vec();

            // ここは、take ではなくスライス使うべきだった
            // take だと足りない場合にエラーにならない
            for i in c_bits_0_pos.iter().copied().take(x_and_y_pc as usize) {
                x_bits[i] = 1;
                y_bits[i] = 1;
            }
            for &i in &c_bits_1_pos[0..(x_pc - x_and_y_pc) as usize] {
                x_bits[i] = 1;
            }

            for &i in
                &c_bits_1_pos[(x_pc - x_and_y_pc) as usize..(x_pc + y_pc - 2 * x_and_y_pc) as usize]
            {
                y_bits[i] = 1;
            }

            Some((from_bits(&x_bits), from_bits(&y_bits)))
        } else {
            None
        };

        let ans = Answer { ans };
        // assert!(self.check_ans(&ans));
        self.assert_check_ans(&ans);
        ans
    }

    fn check_ans(&self, ans: &Answer) -> bool {
        if let Some((x, y)) = ans.ans {
            x < 2_u64.pow(60)
                && y < 2_u64.pow(60)
                && x.count_ones() == self.x_pc
                && y.count_ones() == self.y_pc
                && x ^ y == self.c
        } else {
            true
        }
    }

    fn assert_check_ans(&self, ans: &Answer) {
        if let Some((x, y)) = ans.ans {
            assert!(x < 2_u64.pow(60));
            assert!(y < 2_u64.pow(60));
            assert!(x.count_ones() == self.x_pc);
            assert!(y.count_ones() == self.y_pc);
            assert!(x ^ y == self.c);
        }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<(u64, u64)>,
}

impl Answer {
    fn print(&self) {
        if let Some((x, y)) = self.ans {
            println!("{} {}", x, y);
        } else {
            println!("-1");
        }
    }
}

fn main() {
    Problem::read().solve().print();
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
        let mut rng = SmallRng::from_entropy();
        let x_pc = rng.gen_range(0..=60);
        let y_pc = rng.gen_range(0..=60);
        let c = rng.gen_range(0..2_u64.pow(60));
        let p = Problem { x_pc, y_pc, c };
        // dbg!(&p);
        p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            let p = make_random_problem();
            p.solve();
        }
    }
}

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
