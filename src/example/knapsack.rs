use std::cmp::max;

use ndarray::{Array, Array2};
//---------snippet---------
use mod_neg_ext_int::NegExtInt::{self, *};
pub mod mod_neg_ext_int {
    use std::{cmp::Ordering, ops::Add};
    use NegExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NegExtInt {
        NegInf,
        Fin(i64),
    }
    impl NegExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => panic!("called `NegExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_neginf(self) -> bool {
            matches!(self, NegInf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                NegInf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Fin(a),
                None => NegInf,
            }
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (NegInf, NegInf) => NegInf,
                (NegInf, Fin(_)) => NegInf,
                (Fin(_), NegInf) => NegInf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for NegExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (NegInf, NegInf) => Some(Ordering::Equal),
                (NegInf, Fin(_)) => Some(Ordering::Less),
                (Fin(_), NegInf) => Some(Ordering::Greater),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for NegExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}

//use print_arr::*;
#[allow(dead_code)]
pub mod print_arr {
    use ndarray::{Array2, Array3};
    pub fn print_arr<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            print!("{:?} ", a);
        }
        println!();
    }
    pub fn print_arr2<T: std::fmt::Debug>(arr: &Array2<T>) {
        for i in 0..arr.nrows() {
            for j in 0..arr.ncols() {
                print!("{:?} ", arr[[i, j]]);
            }
            println!();
        }
    }
    pub fn print_arr3<T: std::fmt::Debug>(arr: &Array3<T>) {
        let shape = arr.shape();
        for i in 0..shape[0] {
            for j in 0..shape[1] {
                for k in 0..shape[2] {
                    print!("{:?} ", arr[[i, j, k]]);
                }
                println!();
            }
            println!();
        }
    }
}
//---------snippet---------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Item {
    weight: i64,
    value: i64,
}

#[allow(dead_code)]
fn solve4(n: usize, items: &[Item], max_weight: i64) -> i64 {
    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    // dp[0][0] = 0
    // dp[0][w] = 0 (w!=0)
    // dp[i+1][w] = max(dp[i][w], dp[i][w-items[i].weight] + items[i].value);
    // 答えは必ず存在する。max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    struct Dp {
        dp: Vec<Vec<NegExtInt>>,
    }
    impl Dp {
        fn new(n: usize, max_weight: i64) -> Dp {
            let max_weight = max_weight as usize;
            Dp { dp: vec![vec![NegInf; max_weight + 1]; n + 1] }
        }

        fn at(&self, i: usize, w: i64) -> &NegExtInt {
            // 添字の小さい方だけ考慮すればよい（大きい方はスルー）
            if w < 0 {
                &NegInf
            } else {
                &self.dp[i][w as usize]
            }
        }

        fn at_mut(&mut self, i: usize, w: i64) -> &mut NegExtInt {
            &mut self.dp[i][w as usize]
        }
    }

    let mut dp = Dp::new(n, max_weight);

    for w in 0..=max_weight {
        *dp.at_mut(0, w) = Fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            let choose = *dp.at(i, w - item.weight) + Fin(item.value);
            let no_choose = *dp.at(i, w);
            *dp.at_mut(i + 1, w) = max(choose, no_choose);
        }
    }
    dp.at(n, max_weight).get_fin()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test1() {
        let n: usize = 6;
        let items = [(2, 3), (1, 2), (3, 6), (2, 1), (1, 3), (5, 85)];
        let items = items
            .iter()
            .map(|(weight, value)| Item { weight: *weight, value: *value })
            .collect_vec();

        let max_weight = 9;

        let ans = solve4(n, &items, max_weight);
        assert_eq!(ans, 94);
    }
}
