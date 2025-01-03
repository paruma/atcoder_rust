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
        Problem {
            n_main,
            n_sub,
            n_bad_comb,
            main_prices,
            sub_prices,
            bad_comb,
        }
    }
    fn solve(&self) -> Answer {
        // 解法1: [a[i] + b[j] | i = 1,...,N, j = 1,...,M] の top L + 1 を Priority Queue で求める。
        // a[i] + b[j] を取り出したら、a[i+1]+ b[j] と a[i] + b[j+1] を Priority Queue に入れる。
        let Problem {
            n_main,
            n_sub,
            n_bad_comb,
            main_prices,
            sub_prices,
            bad_comb,
        } = self;

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

        // ソート後の添字で取得する
        let top_list: Vec<(usize, usize)> = {
            let mut buf: Vec<(usize, usize)> = vec![]; // (主菜の添字, 副菜の添字)

            // (価値, 主菜の添字, 副菜の添字) を Priority Queue に入れる
            let mut pq: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            pq.push((main_list[0].price + sub_list[0].price, 0, 0));
            visited.insert((0, 0));
            while let Some((_price, main_idx, sub_idx)) = pq.pop() {
                buf.push((main_idx, sub_idx));
                if buf.len() == n_bad_comb + 1 {
                    break;
                }

                for (next_main_idx, next_sub_idx) in
                    [(main_idx + 1, sub_idx), (main_idx, sub_idx + 1)]
                {
                    if next_main_idx < *n_main
                        && next_sub_idx < *n_sub
                        && !visited.contains(&(next_main_idx, next_sub_idx))
                    {
                        visited.insert((next_main_idx, next_sub_idx));
                        pq.push((
                            main_list[next_main_idx].price + sub_list[next_sub_idx].price,
                            next_main_idx,
                            next_sub_idx,
                        ));
                    }
                }
            }

            buf
        };

        let bad_comb: HashSet<MealIndexComb> = bad_comb.iter().copied().collect::<HashSet<_>>();
        let ans = top_list
            .iter()
            .copied()
            .filter_map(|(main_idx, sub_idx)| {
                let main = main_list[main_idx];
                let sub = sub_list[sub_idx];
                if bad_comb.contains(&MealIndexComb {
                    main: main.idx,
                    sub: sub.idx,
                }) {
                    None
                } else {
                    Some(main.price + sub.price)
                }
            })
            .max()
            .unwrap();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 副菜をソートして、主菜全探索
        let Problem {
            n_main,
            n_sub,
            n_bad_comb,
            main_prices,
            sub_prices,
            bad_comb,
        } = self;

        let bad_comb: HashSet<MealIndexComb> = bad_comb.iter().copied().collect::<HashSet<_>>();
        // 実は main はソート不要
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

        let ans = main_list
            .iter()
            .copied()
            .flat_map(|main| {
                // 主菜 main を固定した上で最も高い副菜を探す（ただし、食べ合わせの良いものに限る）
                sub_list
                    .iter()
                    .find(|sub| {
                        !bad_comb.contains(&MealIndexComb {
                            main: main.idx,
                            sub: sub.idx,
                        })
                    }) // プログラム全体で O(n_bad_cmb) の計算量
                    .map(|sub| main.price + sub.price)
            })
            .max()
            .unwrap();

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // 解法3: 主菜+副菜の金額 top L+1 を含むようなリストを作る。
        // 主菜と副菜を降順にソートして考える。
        // (0オリジンで) i番目の主菜とj番目の副菜の組合せより値段の高い組合せは少なくても (i + 1) * (j + 1) - 1 通りある。
        // (縦と横の長さが i + 1, j + 1 の長方形をイメージするとわかりやすい)
        // つまり、(i + 1) * (j + 1) - 1 >= L + 1 となる (i, j) は調べなくても良い。
        // よって、(i + 1) * (j + 1) - 1 < L + 1 となる (i, j) のみを調べれば良い。
        // これは調和級数forループで調べられる。
        let Problem {
            n_main,
            n_sub,
            n_bad_comb,
            main_prices,
            sub_prices,
            bad_comb,
        } = self;

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

        let bad_comb: HashSet<MealIndexComb> = bad_comb.iter().copied().collect::<HashSet<_>>();

        // (i + 1) * (j + 1) - 1 < n_bad_comb + 1 を満たす範囲でループを回す
        let ans = (0..*n_main)
            .take_while(|i| (i + 1) - 1 < n_bad_comb + 1)
            .flat_map(|i| {
                (0..*n_sub)
                    .take_while(move |j| (i + 1) * (j + 1) - 1 < n_bad_comb + 1)
                    .map(move |j| (i, j))
            })
            .filter_map(|(i, j)| {
                let main_meal = main_list[i];
                let sub_meal = sub_list[j];

                if bad_comb.contains(&MealIndexComb {
                    main: main_meal.idx,
                    sub: sub_meal.idx,
                }) {
                    None
                } else {
                    Some(main_meal.price + sub_meal.price)
                }
            })
            .max()
            .unwrap();

        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // ドント式の要領で
        //  [a[i] + b[j] | i = 1,...,N, j = 1,...,M] の top L + 1 を求める

        let Problem {
            n_main,
            n_sub,
            n_bad_comb,
            main_prices,
            sub_prices,
            bad_comb,
        } = self;

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

        let top_list: Vec<(usize, usize)> = {
            let mut buf = vec![];

            let mut pq = BinaryHeap::<(i64, usize, usize)>::new();

            for (main_i, main) in main_list.iter().copied().enumerate() {
                let sub = sub_list[0];
                pq.push((main.price + sub.price, main_i, 0));
            }

            while let Some((_, main_idx, sub_idx)) = pq.pop() {
                if buf.len() == n_bad_comb + 1 {
                    break;
                }
                buf.push((main_idx, sub_idx));
                if sub_idx < n_sub - 1 {
                    // sub を 1 だけ進める (ドント式だと ÷k から ÷(k+1) にするところ)
                    let next_main_idx = main_idx;
                    let next_sub_idx = sub_idx + 1;
                    let next_price = main_list[next_main_idx].price + sub_list[next_sub_idx].price;

                    pq.push((next_price, next_main_idx, next_sub_idx));
                }
            }
            buf
        };

        let bad_comb: HashSet<MealIndexComb> = bad_comb.iter().copied().collect::<HashSet<_>>();

        let ans = top_list
            .iter()
            .copied()
            .filter_map(|(sorted_main_idx, sorted_sub_idx)| {
                let main = main_list[sorted_main_idx];
                let sub = sub_list[sorted_sub_idx];
                if bad_comb.contains(&MealIndexComb {
                    main: main.idx,
                    sub: sub.idx,
                }) {
                    None
                } else {
                    Some(main.price + sub.price)
                }
            })
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
