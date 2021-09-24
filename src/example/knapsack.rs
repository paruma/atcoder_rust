use std::cmp::max;

use ndarray::{Array, Array2};
use neg_tropical::NegTrop::{self, *};
//---------snippet---------
#[allow(dead_code)]
pub mod neg_tropical {
    use std::{cmp::Ordering, ops::Add};
    use NegTrop::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NegTrop {
        NegInf,
        Fin(i64),
    }
    impl NegTrop {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => panic!("called `Trop::get_fin()` on a `Fin` value"),
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
    }
    impl Add for NegTrop {
        type Output = NegTrop;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (NegInf, NegInf) => NegInf,
                (NegInf, Fin(_)) => NegInf,
                (Fin(_), NegInf) => NegInf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for NegTrop {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (NegInf, NegInf) => Some(Ordering::Equal),
                (NegInf, Fin(_)) => Some(Ordering::Less),
                (Fin(_), NegInf) => Some(Ordering::Greater),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for NegTrop {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
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
fn solve(n: usize, items: &[Item], max_weight: i64) -> i64 {
    let max_weight = max_weight as usize;

    // dp[[i, w]] := [0,i)のitemsを使用したときの重さw以下での価値の最大値
    // dp[[0, 0]] = 0
    // dp[[0, w]] = -Inf (w!=0)
    // dp[[i+1, w]] = max(dp[[i, w]], dp[[i, w-items[i].weight]] + items[i].value;
    // 答えは必ず存在する、max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    let mut dp: Array2<NegTrop> = Array::from_shape_fn((n + 1, max_weight + 1), |_| NegInf);

    dp[[0, 0]] = Fin(0);

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            //let prev_w = max(w as i64 - item.weight, 0) as usize;

            // 配列外での値には注意
            let prev_val = if (w as i64) < item.weight {
                NegInf
            } else {
                let prev_w = (w as i64 - item.weight) as usize;
                dp[[i, prev_w]]
            };
            dp[[i + 1, w]] = max(dp[[i, w]], prev_val + Fin(item.value));
        }
    }

    dp[[n, max_weight]].get_fin()
}

/*
#[allow(dead_code)]
fn solve2(n: usize, items: &[Item], max_weight: i64) -> i64 {
    let max_weight = max_weight as usize;

    // dp[[i, w]] := [0,i)のitemsを使用したときの重さw以下での価値の最大値
    // dp[[0, 0]] = 0
    // dp[[0, w]] = -Inf (w!=0)
    // dp[[i+1, w]] = max(dp[[i, w]], dp[[i, w-items[i].weight]] + items[i].value;
    // 答えは必ず存在する、max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    let mut dp: Array2<NegTrop> = Array::from_shape_fn((n + 1, max_weight + 1), |_| NegInf);

    let dpfn = |i: usize, w: i64| {
        if w < 0 {
            NegInf
        } else {
            dp[[i, w as usize]]
        }
    };
    dp[[0, 0]] = Fin(0);

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            let w_i64 = w as i64;
            dp[[i + 1, w]] = max(
                dpfn(i, w_i64),
                dpfn(i, w_i64 - item.weight) + Fin(item.value),
            );
        }
    }
    dp[[n, max_weight]].get_fin()
}
*/

fn dpfn(dp: &Array2<NegTrop>, i: usize, w: i64) -> NegTrop {
    if w < 0 {
        NegInf
    } else {
        dp[[i, w as usize]]
    }
}
#[allow(dead_code)]
fn solve3(n: usize, items: &[Item], max_weight: i64) -> i64 {
    let max_weight = max_weight as usize;

    // dp[[i, w]] := [0,i)のitemsを使用したときの重さw以下での価値の最大値
    // dp[[0, 0]] = 0
    // dp[[0, w]] = -Inf (w!=0)
    // dp[[i+1, w]] = max(dp[[i, w]], dp[[i, w-items[i].weight]] + items[i].value;
    // 答えは必ず存在する、max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    let mut dp: Array2<NegTrop> = Array::from_shape_fn((n + 1, max_weight + 1), |_| NegInf);

    // これを定義しようとすると借用ルールに引っかかる
    // dpを引数に入れればとりあえずは大丈夫か。
    /*
    let dpfn = |i: usize, w: i64| {
        if w < 0 {
            NegInf
        } else {
            dp[[i, w as usize]]
        }
    };
    */

    dp[[0, 0]] = Fin(0);

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            let w_i64 = w as i64;
            dp[[i + 1, w]] = max(
                dpfn(&dp, i, w_i64),
                dpfn(&dp, i, w_i64 - item.weight) + Fin(item.value),
            );
        }
    }
    dp[[n, max_weight]].get_fin()
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
            .map(|(weight, value)| Item {
                weight: *weight,
                value: *value,
            })
            .collect_vec();

        let max_weight = 9;

        let ans = solve(n, &items, max_weight);
        assert_eq!(ans, 94);

        let ans = solve3(n, &items, max_weight);
        assert_eq!(ans, 94);
    }
}
