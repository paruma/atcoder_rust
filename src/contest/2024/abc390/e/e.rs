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

/// カロリーの最小値
fn solve_sub(foods: &[Food], vit: i64, dp: &mut Vec<Vec<i64>>, x: i64) -> Option<i64> {
    let n = foods.len();
    let x = foods.iter().map(|f| f.cal).sum::<i64>().min(x);

    for cal in 0..=x as usize {
        dp[0][cal] = 0;
    }

    for i in 0..n {
        let food = foods[i];
        for cal in 0..=x {
            let choose = if cal < food.cal {
                i64::MIN
            } else {
                if dp[i][(cal - food.cal) as usize] == i64::MIN {
                    i64::MIN
                } else {
                    dp[i][(cal - food.cal) as usize] + food.vit
                }
            };

            let no_choose = dp[i][cal as usize];

            dp[i + 1][cal as usize] = i64::max(choose, no_choose);
        }
    }

    // dp[n][cal] == ビタミン最大値
    // dp[n][cal] >= bit となるような cal の最小値を求める

    (0..=x).find(|&cal| dp[n][cal as usize] >= vit)
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

    fn solve(&self) -> Answer {
        let n = self.n;
        let x = self.x;
        let foods = &self.foods;
        let type_to_foods = foods.iter().fold(vec![vec![]; 3], |mut acc, food| {
            acc[food.t].push(*food);
            acc
        });

        let sum_vit = foods.iter().map(|f| f.vit).sum::<i64>();

        let mut dp = vec![vec![i64::MIN; x as usize + 1]; n + 1];

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
    Problem::read().solve().print();
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
        let main_ans = p.solve();
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
        // let n = rng.gen_range(1..=10);
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
        // let mut rng = SmallRng::from_entropy();
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
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok);
        assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
use mod_neg_ext_int::NegExtInt::{self, *};
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign},
    };
    use NegExtInt::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum NegExtInt {
        NegInf,
        Fin(i64),
    }
    impl NegExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => panic!("called `NegExtInt::get_fin()` on a `NegInf` value"),
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
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Fin(0),
                Ordering::Greater => match self {
                    NegInf => NegInf,
                    Fin(a) => Fin(a * t),
                },
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
    impl AddAssign for NegExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            match self {
                NegInf => NegInf,
                Fin(a) => Fin(a + rhs),
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
                match x {
                    NegInf => return NegInf,
                    Fin(x) => s += x,
                }
            }
            Fin(s)
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
    impl fmt::Display for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                NegInf => write!(f, "-∞"),
                Fin(x) => write!(f, "{x}"),
            }
        }
    }
    impl fmt::Debug for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                NegInf => write!(f, "-∞"),
                Fin(x) => write!(f, "{x}"),
            }
        }
    }
    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;
        fn identity() -> Self::S {
            Fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegInf
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
}
