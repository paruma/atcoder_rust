//#[derive_readable]
struct Problem {
    n_socks: usize,
    n_lost_colors: usize,
    lost_colors: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_socks: usize,
            n_colors: usize,
            colors: [Usize1; n_colors],
        }
        Problem {
            n_socks,
            n_lost_colors: n_colors,
            lost_colors: colors,
        }
    }
    fn solve(&self) -> Answer {
        let n_socks = self.n_socks;
        let n_lost_colors = self.n_lost_colors;
        let lost_colors = &self.lost_colors;

        // なくしていない色の靴下の組はそのままその組み合わせにする。
        // [貪欲法と三角不等式]
        // (1,2), (2,3) の組み合わせは (1,3), (2,2)の組み合わせにしても損をしない

        let ans = if n_lost_colors % 2 == 0 {
            (0..n_lost_colors / 2)
                .map(|i| lost_colors[2 * i + 1] - lost_colors[2 * i])
                .sum::<usize>() as i64
        } else if n_lost_colors == 1 {
            0
        } else {
            // lost_color[1] - lost_color[0], lost_color[3] - lost_color[2], ...
            let weirdness0 = (0..n_lost_colors / 2)
                .map(|i| (lost_colors[2 * i + 1] - lost_colors[2 * i]) as i64)
                .collect_vec();
            // lost_color[2] - lost_color[1], lost_color[4] - lost_color[3], ...
            let weirdness1 = (0..n_lost_colors / 2)
                .map(|i| (lost_colors[2 * i + 2] - lost_colors[2 * i + 1]) as i64)
                .collect_vec();

            let cumsum0 = CumSum::new(&weirdness0);
            let cumsum1 = CumSum::new(&weirdness1);

            (0..n_lost_colors / 2 + 1) // 範囲に注意
                .map(|i| {
                    // weirdness0[0] + ... + weirdness0[i-1] + weirdness1[i] + ...weirdness1[n_lost_colors/2-1]
                    cumsum0.get_interval_sum(0, i) + cumsum1.get_interval_sum(i, n_lost_colors / 2)
                })
                .min()
                .unwrap()
        };

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
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
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
use cumsum::*;
pub mod cumsum {
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }
    impl CumSum {
        /// 計算量: O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        /// 計算量: O(1)
        pub fn get_interval_sum(&self, begin: usize, end: usize) -> i64 {
            self.cumsum[end] - self.cumsum[begin]
        }
    }
}
