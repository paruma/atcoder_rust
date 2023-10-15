#[derive_readable]
struct Problem {
    a: i64,
    b: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem,
        }
        p
    }
    fn solve(&self) -> Answer {
        let fa = prime_factorize(self.a);
        let x: RF =
            fa.iter().map(|(_prime, cnt)| RF::new(*cnt) * RF::new(self.b) + RF::new(1)).product2();
        let x_mod2: i64 =
            fa.iter().map(|(_prime, cnt)| ((cnt % 2) * (self.b % 2) + 1) % 2).product2();
        //let ans = x * RF::new(self.b) / RF::new(2);
        let ans0 = x * RF::new(self.b);
        let ans0_mod2 = (x_mod2 % 2) * (self.b % 2);
        let ans = if ans0_mod2 == 0 { ans0 / RF::new(2) } else { (ans0 - RF::new(1)) / RF::new(2) };
        let ans = ans.rep();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let fa = prime_factorize(self.a);
        let x: i64 = fa.iter().map(|(_prime, cnt)| cnt * self.b + 1).product2();
        let x_mod2: i64 =
            fa.iter().map(|(_prime, cnt)| ((cnt % 2) * (self.b % 2) + 1) % 2).product2();
        let ans0_mod2 = (x_mod2 % 2) * (self.b % 2);
        let ans0 = x * self.b;
        let ans = if ans0_mod2 == 0 { ans0 / 2 } else { (ans0 - 1) / 2 };
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
        assert_eq!(Problem { a: 4, b: 1 }.solve().ans, 1);
        assert_eq!(Problem { a: 4, b: 1 }.solve2().ans, 1);
        assert_eq!(Problem { a: 2, b: 3 }.solve2().ans, 6);
        assert_eq!(1 + 1, 2);
    }
}

use std::collections::HashMap;

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
use num_integer::{Integer, Roots};
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

pub fn prime_factorize(n: i64) -> HashMap<i64, i64> {
    assert!(n >= 1);
    let mut cnt_table: HashMap<i64, i64> = HashMap::new();
    let mut n = n;
    for i in 2..=n.sqrt() {
        if n.is_multiple_of(&i) {
            // n を i で割れるだけ割る
            let mut cnt = 0;
            while n.is_multiple_of(&i) {
                n /= i;
                cnt += 1;
            }
            cnt_table.insert(i, cnt);
        }
    }
    if n != 1 {
        cnt_table.insert(n, 1);
    }
    cnt_table
}

use num::{One, Zero};
use rf::*;
pub mod rf {
    pub const MOD: i64 = 998_244_353;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RF {
        rep: i64,
    }
    impl RF {
        pub fn new(x: i64) -> RF {
            RF { rep: x.rem_euclid(MOD) }
        }
        pub fn rep(self) -> i64 {
            self.rep
        }
    }
    impl RF {
        pub fn inv(self) -> Self {
            num::pow(self, (MOD - 2) as usize)
        }
    }
    impl num_traits::Zero for RF {
        fn zero() -> Self {
            RF::new(0)
        }
        fn is_zero(&self) -> bool {
            self.rep == 0
        }
    }
    impl num_traits::One for RF {
        fn one() -> Self {
            RF::new(1)
        }
    }
    macro_rules ! bi_ops_impl {($ std_ops : ident , $ fn : ident , $ op : tt ) => {impl std :: ops ::$ std_ops for RF {type Output = Self ; fn $ fn (self , rhs : Self ) -> Self :: Output {RF :: new (self . rep $ op rhs . rep ) } } } ; }
    bi_ops_impl ! (Add , add , + );
    bi_ops_impl ! (Sub , sub , - );
    bi_ops_impl ! (Mul , mul , * );
    impl std::ops::Div for RF {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            std::ops::Mul::mul(self, rhs.inv())
        }
    }
    macro_rules ! bi_ops_assign_impl {($ std_ops_assign : ident , $ fn_assign : ident , $ op : tt ) => {impl std :: ops ::$ std_ops_assign for RF {fn $ fn_assign (& mut self , rhs : Self ) {* self = * self $ op rhs } } } ; }
    bi_ops_assign_impl ! (AddAssign , add_assign , + );
    bi_ops_assign_impl ! (SubAssign , sub_assign , - );
    bi_ops_assign_impl ! (MulAssign , mul_assign , * );
    bi_ops_assign_impl ! (DivAssign , div_assign , / );
    impl std::ops::Neg for RF {
        type Output = Self;
        fn neg(self) -> Self::Output {
            RF::new(-self.rep)
        }
    }
}

use iter_product::*;
mod iter_product {
    pub trait Product2<A>: Sized {
        fn product2<I: Iterator<Item = A>>(iter: I) -> Self;
    }
    impl<'a, T: num::One + std::ops::Mul<T, Output = T> + Copy> Product2<&'a T> for T {
        fn product2<I: Iterator<Item = &'a T>>(iter: I) -> Self {
            iter.fold(Self::one(), |acc, x| acc * (*x))
        }
    }
    impl<T: num::One + std::ops::Mul<T, Output = T> + Copy> Product2<T> for T {
        fn product2<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.fold(Self::one(), |acc, x| acc * x)
        }
    }
    pub trait IteratorExtProduct2: Iterator + Sized {
        fn product2<S>(self) -> S
        where
            Self: Sized,
            S: Product2<Self::Item>,
        {
            Product2::product2(self)
        }
    }
    impl<T: Iterator> IteratorExtProduct2 for T {}
}
