/*
    本番は30秒間に合わず。

    * 二分探索を使っていたので想定解法と比べて log がついてしまった
    * DP の値に i64 ではなく NegExtInt を使っていて、定数倍が遅くなっていた。
        * 当時は enum{NegInf, Fin(i64)} という実装だったため遅かった。
    * 二分探索のたびに DP 配列を初期化してしまっていた
        * next DP をしたり、配列をあらかじめとっておくと解決する
    * 本番は DP 配列をあらかじめ取ろうとしたが、二分探索に渡す述語関数が FnMut ではなく Fn を要求してきて、DP 配列を述語関数（クロージャー）でキャプチャできなかった。

    それぞれの実装は以下の通り

    * solve1: 二分探索 & DP の TLE 解法（定数倍が遅い）
    * solve2: solve1 の DP テーブル初期化を1回だけ行ったもの
    * solve3: solve2 のリファクタリング (二分探索の手動インライン展開をやめるなど)
    * solve4: solve1 を next dp にしたもの
    * solve5: DP を二分探索の外で行ったもの
    * solve6: DP & 貪欲法 (想定解法)

*/
#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Food {
    t: Usize1,
    vit: i64,
    cal: i64,
}
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    x: i64,
    foods: Vec<Food>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            x: i64,
            foods: [Food; n],
        }
        Problem { n, x, foods }
    }

    fn solve1(&self) -> Answer {
        // 愚直実装 (TLE)

        /// カロリーの最小値
        fn solve_sub(foods: &[Food], vit: i64, x: i64) -> Option<i64> {
            let n = foods.len();
            let mut dp = vec![vec![NegExtInt::NEG_INF; x as usize + 1]; n + 1];
            for cal in 0..=x as usize {
                dp[0][cal] = NegExtInt::fin(0);
            }

            for i in 0..n {
                let food = foods[i];
                for cal in 0..=x {
                    let choose = if cal < food.cal {
                        NegExtInt::NEG_INF
                    } else {
                        dp[i][(cal - food.cal) as usize] + food.vit
                    };

                    let no_choose = dp[i][cal as usize];

                    dp[i + 1][cal as usize] = NegExtInt::max(choose, no_choose);
                }
            }

            // dp[n][cal] == ビタミン最大値
            // dp[n][cal] >= bit となるような cal の最小値を求める

            (0..=x).find(|&cal| dp[n][cal as usize] >= NegExtInt::fin(vit))
        }

        let x = self.x;
        let foods = &self.foods;
        let type_to_foods = foods.iter().fold(vec![vec![]; 3], |mut acc, food| {
            acc[food.t].push(*food);
            acc
        });

        let sum_vit = foods.iter().map(|f| f.vit).sum::<i64>();

        let ans = bin_search(0, sum_vit + 1, |vit| {
            // それぞれ vit 以上のビタミンを取る。そのとき、カロリーの最小値 が x 以下か

            let sum_cal = (0..3)
                .map(|t| {
                    let foods = &type_to_foods[t];
                    solve_sub(foods, vit, x).unwrap_or(i64::MAX / 10)
                })
                .sum::<i64>();
            // dbg!(vit);
            // dbg!(sum_cal);
            sum_cal <= x
        });

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // DP 初期化を1回だけ(本番解法)
        // クロージャーと mut の制約で map や bin_search のクロージャーが使えないのが厳しい。

        /// カロリーの最小値
        fn solve_sub(foods: &[Food], vit: i64, dp: &mut [Vec<NegExtInt>], x: i64) -> Option<i64> {
            let n = foods.len();
            let x = foods.iter().map(|f| f.cal).sum::<i64>().min(x);

            for cal in 0..=x as usize {
                dp[0][cal] = NegExtInt::fin(0);
            }

            for i in 0..n {
                let food = foods[i];
                for cal in 0..=x {
                    let choose = if cal < food.cal {
                        NegExtInt::NEG_INF
                    } else {
                        dp[i][(cal - food.cal) as usize] + food.vit
                    };

                    let no_choose = dp[i][cal as usize];

                    dp[i + 1][cal as usize] = std::cmp::max(choose, no_choose);
                }
            }

            // dp[n][cal] == ビタミン最大値
            // dp[n][cal] >= bit となるような cal の最小値を求める

            (0..=x).find(|&cal| dp[n][cal as usize] >= NegExtInt::fin(vit))
        }

        let n = self.n;
        let x = self.x;
        let foods = &self.foods;
        let type_to_foods = foods.iter().fold(vec![vec![]; 3], |mut acc, food| {
            acc[food.t].push(*food);
            acc
        });

        let sum_vit = foods.iter().map(|f| f.vit).sum::<i64>();

        let mut dp = vec![vec![NegExtInt::NEG_INF; x as usize + 1]; n + 1];

        let mut ok = 0;
        let mut ng = sum_vit + 1;

        while num::abs(ok - ng) > 1 {
            let mid = (ok + ng) / 2;
            assert!(mid != ok);
            assert!(mid != ng);
            let vit = mid;
            let mut sum_cal = 0;
            for t in 0..3 {
                let foods = &type_to_foods[t];
                let sub = solve_sub(foods, vit, &mut dp, x).unwrap_or(i64::MAX / 10);
                sum_cal += sub;
            }
            // dbg!(vit);
            // dbg!(sum_cal);

            if sum_cal <= x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let ans = ok;

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // DP 初期化を1回だけ(solve2 のリファクタリング)

        /// カロリーの最小値
        fn solve_sub(
            foods: &[Food],
            vit: i64,
            x: i64,
            dp: &mut Vec<Vec<NegExtInt>>,
        ) -> Option<i64> {
            let n = foods.len();
            for cal in 0..=x as usize {
                dp[0][cal] = NegExtInt::fin(0);
            }

            for i in 0..n {
                let food = foods[i];
                for cal in 0..=x {
                    let choose = if cal < food.cal {
                        NegExtInt::NEG_INF
                    } else {
                        dp[i][(cal - food.cal) as usize] + food.vit
                    };

                    let no_choose = dp[i][cal as usize];

                    dp[i + 1][cal as usize] = NegExtInt::max(choose, no_choose);
                }
            }

            // dp[n][cal] == ビタミン最大値
            // dp[n][cal] >= bit となるような cal の最小値を求める

            (0..=x).find(|&cal| dp[n][cal as usize] >= NegExtInt::fin(vit))
        }
        let n = self.n;

        let x = self.x;
        let foods = &self.foods;
        let type_to_foods = foods.iter().fold(vec![vec![]; 3], |mut acc, food| {
            acc[food.t].push(*food);
            acc
        });

        let sum_vit = foods.iter().map(|f| f.vit).sum::<i64>();

        let mut dp = vec![vec![NegExtInt::NEG_INF; x as usize + 1]; n + 1];

        let ans = bin_search(0, sum_vit + 1, |vit| {
            // それぞれ vit 以上のビタミンを取る。そのとき、カロリーの最小値 が x 以下か

            let sum_cal = (0..3)
                .map(|t| {
                    let foods = &type_to_foods[t];
                    solve_sub(foods, vit, x, &mut dp).unwrap_or(i64::MAX / 10)
                })
                .sum::<i64>();
            // dbg!(vit);
            // dbg!(sum_cal);
            sum_cal <= x
        });

        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // next DP にした

        /// カロリーの最小値
        fn solve_sub(foods: &[Food], vit: i64, x: i64) -> Option<i64> {
            let n = foods.len();
            let mut dp = vec![NegExtInt::NEG_INF; x as usize + 1];
            for cal in 0..=x as usize {
                dp[cal] = NegExtInt::fin(0);
            }

            for i in 0..n {
                let mut next_dp = vec![NegExtInt::NEG_INF; x as usize + 1];
                let food = foods[i];
                for cal in 0..=x {
                    let choose = if cal < food.cal {
                        NegExtInt::NEG_INF
                    } else {
                        dp[(cal - food.cal) as usize] + food.vit
                    };

                    let no_choose = dp[cal as usize];

                    next_dp[cal as usize] = std::cmp::max(choose, no_choose);
                }
                dp = next_dp;
            }

            // dp[cal] == ビタミン最大値
            // dp[cal] >= bit となるような cal の最小値を求める

            (0..=x).find(|&cal| dp[cal as usize] >= NegExtInt::fin(vit))
        }

        let x = self.x;
        let foods = &self.foods;
        let type_to_foods = foods.iter().fold(vec![vec![]; 3], |mut acc, food| {
            acc[food.t].push(*food);
            acc
        });

        let sum_vit = foods.iter().map(|f| f.vit).sum::<i64>();

        let ans = bin_search(0, sum_vit + 1, |vit| {
            // それぞれ vit 以上のビタミンを取る。そのとき、カロリーの最小値 が x 以下か

            let sum_cal = (0..3)
                .map(|t| {
                    let foods = &type_to_foods[t];
                    solve_sub(foods, vit, x).unwrap_or(i64::MAX / 10)
                })
                .sum::<i64>();
            sum_cal <= x
        });

        Answer { ans }
    }

    fn solve5(&self) -> Answer {
        // DP を二分探索の外に出す
        // 計算量: O(NX + X log Σ A)
        // （二分探索の中で DP をした場合の計算量は O(NX log Σ A) であった）

        /// カロリーごとのビタミン摂取量の最大値
        fn max_vit_by_cal(foods: &[Food], x: i64) -> Vec<NegExtInt> {
            let n = foods.len();
            let mut dp = vec![NegExtInt::NEG_INF; x as usize + 1];
            for cal in 0..=x as usize {
                dp[cal] = NegExtInt::fin(0);
            }

            for i in 0..n {
                let mut next_dp = vec![NegExtInt::NEG_INF; x as usize + 1];
                let food = foods[i];
                for cal in 0..=x {
                    let choose = if cal < food.cal {
                        NegExtInt::NEG_INF
                    } else {
                        dp[(cal - food.cal) as usize] + food.vit
                    };

                    let no_choose = dp[cal as usize];

                    next_dp[cal as usize] = std::cmp::max(choose, no_choose);
                }
                dp = next_dp;
            }

            dp
        }

        let x = self.x;
        let foods = &self.foods;
        let type_to_foods = foods.iter().fold(vec![vec![]; 3], |mut acc, food| {
            acc[food.t].push(*food);
            acc
        });

        let max_vit_by_cal_by_food = type_to_foods
            .iter()
            .map(|foods| max_vit_by_cal(foods, x))
            .collect_vec();

        let sum_vit = foods.iter().map(|f| f.vit).sum::<i64>();

        let ans = bin_search(0, sum_vit + 1, |vit| {
            // それぞれのビタミンを vit 以上取ったときのカロリーの最小値をX以下にできる？
            let sum_cal = (0..3)
                .map(|i| {
                    let max_vit_by_cal = &max_vit_by_cal_by_food[i];
                    max_vit_by_cal
                        .iter()
                        .position(|v| *v >= fin(vit))
                        .map(|x| x as i64)
                        .unwrap_or(i64::MAX / 10)
                })
                .sum::<i64>();
            sum_cal <= x
        });
        Answer { ans }
    }

    fn solve6(&self) -> Answer {
        // 二分探索をせずに DP & 貪欲法をする（想定解法）
        // 計算量: O(NX)

        // todo: max_vit_within_cal って名前にする。

        /// カロリーごとのビタミン摂取量の最大値
        /// 戻り値を dp としたとき
        /// dp[cal] = カロリー摂取量 cal 以下となるように食べ物 foods を食べたときの得られるビタミンの最大値
        /// である
        fn max_vit_by_cal(foods: &[Food], max_cal: i64) -> Vec<NegExtInt> {
            let n = foods.len();
            let mut dp = vec![NegExtInt::NEG_INF; max_cal as usize + 1];
            for cal in 0..=max_cal as usize {
                dp[cal] = NegExtInt::fin(0);
            }

            for i in 0..n {
                let mut next_dp = vec![NegExtInt::NEG_INF; max_cal as usize + 1];
                let food = foods[i];
                for cal in 0..=max_cal {
                    let choose = if cal < food.cal {
                        NegExtInt::NEG_INF
                    } else {
                        dp[(cal - food.cal) as usize] + food.vit
                    };

                    let no_choose = dp[cal as usize];

                    next_dp[cal as usize] = std::cmp::max(choose, no_choose);
                }
                dp = next_dp;
            }

            dp
        }

        let x = self.x;
        let foods = &self.foods;
        let type_to_foods = foods.iter().fold(vec![vec![]; 3], |mut acc, food| {
            acc[food.t].push(*food);
            acc
        });

        let max_vit_by_cal_by_food = type_to_foods
            .iter()
            .map(|foods| max_vit_by_cal(foods, x))
            .collect_vec();
        todo!();
        let ans = 0;

        Answer { ans }
    }
    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
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
    Problem::read().solve6().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        let foods = [
            Food {
                t: 1,
                vit: 3,
                cal: 5,
            },
            Food {
                t: 1,
                vit: 7,
                cal: 10,
            },
        ];
        // assert_eq!(solve_sub(&foods, 8, 25), Some(15));
        // assert_eq!(solve_sub(&foods, 7, 25), Some(10));
        // assert_eq!(solve_sub(&foods, 4, 25), Some(10));
        // assert_eq!(solve_sub(&foods, 3, 25), Some(5));
        // assert_eq!(solve_sub(&foods, 2, 25), Some(5));
        // assert_eq!(solve_sub(&foods, 0, 25), Some(0));
        //dbg!(ans);
        assert_eq!(1 + 1, 2);
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve1();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
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
/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// 計算量: O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    debug_assert!(ok != ng);
    debug_assert!(ok.checked_sub(ng).is_some());
    debug_assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        debug_assert!(mid != ok);
        debug_assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
use mod_neg_ext_int::*;
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign},
    };
    pub const NEG_INF: NegExtInt = NegExtInt::NEG_INF;
    pub fn fin(x: i64) -> NegExtInt {
        NegExtInt::fin(x)
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct NegExtInt(i64);
    impl NegExtInt {
        pub const NEG_INF: Self = Self(i64::MIN);
        pub fn fin(x: i64) -> Self {
            Self(x)
        }
        pub fn get_fin(self) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                panic!("called `NegExtInt::get_fin()` on a `NegInf` value")
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                default
            }
        }
        #[inline]
        pub fn is_fin(self) -> bool {
            self.0 != i64::MIN
        }
        pub fn is_neg_inf(self) -> bool {
            self.0 == i64::MIN
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() {
                Some(self.0)
            } else {
                None
            }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Self(a),
                None => Self::NEG_INF,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Self(0),
                Ordering::Greater => {
                    if self.is_fin() {
                        Self(self.0 * t)
                    } else {
                        Self::NEG_INF
                    }
                }
            }
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            if self.is_neg_inf() || rhs.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 + rhs.0)
            }
        }
    }
    impl AddAssign for NegExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            if self.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 + rhs)
            }
        }
    }
    impl AddAssign<i64> for NegExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl std::iter::Sum for NegExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                if x.is_neg_inf() {
                    return Self::NEG_INF;
                }
                s += x.0;
            }
            Self::fin(s)
        }
    }
    impl fmt::Display for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_neg_inf() {
                write!(f, "-∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    impl fmt::Debug for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_neg_inf() {
                write!(f, "-∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegExtInt::fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegExtInt::NEG_INF
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
}
