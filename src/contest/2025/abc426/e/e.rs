fn solve1(start: Pos, goal: Pos) -> f64 {
    let coef = Pos::inner_product(goal - start, -start) / ((goal - start).norm_square());

    if 0.0 <= coef && coef <= 1.0 {
        // start と goal を通る直線の方程式

        let a = start.y - goal.y;
        let b = -(start.x - goal.x);
        let c = start.x * goal.y - goal.x * start.y;

        return c.abs() / f64::sqrt((a * a) + (b * b));
    }

    let d2 = f64::sqrt(start.norm_square());
    let d3 = f64::sqrt(goal.norm_square());

    f64::min(d2, d3)
}

fn solve2(start: Pos, goal: Pos) -> f64 {
    if start == goal {
        return 0.0;
    }
    let coef = Pos::inner_product(goal - start, -start) / ((goal - start).norm_square());

    if 0.0 <= coef && coef <= 1.0 {
        // start と goal を通る直線の方程式

        let a = start.y - goal.y;
        let b = -(start.x - goal.x);
        let c = start.x * goal.y - goal.x * start.y;

        return c.abs() / f64::sqrt((a * a) + (b * b));
    }

    let d2 = f64::sqrt(start.norm_square());
    let d3 = f64::sqrt(goal.norm_square());

    f64::min(d2, d3)
}

fn solve0(start1: Pos, goal1: Pos, start2: Pos, goal2: Pos) -> f64 {
    let start = start2 - start1;
    let goal = goal2 - goal1;
    solve2(start, goal)
}

fn solve(start1: Pos, goal1: Pos, start2: Pos, goal2: Pos) -> f64 {
    if (goal1 - start1).norm_square() > (goal2 - start2).norm_square() {
        return solve(start2, goal2, start1, goal1);
    }
    let dist1 = (goal1 - start1).norm_square().sqrt();
    let dist2 = (goal2 - start2).norm_square().sqrt();

    let mid2 = (start2.scala_mul(dist2 - dist1) + goal2.scala_mul(dist1)).scala_mul(1.0 / dist2);

    let ans1 = solve0(start1, goal1, start2, mid2);
    let ans2 = solve1(mid2 - goal1, goal2 - goal1);
    // dbg!(ans1);
    // dbg!(ans2);

    f64::min(ans1, ans2)
}
fn main() {
    input! {
        t: usize,
    }

    let ans: Vec<f64> = (0..t)
        .map(|_| {
            input! {
                start1: PosXY,
                goal1: PosXY,
                start2: PosXY,
                goal2: PosXY,
            }

            solve(start1, goal1, start2, goal2)
        })
        .collect_vec();
    print_vec(&ans);
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
use {core::f64, pos::*};
pub mod pos {
    use std::io::BufRead;
    use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub struct Pos {
        pub x: f64,
        pub y: f64,
    }
    impl Pos {
        pub fn new(x: f64, y: f64) -> Pos {
            Pos { x, y }
        }
    }
    impl Pos {
        pub fn scala_mul(self, rhs: f64) -> Pos {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl Pos {
        pub fn inner_product(self, rhs: Self) -> f64 {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> f64 {
            self.inner_product(self)
        }
    }
    impl Add for Pos {
        type Output = Pos;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl Sub for Pos {
        type Output = Pos;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl Neg for Pos {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }

    impl AddAssign for Pos {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for Pos {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    use std::fmt::{Debug, Error, Formatter};
    impl Debug for Pos {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
            Ok(())
        }
    }
    use proconio::source::{Readable, Source};
    pub enum PosXY {}
    impl Readable for PosXY {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let x = f64::read(source);
            let y = f64::read(source);
            Pos::new(x, y)
        }
    }
    pub enum PosYX {}
    impl Readable for PosYX {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = f64::read(source);
            let x = f64::read(source);
            Pos::new(x, y)
        }
    }
}
