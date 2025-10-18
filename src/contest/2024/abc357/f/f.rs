//#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Query {
    AddToA { l: usize, r: usize, x: i64 },
    AddToB { l: usize, r: usize, x: i64 },
    Output { l: usize, r: usize },
}

impl Query {
    fn read() -> Self {
        input! { t: usize, l: Usize1, r: Usize1 }
        match t {
            1 => {
                input! {
                    x: i64,
                }
                Query::AddToA { l, r, x }
            }
            2 => {
                input! {
                    x: i64,
                }
                Query::AddToB { l, r, x }
            }
            3 => Query::Output { l, r },
            _ => panic!(),
        }
    }
}
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    a_s: Vec<i64>,
    b_s: Vec<i64>,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            a_s: [i64; n],
            b_s: [i64; n],
        }
        let qs = (0..nq).map(|_| Query::read()).collect_vec();
        Problem { n, a_s, b_s, qs }
    }
    fn solve(&self) -> Answer {
        let ans = 0;
        use ac_library::ModInt998244353 as Mint;

        let abs = izip!(&self.a_s, &self.b_s)
            .map(|(a, b)| RangeXxx::unit(*a, *b))
            .collect_vec();
        let mut seg = LazySegtree::<RangeYyyRangeXxx>::from(abs);

        let mut ans = vec![];
        for &q in &self.qs {
            match q {
                Query::AddToA { l, r, x } => {
                    seg.apply_range(
                        l..=r,
                        Mapping {
                            x: Mint::new(x),
                            y: Mint::new(0),
                        },
                    );
                }
                Query::AddToB { l, r, x } => {
                    seg.apply_range(
                        l..=r,
                        Mapping {
                            x: Mint::new(0),
                            y: Mint::new(x),
                        },
                    );
                }
                Query::Output { l, r } => {
                    let tmp = seg.prod(l..=r).ab_sum.val() as i64;
                    ans.push(tmp);
                }
            }
        }

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
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
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

use ac_library::LazySegtree;
// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
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
use map_monoid_template::*;
#[allow(unused_variables)]
pub mod map_monoid_template {
    use ac_library::ModInt998244353 as Mint;
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub ab_sum: Mint,
        pub a_sum: Mint,
        pub b_sum: Mint,
        pub const_sum: Mint,
        pub len: usize,
    }
    impl RangeXxx {
        pub fn unit(a: i64, b: i64) -> Self {
            let a = Mint::new(a);
            let b = Mint::new(b);
            Self {
                ab_sum: a * b,
                a_sum: a,
                b_sum: b,
                const_sum: Mint::new(0),
                len: 1,
            }
        }
    }
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx {
                ab_sum: 0.into(),
                a_sum: 0.into(),
                b_sum: 0.into(),
                const_sum: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeXxx {
                ab_sum: a.ab_sum + b.ab_sum,
                a_sum: a.a_sum + b.a_sum,
                b_sum: a.b_sum + b.b_sum,
                const_sum: a.const_sum + b.const_sum,
                len: a.len + b.len,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Mapping {
        pub x: Mint,
        pub y: Mint,
    }
    pub struct RangeYyyRangeXxx(Infallible);
    impl MapMonoid for RangeYyyRangeXxx {
        type M = RangeXxxMonoid;
        type F = Mapping;
        fn identity_map() -> Self::F {
            Mapping {
                x: 0.into(),
                y: 0.into(),
            }
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            RangeXxx {
                ab_sum: x.ab_sum + f.y * x.a_sum + f.x * x.b_sum + f.x * f.y * x.len,
                a_sum: x.a_sum + f.x * x.len,
                b_sum: x.b_sum + f.y * x.len,
                const_sum: x.const_sum * f.x * f.y * x.len,
                len: x.len,
            }
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            Self::F {
                x: f.x + g.x,
                y: f.y + g.y,
            }
        }
    }
}
