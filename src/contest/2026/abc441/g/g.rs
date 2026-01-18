use ac_library::LazySegtree;
use map_monoid_template::*;
#[allow(unused_variables)]
#[allow(clippy::module_inception)]
pub mod map_monoid_template {
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub len: usize,
        pub cnty1: usize,
        pub max_x: i64, // y=1 のときの x のmax
    }
    impl RangeXxx {
        pub fn unit(x: i64, y: i64) -> Self {
            let len = 1;
            let cnty1 = (y == 1) as usize;
            let max_x = if y == 0 { 0 } else { x };
            Self {
                len: 1,
                cnty1,
                max_x,
            }
        }
    }
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx {
                len: 0,
                cnty1: 0,
                max_x: i64::MIN / 10,
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeXxx {
                len: a.len + b.len,
                cnty1: a.cnty1 + b.cnty1,
                max_x: i64::max(a.max_x, b.max_x),
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Action {
        pub a: i64,
        pub b: i64,
        pub y_flip: i64,
    }
    pub struct RangeYyyRangeXxx(Infallible);
    impl MapMonoid for RangeYyyRangeXxx {
        type M = RangeXxxMonoid;
        type F = Action;
        fn identity_map() -> Self::F {
            Action {
                a: 1,
                b: 0,
                y_flip: 0,
            }
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            if f.y_flip == 1 {
                RangeXxx {
                    len: x.len,
                    cnty1: x.len - x.cnty1,
                    max_x: 0,
                }
            } else {
                if x.cnty1 == 0 {
                    RangeXxx {
                        len: x.len,
                        cnty1: x.cnty1,
                        max_x: i64::MIN / 10,
                    }
                } else {
                    RangeXxx {
                        len: x.len,
                        cnty1: x.cnty1,
                        max_x: f.b + f.a * x.max_x,
                    }
                }
            }
        }
        fn composition(f1: &Self::F, f2: &Self::F) -> Self::F {
            if f1.y_flip == 1 {
                Self::F {
                    a: f1.a,
                    b: f1.b,
                    y_flip: (f1.y_flip + f2.y_flip) % 2,
                }
            } else {
                Self::F {
                    a: f1.a * f2.a,
                    b: f1.a * f2.b + f1.b,
                    y_flip: (f1.y_flip + f2.y_flip) % 2,
                }
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // 1: 表, 0: 裏
    let mut seg = LazySegtree::<RangeYyyRangeXxx>::from(vec![RangeXxx::unit(0, 1); n]);

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                l: Usize1,
                r: Usize1,
                x: i64,
            }
            seg.apply_range(
                l..=r,
                Action {
                    a: 1,
                    b: x,
                    y_flip: 0,
                },
            );
        } else if t == 2 {
            input! {
                l: Usize1,
                r: Usize1,
            }
            seg.apply_range(
                l..=r,
                Action {
                    a: 0,
                    b: 0,
                    y_flip: 1,
                },
            );
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            let ans = seg.prod(l..=r).max_x.max(0);
            println!("{}", ans);
        }
    }
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

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
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
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
