// 点 p と線分 ab の距離を求める
fn dist_point_segment(p: PosF64, a: PosF64, b: PosF64) -> f64 {
    if a == b {
        return p.dist(a);
    }
    // a を原点にする
    let ap = p - a;
    let ab = b - a;

    // ap から ab への正射影を求める
    let t = ap.inner_product(ab) / ab.norm_square();
    if t < 0.0 {
        p.dist(a)
    } else if t > 1.0 {
        p.dist(b)
    } else {
        let ah = ab * t;
        ah.dist(ap)
    }
}

// min f(x) s.t. x in [l, r] を求める（最小となる x を返す）
fn ternary_search<F>(mut l: f64, mut r: f64, mut f: F) -> f64
where
    F: FnMut(f64) -> f64,
{
    const NUM_ITERATION: i64 = 200;
    for _ in 0..NUM_ITERATION {
        let ml = (l * 2.0 + r) / 3.0;
        let mr = (l + r * 2.0) / 3.0;
        if f(ml) < f(mr) {
            r = mr;
        } else {
            l = ml;
        }
    }
    (l + r) / 2.0
}

// 点 p と線分 ab の距離を求める (三分探索を使う)
fn dist_point_segment2(p: PosF64, a: PosF64, b: PosF64) -> f64 {
    if a == b {
        return p.dist(a);
    }
    let f = |t| p.dist(a * (1.0 - t) + b * t);
    let t = ternary_search(0.0, 1.0, f);
    f(t)
}

// min dist(start1 * (1 - t) + goal1 * t,
//          start2 * (1 - t) + goal2 * t)
// s.t. t ∈ [0,1]
// を求める
fn solve0(start1: PosF64, goal1: PosF64, start2: PosF64, goal2: PosF64) -> f64 {
    let start = start2 - start1;
    let goal = goal2 - goal1;
    dist_point_segment2(PosF64::zero(), start, goal)
}

// 元の問題のソルバー
fn solve(start1: PosF64, goal1: PosF64, start2: PosF64, goal2: PosF64) -> f64 {
    let dist1 = start1.dist(goal1);
    let dist2 = start2.dist(goal2);

    if dist1 > dist2 {
        return solve(start2, goal2, start1, goal1);
    }
    assert!(dist1 <= dist2);

    // "1" がゴールに辿り着いたときの "2" の場所
    // dist1 : (dist2 - dist1) で内分した点
    let mid2 = (start2 * (dist2 - dist1) + goal2 * dist1) / dist2;

    // どっちも動いている時間での最近接距離
    let ans1 = solve0(start1, goal1, start2, mid2);
    // "1" はゴールで止まっていて、"2" のみ動いている時間での最近接距離
    let ans2 = dist_point_segment2(goal1, mid2, goal2);

    f64::min(ans1, ans2)
}
fn main() {
    input! {
        t: usize,
    }

    let ans: Vec<f64> = (0..t)
        .map(|_| {
            input! {
                start1: PosF64,
                goal1: PosF64,
                start2: PosF64,
                goal2: PosF64,
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
use {num::Zero, pos_f64::*};
#[allow(clippy::module_inception)]
pub mod pos_f64 {
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub struct PosF64 {
        pub x: f64,
        pub y: f64,
    }
    impl PosF64 {
        pub fn new(x: f64, y: f64) -> PosF64 {
            PosF64 { x, y }
        }
        pub fn scala_mul(self, rhs: f64) -> PosF64 {
            self * rhs
        }
        pub fn inner_product(self, rhs: Self) -> f64 {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn outer_product(self, rhs: Self) -> f64 {
            self.x * rhs.y - self.y * rhs.x
        }
        pub fn norm_square(self) -> f64 {
            self.inner_product(self)
        }
        pub fn norm(self) -> f64 {
            self.norm_square().sqrt()
        }
        pub fn dist(self, rhs: Self) -> f64 {
            (self - rhs).norm()
        }
        pub fn dist_square(self, rhs: Self) -> f64 {
            (self - rhs).norm_square()
        }
        pub fn rotate(self, theta: f64) -> PosF64 {
            let (s, c) = theta.sin_cos();
            PosF64::new(self.x * c - self.y * s, self.x * s + self.y * c)
        }
        pub fn normalize(self) -> PosF64 {
            self / self.norm()
        }
    }
    impl Add for PosF64 {
        type Output = PosF64;
        fn add(self, rhs: Self) -> Self::Output {
            PosF64::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl Sub for PosF64 {
        type Output = PosF64;
        fn sub(self, rhs: Self) -> Self::Output {
            PosF64::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl Neg for PosF64 {
        type Output = Self;
        fn neg(self) -> Self::Output {
            PosF64::new(-self.x, -self.y)
        }
    }
    impl Sum for PosF64 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(PosF64::new(0.0, 0.0), |acc, x| acc + x)
        }
    }
    impl<'a> Sum<&'a PosF64> for PosF64 {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(PosF64::new(0.0, 0.0), |a, b| a + *b)
        }
    }
    impl num_traits::Zero for PosF64 {
        fn zero() -> Self {
            PosF64::new(0.0, 0.0)
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl AddAssign for PosF64 {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for PosF64 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    impl Mul<f64> for PosF64 {
        type Output = PosF64;
        fn mul(self, rhs: f64) -> Self::Output {
            PosF64::new(self.x * rhs, self.y * rhs)
        }
    }
    impl MulAssign<f64> for PosF64 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = *self * rhs;
        }
    }
    impl Div<f64> for PosF64 {
        type Output = PosF64;
        fn div(self, rhs: f64) -> Self::Output {
            PosF64::new(self.x / rhs, self.y / rhs)
        }
    }
    impl DivAssign<f64> for PosF64 {
        fn div_assign(&mut self, rhs: f64) {
            *self = *self / rhs;
        }
    }
    use std::fmt::{Debug, Error, Formatter};
    impl Debug for PosF64 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
            Ok(())
        }
    }
    use proconio::source::{Readable, Source};
    use std::io::BufRead;
    impl Readable for PosF64 {
        type Output = PosF64;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> PosF64 {
            let x = f64::read(source);
            let y = f64::read(source);
            PosF64::new(x, y)
        }
    }
}
