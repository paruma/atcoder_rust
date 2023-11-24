#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    a: Usize1,
    b: Usize1,
}

struct Problem {
    n_box: usize,
    nq: usize,
    colors: Vec<usize>,
    queries: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_box: usize,
            nq: usize,
            colors: [Usize1; n_box],
            queries: [Query; nq],
        }
        Problem { n_box, nq, colors, queries }
    }
    fn solve(&self) -> Answer {
        let Problem { n_box, nq, colors, queries } = self;

        // box_to_colors[box_i]: box_i 番目の箱に入っているの色の集合
        let mut box_to_colors = colors.iter().copied().map(|c| HashSet::from([c])).collect_vec();
        let mut ans = Vec::new();
        for q in queries {
            // 箱a のボールを箱 b に移す

            let min_i =
                [q.a, q.b].into_iter().min_by_key(|box_i| box_to_colors[*box_i].len()).unwrap();
            let max_i =
                [q.a, q.b].into_iter().max_by_key(|box_i| box_to_colors[*box_i].len()).unwrap();

            // 挿入してから swap
            for x in std::mem::take(&mut box_to_colors[min_i]) {
                box_to_colors[max_i].insert(x);
            }

            if box_to_colors[q.a].len() > box_to_colors[q.b].len() {
                // std::mem::swap(&mut box_to_colors[q.a], &mut box_to_colors[q.b]);
                box_to_colors.swap(q.a, q.b)
            }
            ans.push(box_to_colors[q.b].len());
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let Problem { n_box, nq, colors, queries } = self;

        // box_to_colors[box_i]: box_i 番目の箱に入っているの色の集合
        let mut box_to_colors = colors.iter().copied().map(|c| HashSet::from([c])).collect_vec();
        let mut ans = Vec::new();
        for q in queries {
            // 箱a のボールを箱 b に移す

            if box_to_colors[q.a].len() > box_to_colors[q.b].len() {
                box_to_colors.swap(q.a, q.b)
            }
            for x in std::mem::take(&mut box_to_colors[q.a]) {
                box_to_colors[q.b].insert(x);
            }

            ans.push(box_to_colors[q.b].len());
        }

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
    }
}

fn main() {
    Problem::read().solve2().print();
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

use std::{
    cmp::{max, min},
    collections::HashSet,
};

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
