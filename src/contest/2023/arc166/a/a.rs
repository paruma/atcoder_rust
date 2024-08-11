struct TestCase {
    n: usize,
    xs: Vec<u8>,
    ys: Vec<u8>,
}
struct Problem {
    n_test_case: usize,
    test_cases: Vec<TestCase>,
}

fn count_ch(zs: &[u8], ch: u8) -> usize {
    zs.iter().filter(|ch0| **ch0 == ch).count()
}

fn solve_sub(xs: &[u8], ys: &[u8]) -> bool {
    // ABCの数の整合性があっている
    let cnt_xa = count_ch(xs, b'A');
    let cnt_xb = count_ch(xs, b'B');
    let cnt_ya = count_ch(ys, b'A');
    let cnt_yb = count_ch(ys, b'B');

    if cnt_xa > cnt_ya || cnt_xb > cnt_yb {
        return false;
    }

    // xs の方にある C を A B に変える
    // 左側から cnt_ya - cnt_xa 個を A に変える。
    let mut xs = xs.to_vec();
    let mut cnt_change = 0;
    for i in 0..xs.len() {
        if xs[i] == b'C' {
            if cnt_change < cnt_ya - cnt_xa {
                xs[i] = b'A';
            } else {
                xs[i] = b'B';
            }
            cnt_change += 1;
        }
    }

    let xs_b_pos = xs.iter().copied().positions(|ch| ch == b'B').collect_vec();
    let ys_b_pos = ys.iter().copied().positions(|ch| ch == b'B').collect_vec();
    izip!(xs_b_pos, ys_b_pos).all(|(xp, yp)| xp >= yp)
}
impl TestCase {
    fn solve(&self) -> bool {
        let n = self.n;
        let xs = &self.xs;
        let ys = &self.ys;
        // まず y の C の位置に x でもCがあることを確認
        if izip!(xs, ys).any(|(x, y)| {
            if *y == b'C' && *x != b'C' {
                return true;
            }
            false
        }) {
            return false;
        }

        // y の C で分離する
        let mut begin = 0;
        for i in 0..n {
            if ys[i] == b'C' {
                // [begin, end) で分離する
                let end = i;
                if !solve_sub(&xs[begin..end], &ys[begin..end]) {
                    return false;
                }
                // 更新する
                begin = i + 1;
            }
        }
        // [begin, n) もやる
        let end = n;
        if !solve_sub(&xs[begin..end], &ys[begin..end]) {
            return false;
        }

        true
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_test_case: usize,
        }
        let test_cases = (0..n_test_case)
            .map(|_| {
                input! {
                    n: usize,
                    xs: Bytes,
                    ys: Bytes,
                }
                TestCase { n, xs, ys }
            })
            .collect_vec();
        Problem { n_test_case, test_cases }
    }
    fn solve(&self) -> Answer {
        let ans = self.test_cases.iter().map(|t| t.solve()).collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<bool>,
}

impl Answer {
    #[fastout]
    fn print(&self) {
        for &b in &self.ans {
            let msg = if b { "Yes" } else { "No" };
            println!("{}", msg);
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

    #[test]
    fn test_problem() {
        dbg!(solve_sub(&[], &[]));
    }
}

use bstr::ByteSlice;
// ====== import ======
use itertools::izip;
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
