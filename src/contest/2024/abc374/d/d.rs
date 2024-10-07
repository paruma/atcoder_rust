//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    s: i64,
    t: i64,
    segs: Vec<Seg>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Seg {
    src: Pos<i64>,
    dst: Pos<i64>,
}

impl Seg {
    fn new(src: Pos<i64>, dst: Pos<i64>) -> Self {
        Seg { src, dst }
    }
    fn rev(self) -> Seg {
        Seg {
            src: self.dst,
            dst: self.src,
        }
    }
}

fn dist(p1: Pos<i64>, p2: Pos<i64>) -> f64 {
    let d = p2 - p1;
    ((d.x * d.x + d.y * d.y) as f64).sqrt()
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            s: i64,
            t: i64,
            xs: [(i64, i64, i64, i64); n],
        }

        let segs = xs
            .iter()
            .copied()
            .map(|(x1, y1, x2, y2)| Seg::new(Pos::new(x1, y1), Pos::new(x2, y2)))
            .collect_vec();
        Problem { n, s, t, segs }
    }

    fn solve(&self) -> Answer {
        // 向きを決める
        // 順番を決める

        let n = self.n;
        let s = self.s as f64;
        let t = self.t as f64;
        let segs = &self.segs;
        let ans = (0..n)
            .powerset()
            .map(|norm_set| {
                let norm_set = norm_set.iter().copied().collect::<HashSet<_>>();

                let segs = (0..n)
                    .map(|i| {
                        if norm_set.contains(&i) {
                            segs[i]
                        } else {
                            segs[i].rev()
                        }
                    })
                    .collect_vec();

                (0..n)
                    .permutations(n)
                    .map(|ord| {
                        // 始点(0,0) → 最初の点
                        let mut current = Pos::new(0, 0);
                        let mut sum = 0.0;

                        for &i in &ord {
                            sum += dist(current, segs[i].src) / s;
                            sum += dist(segs[i].src, segs[i].dst) / t;
                            current = segs[i].dst;
                        }
                        sum
                    })
                    .min_by(f64::total_cmp)
                    .unwrap()
            })
            .min_by(f64::total_cmp)
            .unwrap();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 添字を使わないようにリファクタリング

        let n = self.n;
        let s = self.s as f64;
        let t = self.t as f64;
        let segs = &self.segs;

        let ans = segs
            .iter()
            .copied()
            .map(|s| [s, s.rev()])
            .multi_cartesian_product()
            .map(|segs| {
                segs.iter()
                    .copied()
                    .permutations(n)
                    .map(|segs| {
                        // 始点(0,0) → 最初の点
                        let mut current = Pos::new(0, 0);
                        let mut sum = 0.0;

                        for seg in segs {
                            sum += dist(current, seg.src) / s;
                            sum += dist(seg.src, seg.dst) / t;
                            current = seg.dst;
                        }
                        sum
                    })
                    .min_by(f64::total_cmp)
                    .unwrap()
            })
            .min_by(f64::total_cmp)
            .unwrap();

        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Answer {
    ans: f64,
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
use core::f64;
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
use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }
    impl<T: Mul<Output = T> + Copy> Pos<T> {
        pub fn scala_mul(self, rhs: T) -> Pos<T> {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl<T: Add<Output = T> + Mul<Output = T> + Copy> Pos<T> {
        pub fn inner_product(self, rhs: Self) -> T {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> T {
            self.inner_product(self)
        }
    }
    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl<T: Neg<Output = T>> Neg for Pos<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl<T: Add<Output = T> + Copy> AddAssign for Pos<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl<T: Sub<Output = T> + Copy> SubAssign for Pos<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    pub const DIR8_LIST: [Pos<i64>; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos<i64>; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
    impl Pos<i64> {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR4_LIST.iter().copied().map(move |d| d + self)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR8_LIST.iter().copied().map(move |d| d + self)
        }
    }
}
