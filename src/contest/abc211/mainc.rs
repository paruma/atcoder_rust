#![allow(clippy::let_unit_value)]
use ndarray::{Array, Array2};
use num::{One, Zero};
use proconio::{input, marker::Chars};
pub mod rf {
    pub const MOD: i64 = 1_000_000_007;
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

pub mod iter_sum {
    pub trait Sum2<A>: Sized {
        fn sum2<I: Iterator<Item = A>>(iter: I) -> Self;
    }
    impl<'a, T: num::Zero + std::ops::Add<T, Output = T> + Copy> Sum2<&'a T> for T {
        fn sum2<I: Iterator<Item = &'a T>>(iter: I) -> Self {
            iter.fold(Self::zero(), |acc, x| acc + *x)
        }
    }
    impl<T: num::Zero + std::ops::Add<T, Output = T> + Copy> Sum2<T> for T {
        fn sum2<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.fold(Self::zero(), |acc, x| acc + x)
        }
    }
    pub trait IteratorExtSum2: Iterator + Sized {
        fn sum2<S>(self) -> S
        where
            Self: Sized,
            S: Sum2<Self::Item>,
        {
            Sum2::sum2(self)
        }
    }
    impl<T: Iterator> IteratorExtSum2 for T {}
}
use iter_sum::*;
use rf::*;

fn read() -> Vec<char> {
    input! {s: Chars}
    s
}

fn solve(s: Vec<char>) -> RF {
    let src = "chokudai".chars().collect::<Vec<_>>();
    let dst = s;
    let mut dp: Array2<RF> = Array::from_shape_fn((src.len() + 1, dst.len() + 1), |_| RF::zero());

    dp[[0, 0]] = RF::one();

    for src_i in 0..=src.len() {
        for dst_i in 0..=dst.len() {
            if src_i == 0 && dst_i == 0 {
                continue;
            }
            let cnt_normal = if src_i == 0 || dst_i == 0 {
                None
            } else if src[src_i - 1] == dst[dst_i - 1] {
                Some(dp[[src_i - 1, dst_i - 1]])
            } else {
                None
            };
            let cnt_insert = if dst_i == 0 {
                None
            } else {
                Some(dp[[src_i, dst_i - 1]])
            };

            dp[[src_i, dst_i]] = [cnt_normal, cnt_insert].iter().flatten().sum2::<RF>();
        }
    }
    dp[[src.len(), dst.len()]]
}

//fn output() {}

fn main() {
    let s = read();
    let ans = solve(s);
    //output();
    println!("{}", ans.rep());
}
