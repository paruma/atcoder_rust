//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
        }
        Problem { n }
    }
    fn solve(&self) -> Answer {
        // 等比数列の和の公式を使う
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let n_digits = format!("{}", n).len();
        let pow10 = Mint::new(10).pow(n_digits as u64);

        let ans = (Mint::pow(pow10, n as u64) - 1) / (pow10 - 1) * n;
        let ans = ans.val() as i64;
        Answer { ans }
    }
    fn solve2(&self) -> Answer {
        // 行列累乗を使う (この解法だと素数 mod 以外であっても使える)
        // 「10^(n_digits) 倍して n を足す」をn回繰り返す
        // x[n+1] = 10^(n_digits) * x[n] + n, x[0] = 0 としたとき、x[n] が答えとなる。
        // これを行列累乗で解く。漸化式は1次関数の形で表されているので、同次座標を使うとうまくいく。
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let n_digits = format!("{}", n).len();
        type M = Matrix22Mul<Mint>;
        let matrix = M::pow(
            &Matrix22::from_array([
                [Mint::new(10).pow(n_digits as u64), Mint::new(n)],
                [Mint::new(0), Mint::new(1)],
            ]),
            n as usize,
        );

        let ans = matrix.raw[0][1];
        let ans = ans.val() as i64;
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
    marker::{Bytes, Usize1},
};
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
use matrix22::*;
pub mod matrix22 {
    use ac_library::Monoid;
    use core::fmt::Debug;
    use std::{
        convert::Infallible,
        iter::{Product, Sum},
        marker::PhantomData,
        ops::{Add, Mul},
    };
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Matrix22<T>
    where
        T: Clone + Copy + Debug + PartialEq,
    {
        pub raw: [[T; 2]; 2],
    }
    impl<T> Matrix22<T>
    where
        T: Clone + Copy + Debug + PartialEq,
    {
        pub fn new(a00: T, a01: T, a10: T, a11: T) -> Self {
            Self {
                raw: [[a00, a01], [a10, a11]],
            }
        }
        pub fn from_array(arr: [[T; 2]; 2]) -> Self {
            Self { raw: arr }
        }
        pub fn apply(self, x: (T, T)) -> (T, T)
        where
            T: Add<Output = T> + Mul<Output = T>,
        {
            (
                self.raw[0][0] * x.0 + self.raw[0][1] * x.1,
                self.raw[1][0] * x.0 + self.raw[1][1] * x.1,
            )
        }
        fn t_zero() -> T
        where
            T: Sum,
        {
            std::iter::empty().sum()
        }
        fn t_one() -> T
        where
            T: Product,
        {
            std::iter::empty().product()
        }
        pub fn identity() -> Self
        where
            T: Sum + Product,
        {
            Matrix22::from_array([
                [Self::t_one(), Self::t_zero()],
                [Self::t_zero(), Self::t_one()],
            ])
        }
    }
    impl<T> Add for Matrix22<T>
    where
        T: Clone + Copy + Debug + PartialEq + Add<Output = T>,
    {
        type Output = Matrix22<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Matrix22::from_array([
                [
                    self.raw[0][0] + rhs.raw[0][0],
                    self.raw[0][1] + rhs.raw[0][1],
                ],
                [
                    self.raw[1][0] + rhs.raw[1][0],
                    self.raw[1][1] + rhs.raw[1][1],
                ],
            ])
        }
    }
    impl<T> Mul for Matrix22<T>
    where
        T: Clone + Copy + Debug + PartialEq + Add<Output = T> + Mul<Output = T>,
    {
        type Output = Matrix22<T>;
        fn mul(self, rhs: Self) -> Self::Output {
            Matrix22::from_array([
                [
                    self.raw[0][0] * rhs.raw[0][0] + self.raw[0][1] * rhs.raw[1][0],
                    self.raw[0][0] * rhs.raw[0][1] + self.raw[0][1] * rhs.raw[1][1],
                ],
                [
                    self.raw[1][0] * rhs.raw[0][0] + self.raw[1][1] * rhs.raw[1][0],
                    self.raw[1][0] * rhs.raw[0][1] + self.raw[1][1] * rhs.raw[1][1],
                ],
            ])
        }
    }
    pub struct Matrix22Mul<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for Matrix22Mul<T>
    where
        T: Clone + Copy + Debug + PartialEq + Sum + Product + Add<Output = T> + Mul<Output = T>,
    {
        type S = Matrix22<T>;
        fn identity() -> Self::S {
            Matrix22::identity()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (*a) * (*b)
        }
    }
}
use extend_acl_monoid::*;
pub mod extend_acl_monoid {
    use ac_library::Monoid;
    pub trait MonoidExtPow: Monoid {
        /// base^n を求める
        fn pow(base: &Self::S, n: usize) -> Self::S {
            let mut base = base.clone();
            let mut ans = Self::identity();
            let mut n = n;
            while n > 0 {
                if n & 1 == 1 {
                    ans = Self::binary_operation(&ans, &base);
                }
                base = Self::binary_operation(&base, &base);
                n >>= 1;
            }
            ans
        }
    }
    impl<T> MonoidExtPow for T where T: Monoid {}
}
