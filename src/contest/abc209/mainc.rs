#![allow(clippy::let_unit_value)]

use num::{One, Zero};
use proconio::input;
pub mod rf {
    pub const MOD: i64 = 1_000_000_007;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RF {
        rep: i64,
    }
    impl RF {
        pub fn new(x: i64) -> RF {
            RF {
                rep: x.rem_euclid(MOD),
            }
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

use rf::*;
fn read() -> (i64, Vec<i64>) {
    input! {n: i64, c: [i64;n]}
    (n, c)
}

fn solve(_n: i64, c: &[i64]) -> RF {
    let mut c = c.to_vec();
    c.sort();

    c.iter()
        .enumerate()
        .map(|(i, x)| {
            let i = i as i64;
            if *x - i <= 0 {
                RF::zero()
            } else {
                RF::new(x - i)
            }
        })
        .fold(RF::one(), |acc, x| acc * x)
}

//fn output() {}

fn main() {
    let (n, c) = read();
    let ans = solve(n, &c);
    println!("{}", ans.rep());
}
