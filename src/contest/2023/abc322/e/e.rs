#[derive(Clone, Debug, PartialEq, Eq)]
struct Plan {
    cost: i64,
    param_inc_list: Vec<i64>,
}

struct Problem {
    n_plans: usize,
    n_params: usize,
    param_lb: i64, // すべてのパラメータをこの値以上にしたい
    plan_list: Vec<Plan>,
}

struct Dp {
    // dp[i][vec![p0, p1, p2]]: [0,i) の 開発案を使って、各パラメータをp_i 以上にする場合のコストの最小値
    dp: Vec<HashMap<Vec<i64>, ExtInt>>,
    n_params: usize,
}

impl Dp {
    fn new(n_plans: usize, n_params: usize, param_lb: i64) -> Dp {
        // [0, param_lb]^n_params -> ExtInt
        let map = repeat(0..=param_lb)
            .take(n_params)
            .multi_cartesian_product()
            .map(|v| (v, Inf))
            .collect::<HashMap<_, _>>();
        Dp { dp: vec![map; n_plans + 1], n_params }
    }

    fn at(&self, i: usize, param_lb_list: &[i64]) -> &ExtInt {
        assert_eq!(param_lb_list.len(), self.n_params);
        // param_lb_list の中で負があったら0にする
        let param_lb_list = param_lb_list.iter().map(|p| max(*p, 0)).collect_vec();
        &self.dp[i][&param_lb_list]
    }

    fn at_mut(&mut self, i: usize, param_lb_list: &[i64]) -> &mut ExtInt {
        assert_eq!(param_lb_list.len(), self.n_params);
        self.dp[i].get_mut(param_lb_list).unwrap()
    }
}

fn vec_sub(v1: &[i64], v2: &[i64]) -> Vec<i64> {
    assert_eq!(v1.len(), v2.len());
    izip!(v1, v2).map(|(x, y)| *x - *y).collect_vec()
}

// i64の普通の値 → n進数で表現
fn to_n_ary_number(n: i64, value: i64) -> Vec<i64> {
    let mut digits = vec![];
    let mut value = value;
    while value != 0 {
        digits.push(value % n);
        value /= n;
    }
    digits.reverse();
    digits
}

// n進数で表現した値 → i64の普通の値
fn from_n_ary_number(n: i64, digits: &[i64]) -> i64 {
    digits.iter().fold(0, |acc, x| acc * n + x)
}

struct Dp2 {
    // dp[i][k]: [0,i) までを使って、パラメータを kのp+1進数表現された配列以上にする場合のコストの最小値
    dp: Vec<Vec<ExtInt>>,
    n_params: usize,
    param_lb: i64,
}

impl Dp2 {
    fn new(n_plans: usize, n_params: usize, param_lb: i64) -> Dp2 {
        // (param_lb + 1)^n_params
        Dp2 {
            dp: vec![vec![Inf; usize::pow((param_lb + 1) as usize, n_params as u32)]; n_plans + 1],
            n_params,
            param_lb,
        }
    }

    fn at(&self, i: usize, param_lb_list: &[i64]) -> &ExtInt {
        assert_eq!(param_lb_list.len(), self.n_params);
        // param_lb_list の中で負があったら0にする
        let param_lb_list = param_lb_list.iter().map(|p| max(*p, 0)).collect_vec();
        let param_n_ary = from_n_ary_number(self.param_lb + 1, &param_lb_list);
        &self.dp[i][param_n_ary as usize]
    }

    fn at_mut(&mut self, i: usize, param_lb_list: &[i64]) -> &mut ExtInt {
        assert_eq!(param_lb_list.len(), self.n_params);
        let param_n_ary = from_n_ary_number(self.param_lb + 1, param_lb_list);
        &mut self.dp[i][param_n_ary as usize]
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_plans: usize,
            n_params: usize,
            param_lb: i64,
        }
        let plan_list = (0..n_plans)
            .map(|_| {
                input! {
                    cost: i64,
                    param_inc_list: [i64; n_params],
                }
                Plan { cost, param_inc_list }
            })
            .collect_vec();
        Problem { n_plans, n_params, param_lb, plan_list }
    }
    fn solve(&self) -> Answer {
        // DP の型が Vec<HashMap<Vec<i64>, ExtInt>> の解法
        let mut dp = Dp::new(self.n_plans, self.n_params, self.param_lb);
        *dp.at_mut(0, &vec![0; self.n_params]) = Fin(0);
        for (plan_idx, plan) in self.plan_list.iter().enumerate() {
            for param_list in
                repeat(0..=self.param_lb).take(self.n_params).multi_cartesian_product()
            {
                let choose =
                    *dp.at(plan_idx, &vec_sub(&param_list, &plan.param_inc_list)) + Fin(plan.cost);
                let no_choose = *dp.at(plan_idx, &param_list);
                *dp.at_mut(plan_idx + 1, &param_list) = min(choose, no_choose);
            }
        }
        let ans = dp.at(self.n_plans, &vec![self.param_lb; self.n_params]).get_fin_or(-1);
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // DP の型が Vec<Vec<ExtInt>> の解法 (配列をn進数でエンコードする)
        let mut dp = Dp2::new(self.n_plans, self.n_params, self.param_lb);
        *dp.at_mut(0, &vec![0; self.n_params]) = Fin(0);
        for (plan_idx, plan) in self.plan_list.iter().enumerate() {
            for param_list in
                repeat(0..=self.param_lb).take(self.n_params).multi_cartesian_product()
            {
                let choose =
                    *dp.at(plan_idx, &vec_sub(&param_list, &plan.param_inc_list)) + Fin(plan.cost);
                let no_choose = *dp.at(plan_idx, &param_list);
                *dp.at_mut(plan_idx + 1, &param_list) = min(choose, no_choose);
            }
        }
        let ans = dp.at(self.n_plans, &vec![self.param_lb; self.n_params]).get_fin_or(-1);
        Answer { ans }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    #[fastout]
    fn print(&self) {
        println!("{}", self.ans);
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
    fn test_to_n_ary_number() {
        assert_eq!(to_n_ary_number(10, 123), vec![1, 2, 3]);
    }
    #[test]
    fn test_from_n_ary_number() {
        assert_eq!(from_n_ary_number(10, &[1, 2, 3]), 123);
    }

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use std::{
    cmp::{max, min},
    collections::HashMap,
    iter::repeat,
};

// ====== import ======
use itertools::izip;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== snippet ======
use mod_ext_int::ExtInt::{self, *};
pub mod mod_ext_int {
    use std::{cmp::Ordering, ops::Add};
    use ExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtInt {
        Inf,
        Fin(i64),
    }
    impl ExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `ExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                Inf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_inf(self) -> bool {
            matches!(self, Inf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                Inf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Fin(a),
                None => Inf,
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for ExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for ExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}
