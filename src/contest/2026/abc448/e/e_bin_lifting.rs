// repunit を繰り返し二乗法風に実装
// 11...11 (l個) を Mint で計算
fn repu<Mint: ModIntBase>(mut l: i64) -> Mint {
    let mut acc = Mint::new(0);
    let mut pow10 = Mint::new(10); // 10^{2^k}
    let mut repu = Mint::new(1); // 11...11 (2^k)

    while l > 0 {
        if l % 2 == 1 {
            acc = acc * pow10 + repu
        }
        repu = (pow10 + Mint::new(1)) * repu;
        pow10 = pow10 * pow10;

        l /= 2;
    }
    acc
}

// 再帰で書く方が実装しやすい
fn repu_rec<Mint: ModIntBase>(l: i64) -> Mint {
    // 11..11 (l個) と 10^l を計算する

    // f(2l) = f(l) 10^l + f(l)
    // f(2l+1) = 10^{2l} + f(l) 10^l + f(l)
    fn rec<Mint: ModIntBase>(l: i64) -> (Mint, Mint) {
        if l == 0 {
            return (Mint::new(0), Mint::new(1));
        }
        let (repu, pow10): (Mint, Mint) = rec(l / 2);
        if l % 2 == 0 {
            (repu * (pow10 + Mint::new(1)), pow10 * pow10)
        } else {
            (
                repu * (pow10 + Mint::new(1)) + pow10 * pow10,
                pow10 * pow10 * Mint::new(10),
            )
        }
    }
    rec(l).0
}
fn calc<Mint: ModIntBase + Sum + Product>(cls: &[(i64, i64)]) -> Mint {
    cls.iter().copied().fold(Mint::new(0), |acc, (c, l)| {
        acc * Mint::new(10).pow(l as u64) + repu_rec::<Mint>(l) * Mint::new(c)
    })
}

use std::iter::{Product, Sum};

use static_mod_int::*;
pub mod static_mod_int {
    use ac_library::{ButterflyCache, Modulus, StaticModInt};
    use std::{cell::RefCell, thread::LocalKey};
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod10007 {}
    impl Modulus for Mod10007 {
        const VALUE: u32 = 10007;
        const HINT_VALUE_IS_PRIME: bool = true;
        fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
            thread_local! {static BUTTERFLY_CACHE : RefCell < Option < ButterflyCache < Mod10007 >>> = RefCell :: default () ; }
            &BUTTERFLY_CACHE
        }
    }
    pub type ModInt10007 = StaticModInt<Mod10007>;
}

// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        k: usize,
        m: u32,
        cls: [(i64, i64); k],
    }
    use ac_library::ModInt as Mint1;
    Mint1::set_modulus(m);

    use ModInt10007 as Mint2;

    let x1 = calc::<Mint1>(&cls);
    let x2 = calc::<Mint2>(&cls);

    // dbg!(x1);
    // dbg!(x2);
    // dbg!(Mint1::modulus());
    // dbg!(Mint2::modulus());

    let ans: Mint2 = (x2 - Mint2::new(x1.val())) / Mint2::new(m);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        let p = 10007_u32;

        use ac_library::ModInt as Mint2;
        Mint2::set_modulus(p);
        // dbg!(repu::<Mint2>(4));
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

