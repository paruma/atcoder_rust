#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use ndarray::{Array, Array2};
use proconio::input;

//------snippet------
use num::{One, Zero};
use rf::*;
pub mod rf {
    pub const MOD: i64 = 998_244_353;
    #[allow()]
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

//-------------------

fn read() -> (usize, Vec<i64>) {
    input! {
        //from OnceSource::from(""),
        n: usize,
        a: [i64; n]
    }
    let ret: (usize, Vec<i64>) = (n, a);
    ret
}
pub fn print_arr2(arr: &Array2<RF>) {
    for i in 0..arr.nrows() {
        for j in 0..arr.ncols() {
            print!("{:?} ", arr[[i, j]].rep());
        }
        println!();
    }
}

fn solve(n: usize, a_s: &[i64]) -> Vec<RF> {
    let mut dp: Array2<RF> = Array::from_shape_fn((n, 10), |_| RF::zero());

    for a in 0..=9 {
        dp[[0, a]] = if (a as i64) == a_s[0] {
            RF::one()
        } else {
            RF::zero()
        };
    }

    for i in 0..(n - 1) {
        for a in 0..=9 {
            //0..9って書いてた
            let next_add = ((a + a_s[i + 1]) % 10) as usize;
            let next_mul = ((a * a_s[i + 1]) % 10) as usize;

            let a = a as usize;
            let current = dp[[i, a]];
            dp[[i + 1, next_add]] += current;
            dp[[i + 1, next_mul]] += current;
            //print_arr2(&dp);
        }
    }

    //print_arr2(&dp);
    (0..=9_usize).map(|a| dp[[n - 1, a]]).collect_vec()
}

fn output(ans: &[RF]) {
    for a in ans {
        println!("{}", a.rep());
    }
}

fn main() {
    let (n, a) = read();
    let ans = solve(n, &a);
    output(&ans);
    //println!("{}", ans.rep());
}
