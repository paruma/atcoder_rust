#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MealIndexComb {
    main: Usize1,
    sub: Usize1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Meal {
    idx: usize,
    price: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MealComb {
    main: Meal,
    sorted_main_idx: usize,
    sub: Meal,
    sorted_sub_idx: usize,
}

impl MealComb {
    fn price(&self) -> i64 {
        self.main.price + self.sub.price
    }
}

impl PartialOrd for MealComb {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.price(), &other.price())
    }
}

impl Ord for MealComb {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Problem {
    n_main: usize,
    n_sub: usize,
    n_bad_comb: usize,
    main_prices: Vec<i64>,
    sub_prices: Vec<i64>,
    bad_comb: Vec<MealIndexComb>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_main: usize,
            n_sub: usize,
            n_bad_comb: usize,
            main_prices: [i64; n_main],
            sub_prices: [i64; n_sub],
            bad_comb: [MealIndexComb; n_bad_comb],
        }
        Problem { n_main, n_sub, n_bad_comb, main_prices, sub_prices, bad_comb }
    }
    fn solve(&self) -> Answer {
        let Problem { n_main, n_sub, n_bad_comb, main_prices, sub_prices, bad_comb } = self;

        let main_list = main_prices
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, price)| Meal { idx, price })
            .sorted_by_key(|m| Reverse(m.price))
            .collect_vec();

        let sub_list = sub_prices
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, price)| Meal { idx, price })
            .sorted_by_key(|m| Reverse(m.price))
            .collect_vec();

        // 主菜副菜の組み合わせの上位 n_bad_comb + 1個を抽出する

        let top_list: Vec<MealComb> = {
            let mut buf = vec![];
            //(price, ソート後index) を置いてBinaryHeapすべきだった(price の順に出てくる)
            let mut pq = BinaryHeap::new();
            let mut visited: HashSet<MealComb> = HashSet::new();
            let init = MealComb {
                main: main_list[0],
                sorted_main_idx: 0,
                sub: sub_list[0],
                sorted_sub_idx: 0,
            };
            visited.insert(init);
            pq.push(init);

            while let Some(current) = pq.pop() {
                if buf.len() == *n_bad_comb + 1 {
                    break;
                }
                buf.push(current);

                let next1_opt = {
                    let main_opt = main_list.get(current.sorted_main_idx + 1);
                    let sub = sub_list[current.sorted_sub_idx];

                    main_opt.map(|&main| MealComb {
                        main,
                        sorted_main_idx: current.sorted_main_idx + 1,
                        sub,
                        sorted_sub_idx: current.sorted_sub_idx,
                    })
                };

                let next2_opt = {
                    let main = main_list[current.sorted_main_idx];
                    let sub_opt = sub_list.get(current.sorted_sub_idx + 1);
                    sub_opt.map(|&sub| MealComb {
                        main,
                        sorted_main_idx: current.sorted_main_idx,
                        sub,
                        sorted_sub_idx: current.sorted_sub_idx + 1,
                    })
                };

                for next in [next1_opt, next2_opt].into_iter().flatten() {
                    if !visited.contains(&next) {
                        visited.insert(next);
                        pq.push(next);
                    }
                }
            }
            buf
        };
        let x = HashSet::from_iter(bad_comb.iter());
        let bad_comb: HashSet<MealIndexComb> = bad_comb.iter().copied().collect::<HashSet<_>>();

        let ans = top_list
            .iter()
            .copied()
            .filter(|comb| {
                !bad_comb.contains(&MealIndexComb { main: comb.main.idx, sub: comb.sub.idx })
            })
            .map(|comb| comb.price())
            .max()
            .unwrap();
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

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
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
