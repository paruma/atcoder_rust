//#[derive_readable]

struct Problem {
    h: usize,
    w: usize,
    grid1: Vec<Vec<i64>>,
    grid2: Vec<Vec<i64>>,
}

fn inversion_number(permu: &[usize]) -> i64 {
    // 0 1 2 4 3: 1個
    // 0 1 4 5 3 : 2個
    let mut cnt = 0;
    for (i, &p) in permu.iter().enumerate() {
        cnt += permu[i + 1..].iter().filter(|x| **x < p).count();
    }
    cnt as i64
}

pub fn inversion_number2(xs: &[usize]) -> i64 {
    use ac_library::{Additive, Segtree};
    if xs.is_empty() {
        return 0;
    }
    let max_val = xs.iter().copied().max().unwrap();

    // 各値が今までに現れた回数を記録する
    let mut segtree = Segtree::<Additive<i64>>::new(max_val + 1);
    let mut cnt = 0;
    for &x in xs {
        cnt += segtree.prod(x + 1..); // 今までに見たxより大きい値の数
        segtree.set(x, segtree.get(x) + 1)
    }

    cnt
}

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            grid1: [[i64; w]; h],
            grid2: [[i64; w]; h],
        }
        Problem { h, w, grid1, grid2 }
    }
    fn solve(&self) -> Answer {
        let Problem { h, w, grid1, grid2 } = self;
        let h = *h;
        let w = *w;
        let ans = iproduct!((0..h).permutations(h), (0..w).permutations(w))
            .filter_map(|(y_permu, x_permu)| {
                let grid1_after = (0..h)
                    .map(|y| (0..w).map(|x| grid1[y_permu[y]][x_permu[x]]).collect_vec())
                    .collect_vec();

                (grid1_after == *grid2)
                    .then_some(inversion_number2(&x_permu) + inversion_number2(&y_permu))
            })
            .min()
            .unwrap_or(-1);
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
        dbg!(inversion_number(&[0, 1, 2, 3]));
    }
}

// ====== import ======
use itertools::iproduct;
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
