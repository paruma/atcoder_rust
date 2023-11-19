#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    a: Usize1,
    b: Usize1,
}

struct Bag {
    ball_set: HashSet<usize>,
    box_idx: usize,
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
        let mut bag_list_template = vec![HashSet::<usize>::new(); *n_box];
        let mut bag_empty = HashSet::<usize>::new();
        let mut bag_list = vec![];
        for i in 0..*n_box {
            bag_list.push(Some(&mut bag_list_template[i]));
        }
        for (box_i, color) in colors.iter().copied().enumerate() {
            match &mut bag_list[box_i] {
                Some(x) => {
                    x.insert(color);
                }
                None => unreachable!(),
            }
        }

        for query in queries.iter().copied() {
            // a → b
            match &mut bag_list[query.a] {
                Some(x) => {
                    //
                    match &mut bag_list[query.b] {
                        Some(y) => {
                            //
                            y.extend(x.iter());
                        }
                        None => {
                            //
                        }
                    }
                }
                None => {
                    //
                }
            }
            let x = &mut bag_list[query.a];
            let y = &mut bag_list[query.b];
            match (x, y) {
                (None, None) => todo!(),
                (None, Some(_)) => todo!(),
                (Some(_), None) => todo!(),
                (Some(_), Some(_)) => todo!(),
            }

            // a を bに入れる

            bag_list[query.a] = None;
            //
        }

        // let mut bag_list = bag_list
        //     .into_iter()
        //     .enumerate()
        //     .filter_map(|(i, x)| x.map(|x| (i, x))

        let ans = 0;
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

use std::collections::HashSet;

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
