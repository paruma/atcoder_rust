#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Player {
    team: usize,
    power: usize,
}
#[derive(Debug, Clone)]

struct Problem {
    n: usize,
    ps: Vec<Player>,
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! chmin {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            ps: [Player; n],
        }
        Problem { n, ps }
    }
    fn solve(&self) -> Answer {
        // 解法: dp[何人見た？][チーム1の強さ][チーム2の強さ] = 変更人数 というDPをする
        // そのままだと MLE になるので、next DP にする
        let n = self.n;
        let ps = &self.ps;
        let power_sum = ps.iter().copied().map(|p| p.power).sum::<usize>();

        if power_sum % 3 != 0 {
            return Answer { ans: None };
        }

        // NOTE: ここは power_sum/3 + 1 で良かった（そうすると MLE が避けられる）
        let mut dp = vec![vec![usize::MAX; power_sum + 1]; power_sum + 1];

        dp[0][0] = 0;

        for i in 0..n {
            let mut next_dp = vec![vec![usize::MAX; power_sum + 1]; power_sum + 1];

            for p1 in 0..=power_sum {
                for p2 in 0..=power_sum {
                    if dp[p1][p2] == usize::MAX {
                        continue;
                    }
                    // NOTE: next_dp[p1 + ps[i].power] は配列外参照しそうでやばい（実際には continue のおかげで配列外参照は発生しない）
                    // dp配列の長さを power_sum/3 + 1 で取る場合は配列外参照のケアが必要になる
                    chmin!(
                        next_dp[p1 + ps[i].power][p2],
                        dp[p1][p2] + (ps[i].team != 1) as usize
                    );
                    chmin!(
                        next_dp[p1][p2 + ps[i].power],
                        dp[p1][p2] + (ps[i].team != 2) as usize
                    );
                    chmin!(next_dp[p1][p2], dp[p1][p2] + (ps[i].team != 3) as usize);
                }
            }
            dp = next_dp;
        }
        let ans = dp[power_sum / 3][power_sum / 3];
        let ans = if ans == usize::MAX { None } else { Some(ans) };

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解法: dp[何人見た？][チーム1の強さ][チーム2の強さ] = 変更人数 というDPをする
        // solve1 のリファクタリング
        let ps = &self.ps;
        let power_sum = ps.iter().copied().map(|p| p.power).sum::<usize>();

        if power_sum % 3 != 0 {
            return Answer { ans: None };
        }

        let mut dp = vec![vec![ExtInt::Inf; power_sum / 3 + 1]; power_sum / 3 + 1];

        dp[0][0] = Fin(0);

        for &player in ps {
            let mut next_dp = vec![vec![ExtInt::Inf; power_sum / 3 + 1]; power_sum / 3 + 1];

            for p1 in 0..=power_sum / 3 {
                for p2 in 0..=power_sum / 3 {
                    if dp[p1][p2].is_inf() {
                        continue;
                    }
                    if p1 + player.power <= power_sum / 3 {
                        chmin!(
                            next_dp[p1 + player.power][p2],
                            dp[p1][p2] + Fin((player.team != 1) as i64)
                        );
                    }
                    if p2 + player.power <= power_sum / 3 {
                        chmin!(
                            next_dp[p1][p2 + player.power],
                            dp[p1][p2] + Fin((player.team != 2) as i64)
                        );
                    }
                    chmin!(next_dp[p1][p2], dp[p1][p2] + Fin((player.team != 3) as i64));
                }
            }
            dp = next_dp;
        }
        let ans = dp[power_sum / 3][power_sum / 3];
        let ans = ans.to_option().map(|x| x as usize);

        Answer { ans }
    }

    fn solve_naive(&self) -> Answer {
        // MLE する
        let n = self.n;
        let ps = &self.ps;
        let power_sum = ps.iter().copied().map(|p| p.power).sum::<usize>();

        if power_sum % 3 != 0 {
            return Answer { ans: None };
        }

        let mut dp = vec![vec![vec![usize::MAX; power_sum + 1]; power_sum + 1]; n + 1];

        dp[0][0][0] = 0;

        for i in 0..n {
            for p1 in 0..=power_sum {
                for p2 in 0..=power_sum {
                    if dp[i][p1][p2] == usize::MAX {
                        continue;
                    }

                    chmin!(
                        dp[i + 1][p1 + ps[i].power][p2],
                        dp[i][p1][p2] + (ps[i].team != 1) as usize
                    );
                    chmin!(
                        dp[i + 1][p1][p2 + ps[i].power],
                        dp[i][p1][p2] + (ps[i].team != 2) as usize
                    );
                    chmin!(
                        dp[i + 1][p1][p2],
                        dp[i][p1][p2] + (ps[i].team != 3) as usize
                    );
                }
            }
        }
        let ans = dp[n][power_sum / 3][power_sum / 3];
        let ans = if ans == usize::MAX { None } else { Some(ans) };
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<usize>,
}

impl Answer {
    fn print(&self) {
        if let Some(ans) = self.ans {
            println!("{}", ans);
        } else {
            println!("{}", -1);
        }
    }
}

fn main() {
    Problem::read().solve2().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
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
use mod_ext_int::ExtInt::{self, *};
pub mod mod_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        iter::Sum,
        ops::{Add, AddAssign},
    };
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
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Fin(0),
                Ordering::Greater => match self {
                    Inf => Inf,
                    Fin(a) => Fin(a * t),
                },
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
    impl AddAssign for ExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            match self {
                Inf => Inf,
                Fin(a) => Fin(a + rhs),
            }
        }
    }
    impl AddAssign<i64> for ExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl Sum for ExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                match x {
                    Inf => return Inf,
                    Fin(x) => s += x,
                }
            }
            Fin(s)
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
    pub struct ExtIntAdditive(Infallible);
    impl Monoid for ExtIntAdditive {
        type S = ExtInt;
        fn identity() -> Self::S {
            Fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct ExtIntMin(Infallible);
    impl Monoid for ExtIntMin {
        type S = ExtInt;
        fn identity() -> Self::S {
            Inf
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.min(b)
        }
    }
}
