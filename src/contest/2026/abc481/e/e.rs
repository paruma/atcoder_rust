// #[fastout]
fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            n: usize,
            ps: [PosXY; n],
        }

        let ps = ps
            .iter()
            .copied()
            .sorted_by(|&p1, &p2| argcmp((p1.x, p1.y), (p2.x, p2.y)))
            .collect_vec();
        let ps2 = chain!(&ps, &ps).copied().collect_vec();

        let ans = (0..n)
            .map(|begin| {
                //
                let end_inc = bin_search(begin as i64, (begin + n) as i64, |i| {
                    let i = i as usize;
                    // ps2[begin] と ps2[i] の角度が180度以下か
                    ps2[begin].outer_product(ps2[i]) >= 0
                }) as usize;
                end_inc - begin + 1
            })
            .max()
            .unwrap();

        println!("{}", ans);
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
// 使用したテンプレート・スニペットの公開元:
// https://github.com/paruma/atcoder_rust
use pos::*;
#[allow(clippy::module_inception)]
pub mod pos {
    use std::io::BufRead;
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Pos {
        pub x: i64,
        pub y: i64,
    }

    impl Pos {
        pub fn new(x: i64, y: i64) -> Pos {
            Pos { x, y }
        }

        pub fn new_from_usize(x: usize, y: usize) -> Pos {
            Pos::new(x as i64, y as i64)
        }

        pub fn scalar_mul(self, rhs: i64) -> Pos {
            Pos::new(self.x * rhs, self.y * rhs)
        }

        pub fn inner_product(self, rhs: Self) -> i64 {
            self.x * rhs.x + self.y * rhs.y
        }

        pub fn outer_product(self, rhs: Self) -> i64 {
            self.x * rhs.y - self.y * rhs.x
        }

        pub fn norm_square(self) -> i64 {
            self.inner_product(self)
        }

        pub fn l1_norm(self) -> i64 {
            self.x.abs() + self.y.abs()
        }

        pub fn linf_norm(self) -> i64 {
            self.x.abs().max(self.y.abs())
        }

        pub fn dist_square(self, rhs: Self) -> i64 {
            (self - rhs).norm_square()
        }

        pub fn l1_dist(self, rhs: Self) -> i64 {
            (self - rhs).l1_norm()
        }

        pub fn linf_dist(self, rhs: Self) -> i64 {
            (self - rhs).linf_norm()
        }

        /// 向きが同じであれば同一視する正規化 (方向ベクトル)
        ///
        /// 最大公約数で割り、符号はそのまま残す。
        /// (0,0) の場合は (0,0) を返す。
        ///
        /// 計算量: O(log(min(|x|, |y|)))
        pub fn normalize_direction(self) -> Pos {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            let g = num::integer::gcd(self.x.abs(), self.y.abs());
            Pos::new(self.x / g, self.y / g)
        }

        /// 平行であれば同一視する正規化（直線の傾きを表す）
        ///
        /// 最大公約数で割り、最初の非零成分 (x が優先) が正になるように符号を統一する。
        /// (0,0) の場合は (0,0) を返す。
        ///
        /// 計算量: O(log(min(|x|, |y|)))
        pub fn normalize_slope(self) -> Pos {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            // 最初の非零成分 (x 優先) が正になるように符号を反転
            let p = if self.x < 0 || (self.x == 0 && self.y < 0) {
                -self
            } else {
                self
            };
            let g = num::integer::gcd(p.x, p.y);
            Pos::new(p.x / g, p.y / g)
        }

        // 原点を中心に反時計回りに90度回転
        pub fn rotate90(self) -> Pos {
            Pos::new(-self.y, self.x)
        }

        // 原点を中心に時計回りに90度回転
        pub fn rotate270(self) -> Pos {
            Pos::new(self.y, -self.x)
        }

        /// グリッドの幅 `width` を指定して、座標 `(x, y)` を 1次元インデックス `y * width + x` に変換する。
        pub fn to_index_1d(self, width: usize) -> usize {
            assert!(
                self.x >= 0 && self.y >= 0,
                "Pos::to_index_1d: x と y は 0 以上である必要があります。pos: ({}, {})",
                self.x,
                self.y
            );
            assert!(
                (self.x as usize) < width,
                "Pos::to_index_1d: x は width 未満である必要があります。x: {}, width: {}",
                self.x,
                width
            );
            (self.y as usize) * width + (self.x as usize)
        }

        /// 1次元インデックスとグリッドの幅 `width` から、座標 `(x, y)` を復元する。
        pub fn from_index_1d(index: usize, width: usize) -> Pos {
            Pos::new((index % width) as i64, (index / width) as i64)
        }

        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR4_LIST.iter().copied().map(move |d| self + d)
        }

        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR8_LIST.iter().copied().map(move |d| self + d)
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

    impl Sum for Pos {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Pos::new(0, 0), |acc, x| acc + x)
        }
    }

    impl<'a> Sum<&'a Pos> for Pos {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Pos::new(0, 0), |a, b| a + *b)
        }
    }

    impl num_traits::Zero for Pos {
        fn zero() -> Self {
            Pos::new(0, 0)
        }

        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
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

    impl Mul<i64> for Pos {
        type Output = Pos;

        fn mul(self, rhs: i64) -> Self::Output {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }

    impl MulAssign<i64> for Pos {
        fn mul_assign(&mut self, rhs: i64) {
            *self = *self * rhs
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosXY {}
    impl Readable for PosXY {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let x = i64::read(source);
            let y = i64::read(source);
            Pos::new(x, y)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosYX {}
    impl Readable for PosYX {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = i64::read(source);
            let x = i64::read(source);
            Pos::new(x, y)
        }
    }

    /// 1-indexed で与えられた座標(YX)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosYX1 {}
    impl Readable for PosYX1 {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = i64::read(source) - 1;
            let x = i64::read(source) - 1;
            Pos::new(x, y)
        }
    }

    pub const DIR8_LIST: [Pos; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];

    pub const DIR4_LIST: [Pos; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
}
// 参考: 整数のまま行う偏角ソート（行列式のあれです。）の実装バリエーションの検討とご紹介です。 - ブログ名 https://ngtkana.hatenablog.com/entry/2021/11/13/202103
/// x軸正の向きを0度として、反時計回りを正とする偏角で順序を決める。
///
/// (0, 0) は未考慮。
///
/// オーバーフローには注意: 座標の絶対値が 3×10^9 を超える場合はオーバーフローする可能性がある
pub fn argcmp((x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> std::cmp::Ordering {
    ((y0, x0) < (0, 0))
        .cmp(&((y1, x1) < (0, 0)))
        .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
}
/// 二分探索をする。
///
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
///
/// # 計算量
/// O(log(|ok - ng|))
///
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
///
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    debug_assert!(ok != ng);
    debug_assert!(ok.checked_sub(ng).is_some());
    debug_assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        debug_assert!(mid != ok); // |ok - ng| > 1 なので
        debug_assert!(mid != ng); // 同じく
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
