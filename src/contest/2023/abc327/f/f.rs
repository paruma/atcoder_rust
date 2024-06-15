#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Apple {
    time: usize,
    pos: usize,
}

#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Basket {
    time_len: usize,
    range_len: usize,
}

fn seg_to_vec(seg: &mut RangeAffineRangeMinMaxSegtree, size: usize) -> Vec<i64> {
    (0..size).map(|i| seg.get_max(i)).collect_vec()
}

#[derive(Debug, Clone)]
struct Problem {
    n_apples: usize,
    basket: Basket,
    apples: Vec<Apple>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_apples: usize,
            basket: Basket,
            apples: [Apple; n_apples],
        }
        Problem {
            n_apples,
            basket,
            apples,
        }
    }
    fn solve(&self) -> Answer {
        let time_max = self
            .apples
            .iter()
            .copied()
            .map(|apple| apple.time)
            .max()
            .unwrap();

        let pos_max = self
            .apples
            .iter()
            .copied()
            .map(|apple| apple.pos)
            .max()
            .unwrap();

        let time_to_apples = {
            let mut time_to_apples = vec![vec![]; time_max + 1];
            for apple in &self.apples {
                time_to_apples[apple.time].push(*apple);
            }
            time_to_apples
        };

        // 各座標の値 x に対して、[x, x + self.bascket.range_len) にあるりんごの数を集計する
        let mut seg = RangeAffineRangeMinMaxSegtree::new(&vec![0; pos_max + 1]);

        let mut cand = vec![];

        for time in 0..=time_max {
            // seg から time_to_apples[time - self.basket.time_len] を削除する
            if time >= self.basket.time_len {
                for apple in &time_to_apples[time - self.basket.time_len] {
                    seg.apply_range_add(
                        (apple.pos + 1).saturating_sub(self.basket.range_len)..=apple.pos,
                        -1,
                    )
                }
            }

            // seg に time_to_apples[pos] を追加する
            for apple in &time_to_apples[time] {
                seg.apply_range_add(
                    (apple.pos + 1).saturating_sub(self.basket.range_len)..=apple.pos,
                    1,
                )
            }

            cand.push(seg.range_max(..))
        }

        let ans = cand.iter().copied().max().unwrap();
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
use range_affine_range_minmax::*;
pub mod range_affine_range_minmax {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::{cmp::Ordering, convert::Infallible, ops::RangeBounds};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMinMax {
        pub min: i64,
        pub max: i64,
        pub len: i64,
    }
    impl RangeMinMax {
        pub fn unit(x: i64) -> RangeMinMax {
            RangeMinMax {
                min: x,
                max: x,
                len: 1,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Affine {
        pub slope: i64,
        pub intercept: i64,
    }
    impl Affine {
        /// 区間変更用（定数関数）
        pub fn constant_func(x: i64) -> Affine {
            Affine {
                slope: 0,
                intercept: x,
            }
        }
        /// 区間加算用
        pub fn addition_func(x: i64) -> Affine {
            Affine {
                slope: 1,
                intercept: x,
            }
        }
    }
    pub struct RangeMinMaxMonoid(Infallible);
    impl Monoid for RangeMinMaxMonoid {
        type S = RangeMinMax;
        fn identity() -> RangeMinMax {
            RangeMinMax {
                min: INF,
                max: -INF,
                len: 0,
            }
        }
        fn binary_operation(a: &RangeMinMax, b: &RangeMinMax) -> RangeMinMax {
            RangeMinMax {
                min: Ord::min(a.min, b.min),
                max: Ord::max(a.max, b.max),
                len: a.len + b.len,
            }
        }
    }
    const INF: i64 = i64::MAX;
    pub struct RangeAffineRangeMinMax(Infallible);
    impl MapMonoid for RangeAffineRangeMinMax {
        type M = RangeMinMaxMonoid;
        type F = Affine;
        fn identity_map() -> Affine {
            Affine {
                slope: 1,
                intercept: 0,
            }
        }
        fn composition(a: &Affine, b: &Affine) -> Affine {
            Affine {
                slope: a.slope * b.slope,
                intercept: a.slope * b.intercept + a.intercept,
            }
        }
        fn mapping(f: &Affine, x: &RangeMinMax) -> RangeMinMax {
            if x.len == 0 {
                return RangeMinMaxMonoid::identity();
            }
            match f.slope.cmp(&0) {
                Ordering::Equal => RangeMinMax {
                    min: f.intercept,
                    max: f.intercept,
                    len: x.len,
                },
                Ordering::Greater => RangeMinMax {
                    min: f.intercept + f.slope * x.min,
                    max: f.intercept + f.slope * x.max,
                    len: x.len,
                },
                Ordering::Less => RangeMinMax {
                    min: f.intercept + f.slope * x.max,
                    max: f.intercept + f.slope * x.min,
                    len: x.len,
                },
            }
        }
    }
    pub struct RangeAffineRangeMinMaxSegtree {
        segtree: LazySegtree<RangeAffineRangeMinMax>,
    }
    impl RangeAffineRangeMinMaxSegtree {
        pub fn new(xs: &[i64]) -> RangeAffineRangeMinMaxSegtree {
            let xs = xs.iter().copied().map(RangeMinMax::unit).collect_vec();
            RangeAffineRangeMinMaxSegtree {
                segtree: LazySegtree::from(xs),
            }
        }
        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, RangeMinMax::unit(x));
        }
        pub fn get_min(&mut self, p: usize) -> i64 {
            self.segtree.get(p).min
        }
        pub fn get_max(&mut self, p: usize) -> i64 {
            self.segtree.get(p).max
        }
        pub fn range_min<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).min
        }
        pub fn range_max<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).max
        }
        pub fn all_min(&self) -> i64 {
            self.segtree.all_prod().min
        }
        pub fn all_max(&self) -> i64 {
            self.segtree.all_prod().max
        }
        pub fn apply_affine(&mut self, p: usize, slope: i64, intercept: i64) {
            self.segtree.apply(p, Affine { slope, intercept })
        }
        pub fn apply_update(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, Affine::constant_func(x))
        }
        pub fn apply_add(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, Affine::addition_func(x))
        }
        pub fn apply_range_affine<R>(&mut self, range: R, slope: i64, intercept: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine { slope, intercept })
        }
        pub fn apply_range_update<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::constant_func(x))
        }
        pub fn apply_range_add<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::addition_func(x))
        }
    }
}
