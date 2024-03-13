//#[derive_readable]
struct Problem {
    w: usize,
    h: usize,
    n: usize,
    xs: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            w: usize,
            h: usize,
            n: usize,
            xs: [usize; n],
        }
        Problem { w, h, n, xs }
    }
    fn solve(&self) -> Answer {
        let w = self.w;
        let h = self.h;
        let mut cnt = vec![0_i64; 31];
        for wi in 0..31 {
            let wb = w >> wi & 1;
            if wb == 0 {
                continue;
            }
            for hi in 0..31 {
                let hb = h >> hi & 1;
                if hb == 0 {
                    continue;
                }

                let min_i = wi.min(hi);
                let max_i = wi.max(hi);
                // 2^{min_i} サイズのチョコが何個作れるか。
                cnt[min_i] += 1 << (max_i - min_i);
            }
        }

        let mut current_size = 30; // 2^{current_size} のサイズを見ている

        for x in self.xs.iter().copied().sorted().rev() {
            while x < current_size {
                // チョコレートを割る
                cnt[current_size - 1] += cnt[current_size] * 4;
                cnt[current_size] = 0;
                current_size -= 1;
            }
            if cnt[x] == 0 {
                return Answer { ans: false };
            }
            cnt[x] -= 1;
        }

        let ans = true;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        println!("{}", if self.ans { "Yes" } else { "No" });
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