use ac_library::modint::ModIntBase;
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
use matrix::*;
#[allow(clippy::module_inception)]
pub mod matrix {
    use std::iter::{Product, Sum};
    use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub, SubAssign};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Matrix<T, const R: usize, const C: usize> {
        pub data: [[T; C]; R],
    }
    fn t_zero<T>() -> T
    where
        T: Sum,
    {
        std::iter::empty().sum()
    }
    fn t_one<T>() -> T
    where
        T: Product,
    {
        std::iter::empty().product()
    }
    impl<T, const R: usize, const C: usize> Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 指定された値で埋められた新しい行列を作成します。
        pub fn new(initial_value: T) -> Self {
            Self {
                data: [[initial_value; C]; R],
            }
        }
        /// 配列から行列を作成します。
        pub fn from_array(data: [[T; C]; R]) -> Self {
            Self { data }
        }
        /// スカラ倍 (Matrix * T)
        pub fn scalar_mul(self, rhs: T) -> Self {
            let mut result = Self::new(t_zero());
            for i in 0..R {
                for j in 0..C {
                    result.data[i][j] = self.data[i][j] * rhs;
                }
            }
            result
        }
        /// ベクトルを行列に適用します (行列-ベクトル積)。
        /// `self`はR行C列の行列、`x`はC要素の列ベクトルです。
        /// 結果はR要素の列ベクトルになります。
        pub fn apply(self, x: [T; C]) -> [T; R] {
            let mut result = [t_zero(); R];
            for i in 0..R {
                for j in 0..C {
                    result[i] = result[i] + self.data[i][j] * x[j];
                }
            }
            result
        }
    }
    impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
        type Output = T;
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.data[index.0][index.1]
        }
    }
    impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            &mut self.data[index.0][index.1]
        }
    }
    impl<T, const R: usize, const C: usize> Add for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            let mut result = Self::new(t_zero());
            for i in 0..R {
                for j in 0..C {
                    result.data[i][j] = self.data[i][j] + rhs.data[i][j];
                }
            }
            result
        }
    }
    impl<T, const R: usize, const C: usize> AddAssign for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + AddAssign + Sub<Output = T> + Mul<Output = T>,
    {
        fn add_assign(&mut self, rhs: Self) {
            for i in 0..R {
                for j in 0..C {
                    self.data[i][j] += rhs.data[i][j];
                }
            }
        }
    }
    impl<T, const R: usize, const C: usize> Sub for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Sub<Output = T> + Add<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            let mut result = Self::new(t_zero());
            for i in 0..R {
                for j in 0..C {
                    result.data[i][j] = self.data[i][j] - rhs.data[i][j];
                }
            }
            result
        }
    }
    impl<T, const R: usize, const C: usize> SubAssign for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + SubAssign + Add<Output = T> + Mul<Output = T>,
    {
        fn sub_assign(&mut self, rhs: Self) {
            for i in 0..R {
                for j in 0..C {
                    self.data[i][j] -= rhs.data[i][j];
                }
            }
        }
    }
    impl<T, const R: usize, const C: usize> Neg for Matrix<T, R, C>
    where
        T: Copy
            + Sum
            + Product
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Neg<Output = T>,
    {
        type Output = Self;
        fn neg(self) -> Self::Output {
            let mut data = self.data;
            for row in &mut data {
                for x in row {
                    *x = -*x;
                }
            }
            Self { data }
        }
    }
    impl<T, const R: usize, const C: usize> Sum for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::new(t_zero()), |acc, x| acc + x)
        }
    }
    impl<T, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Matrix<T, R, K>;
        fn mul(self, rhs: Matrix<T, C, K>) -> Self::Output {
            let mut result = Matrix::<T, R, K>::new(t_zero());
            for i in 0..R {
                for j in 0..K {
                    for l in 0..C {
                        result.data[i][j] = result.data[i][j] + self.data[i][l] * rhs.data[l][j];
                    }
                }
            }
            result
        }
    }
    impl<T, const R: usize, const C: usize> Mul<i64> for Matrix<T, R, C>
    where
        T: Copy
            + Sum
            + Product
            + Mul<i64, Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>,
    {
        type Output = Self;
        fn mul(self, rhs: i64) -> Self::Output {
            let mut result = Self::new(t_zero());
            for i in 0..R {
                for j in 0..C {
                    result.data[i][j] = self.data[i][j] * rhs;
                }
            }
            result
        }
    }
    impl<T, const N: usize> Matrix<T, N, N>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        pub fn pow(self, mut n: u64) -> Self {
            let mut res = Matrix::<T, N, N>::identity();
            let mut base = self;
            while n > 0 {
                if n % 2 == 1 {
                    res = res * base;
                }
                base = base * base;
                n /= 2;
            }
            res
        }
        /// 単位行列を作成します。正方行列の場合のみ有効です。
        pub fn identity() -> Self {
            let mut matrix = Self::new(t_zero());
            for i in 0..N {
                matrix.data[i][i] = t_one();
            }
            matrix
        }
    }
    impl<T> Matrix<T, 2, 2>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 2x2行列の行列式を計算します。
        pub fn det(self) -> T {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        }
    }
    impl<T> Matrix<T, 3, 3>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 3x3行列の行列式を計算します。
        pub fn det(self) -> T {
            let a = self.data[0][0];
            let b = self.data[0][1];
            let c = self.data[0][2];
            let d = self.data[1][0];
            let e = self.data[1][1];
            let f = self.data[1][2];
            let g = self.data[2][0];
            let h = self.data[2][1];
            let i = self.data[2][2];
            a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
        }
    }
    impl<T> Matrix<T, 4, 4>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 4x4行列の行列式を計算します。
        pub fn det(self) -> T {
            let m = self.data;
            let m11 = Matrix::<T, 3, 3>::from_array([
                [m[1][1], m[1][2], m[1][3]],
                [m[2][1], m[2][2], m[2][3]],
                [m[3][1], m[3][2], m[3][3]],
            ]);
            let m12 = Matrix::<T, 3, 3>::from_array([
                [m[1][0], m[1][2], m[1][3]],
                [m[2][0], m[2][2], m[2][3]],
                [m[3][0], m[3][2], m[3][3]],
            ]);
            let m13 = Matrix::<T, 3, 3>::from_array([
                [m[1][0], m[1][1], m[1][3]],
                [m[2][0], m[2][1], m[2][3]],
                [m[3][0], m[3][1], m[3][3]],
            ]);
            let m14 = Matrix::<T, 3, 3>::from_array([
                [m[1][0], m[1][1], m[1][2]],
                [m[2][0], m[2][1], m[2][2]],
                [m[3][0], m[3][1], m[3][2]],
            ]);
            m[0][0] * m11.det() - m[0][1] * m12.det() + m[0][2] * m13.det() - m[0][3] * m14.det()
        }
    }
    pub type Matrix22<T> = Matrix<T, 2, 2>;
    pub type Matrix33<T> = Matrix<T, 3, 3>;
    pub type Matrix44<T> = Matrix<T, 4, 4>;
}
