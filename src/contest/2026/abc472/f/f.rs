// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        nq: usize,
        ps: [PosF64; n],
        qs: [(Usize1, Usize1); nq]
    }

    let ps = chain!(&ps, &ps).copied().collect_vec();

    let plus_list = ps
        .iter()
        .copied()
        .tuple_windows()
        .map(|(p, q)| p + q)
        .collect_vec();

    let mult_list = ps
        .iter()
        .copied()
        .tuple_windows()
        .map(|(p, q)| p.outer_product(q))
        .collect_vec();
    let term1_list = ps
        .iter()
        .copied()
        .tuple_windows()
        .map(|(p, q)| (p + q) / 3.0 * (p.outer_product(q)))
        .collect_vec();

    let term2_list = ps
        .iter()
        .copied()
        .tuple_windows()
        .map(|(p, q)| {
            let a = (p + q) / 3.0;
            let b = p + q;

            [-a.x * b.y, a.x * b.x, -a.y * b.y, a.y * b.x]
        })
        .collect_vec();

    let cum_plus_list = plus_list
        .iter()
        .copied()
        .scanl(PosF64::zero(), |acc, x| *acc + x)
        .collect_vec();

    let cum_mult_list = mult_list
        .iter()
        .copied()
        .scanl(0.0, |acc, x| *acc + x)
        .collect_vec();

    let cum_term1_list = term1_list
        .iter()
        .copied()
        .scanl(PosF64::zero(), |acc, x| *acc + x)
        .collect_vec();

    let cum_term2_list = term2_list
        .iter()
        .copied()
        .scanl([0.0, 0.0, 0.0, 0.0], |[a, b, c, d], [x, y, z, w]| {
            [*a + x, *b + y, *c + z, *d + w]
        })
        .collect_vec();

    for (u, v) in qs {
        let v = if v < u { v + n } else { v };

        // 面積の総和
        let area_sum = {
            let s1 = cum_mult_list[v] - cum_mult_list[u];
            let s2 = cum_plus_list[v] - cum_plus_list[u];

            s1 - s2.outer_product(ps[u])
        };

        // vec の総和
        let vec_sum = {
            let term1 = cum_term1_list[v] - cum_term1_list[u];
            let term2 = {
                let begin = cum_term2_list[u];
                let end = cum_term2_list[v];

                let diff = [
                    end[0] - begin[0],
                    end[1] - begin[1],
                    end[2] - begin[2],
                    end[3] - begin[3],
                ];

                PosF64::new(
                    diff[0] * ps[u].x + diff[1] * ps[u].y,
                    diff[2] * ps[u].x + diff[3] * ps[u].y,
                )
            };

            let term3 = ps[u] * (cum_mult_list[v] - cum_mult_list[u]) / 3.0;
            let term4 = -ps[u] * (cum_plus_list[v] - cum_plus_list[u]).outer_product(ps[u]) / 3.0;
            term1 - term2 + term3 + term4
        };

        let ans = vec_sum / area_sum;
        println!("{} {}", ans.x, ans.y);
        //
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
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
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
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
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
        pub fn scalar_mul(self, rhs: f64) -> PosF64 {
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
use scan_iter::*;
#[allow(clippy::module_inception)]
pub mod scan_iter {
    #[derive(Clone)]
    pub struct Scanl<I, B, F> {
        iter: I,
        state: Option<B>,
        f: F,
    }
    impl<I, B, F> Scanl<I, B, F> {
        fn new(iter: I, init: B, f: F) -> Scanl<I, B, F> {
            Scanl {
                iter,
                state: Some(init),
                f,
            }
        }
    }
    impl<I, B, F> Iterator for Scanl<I, B, F>
    where
        B: Copy,
        I: Iterator,
        F: FnMut(&mut B, I::Item) -> B,
    {
        type Item = B;
        #[inline]
        fn next(&mut self) -> Option<B> {
            let retval = self.state?;
            let a_opt = self.iter.next();
            self.state = self
                .state
                .and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));
            Some(retval)
        }
    }
    pub trait IteratorExtScanLeft: Iterator + Sized {
        fn scanl<B, F>(self, init: B, f: F) -> Scanl<Self, B, F>
        where
            Self: Sized,
            F: FnMut(&mut B, Self::Item) -> B,
        {
            Scanl::new(self, init, f)
        }
    }
    impl<T: Iterator> IteratorExtScanLeft for T {}
}
