//#[derive_readable]
struct Problem {
    n: usize,
    xs: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: Bytes
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        // 解法
        // 括弧列の問題と同じようにAを+1, Bを-1として考えると、和はAの票数-Bの票数になる。和が正・0・負のどれかということに関心がある。
        // 1234
        // 2134
        // 2314
        // 2341
        // のように隣接した2要素を交換しながら1を動かして、全通り試す。
        // 隣接した2要素を交換したときの票数差（累積和）は括弧列のときと同様に簡単に計算できる。
        let mut xs = self
            .xs
            .iter()
            .copied()
            .map(|ch| match ch {
                b'A' => 1_i64,
                b'B' => -1,
                _ => panic!(),
            })
            .collect_vec();

        let mut cumsum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            cumsum[i] = cumsum[i - 1] + xs[i - 1];
        }

        let mut ans = 1;

        // (0,1), (1,2)... をスワップさせていって、全パターン試す。
        for i in 1..xs.len() {
            if xs[i - 1] != xs[i] {
                // Aの票数-Bの票数
                let before = cumsum[i];
                let after = cumsum[i] + xs[i] * 2;

                // Aの方が投票数が多い → 1
                // 投票数が同数 → 0
                // Bの方が投票数が多い → -1
                let before_result = i64::clamp(before, -1, 1);
                let after_result = i64::clamp(after, -1, 1);
                if before_result != after_result {
                    ans += 1;
                }

                cumsum[i] = after;
            }
            xs.swap(i - 1, i);
        }

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
