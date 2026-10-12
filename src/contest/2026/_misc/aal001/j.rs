// 問題文と制約は読みましたか？
// #[fastout]

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
        pub cnt0: i64,
        pub cnt1: i64,
    }
    impl RangeXxx {
        pub fn unit(cnt0: i64, cnt1: i64) -> Self {
            Self { cnt0, cnt1 }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx { cnt0: 0, cnt1: 0 }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeXxx {
                cnt0: a.cnt0 + b.cnt0,
                cnt1: a.cnt1 + b.cnt1,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeYyyRangeXxx(Infallible);
    impl MapMonoid for RangeYyyRangeXxx {
        type M = RangeXxxMonoid;
        type F = bool;
        fn identity_map() -> Self::F {
            false
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            if *f {
                RangeXxx {
                    cnt0: x.cnt1,
                    cnt1: x.cnt0,
                }
            } else {
                *x
            }
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f ^ g
        }
    }
}

define_queries! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Query: usize {
        1 => Flip { l: i64, r: i64 },
        2 => Count { l: i64, r: i64 },
    }
}
fn main() {
    input! {
        nq: usize,
    }

    let qs = (0..nq)
        .map(|_| {
            input! {
                t: usize,
            }

            if t == 1 {
                input! {
                    l: i64,
                    r: i64,
                }
                Query::Flip { l, r: r + 1 }
            } else {
                input! {
                    l: i64,
                    r: i64,
                }
                Query::Count { l, r: r + 1 }
            }
        })
        .collect_vec();

    let space = qs
        .iter()
        .copied()
        .flat_map(|q| match q {
            Query::Flip { l, r } => [l, r],
            Query::Count { l, r } => [l, r],
        })
        .collect_vec();
    let space = space.iter().copied().sorted().dedup().collect_vec();

    let cc = CoordinateCompression::new(&space);
    let cnt0s = space
        .iter()
        .copied()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    dbg!(&cnt0s);

    let mut seg = LazySegtree::<RangeYyyRangeXxx>::from(
        cnt0s
            .iter()
            .copied()
            .map(|cnt0| RangeXxx::unit(cnt0, 0))
            .collect_vec(),
    );

    for &q in &qs {
        match q {
            Query::Flip { l, r } => {
                let ccl = cc.compress(l);
                let ccr = cc.compress(r);
                seg.apply_range(ccl..ccr, true);
            }
            Query::Count { l, r } => {
                let ccl = cc.compress(l);
                let ccr = cc.compress(r);
                let cnt1 = seg.prod(ccl..ccr).cnt1;
                println!("{}", cnt1);
            }
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
#[macro_use]
pub mod define_queries {
    /// クエリ形式の入力を proconio::input! で読み込める enum を定義するマクロ。
    ///
    /// 出典： <https://zenn.dev/magurofly/articles/6ee845bd5e385e>
    ///
    /// # 利用例
    ///
    /// ```
    /// use mylib::define_queries;
    /// use proconio::marker::Usize1;
    ///
    /// define_queries! {
    ///     #[derive(Debug, PartialEq)]
    ///     enum Query: usize {
    ///         1 => Add { a: i64, b: i64 },
    ///         2 => Show { k: Usize1 },
    ///     }
    /// }
    /// ```
    #[macro_export]
    macro_rules ! define_queries {($ ($ (# [$ attr : meta ] ) * enum $ enum_name : ident : $ sig : ty {$ ($ pattern : pat => $ variant : ident $ ({$ ($ name : ident : $ marker : ty $ (, ) ? ) ,* } ) ? $ (, ) ? ) ,* } ) * ) => {$ ($ (# [$ attr ] ) * enum $ enum_name {$ ($ variant $ ({$ ($ name : <$ marker as proconio :: source :: Readable >:: Output ) ,* } ) ? ) ,* } impl proconio :: source :: Readable for $ enum_name {type Output = Self ; fn read < R : std :: io :: BufRead , S : proconio :: source :: Source < R >> (source : & mut S ) -> Self {#! [allow (unreachable_patterns ) ] match <$ sig as proconio :: source :: Readable >:: read (source ) {$ ($ pattern => $ enum_name ::$ variant $ ({$ ($ name : <$ marker as proconio :: source :: Readable >:: read (source ) ) ,* } ) ? ) ,* , _ => unreachable ! () } } } ) * } }
}
use coordinate_compression::*;
#[allow(clippy::module_inception)]
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;
    #[derive(Debug, Clone)]
    pub struct CoordinateCompression<T> {
        space: Vec<T>,
    }
    impl<T: Ord + Copy> CoordinateCompression<T> {
        /// 与えられた要素から座標圧縮空間を構築する。
        ///
        /// # 計算量
        /// O(N log N) (N = |space|)
        pub fn new(space: &[T]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }
        /// 与えられた値を座標圧縮したインデックスを返す。
        ///
        /// 値が空間に存在しない場合はパニックする。
        ///
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress(&self, x: T) -> usize {
            self.space.binary_search(&x).unwrap()
        }
        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        ///
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_floor(&self, x: T) -> usize {
            self.space.upper_bound(&x) - 1
        }
        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        ///
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_ceil(&self, x: T) -> usize {
            self.space.lower_bound(&x)
        }
        /// 与えられた各要素を座標圧縮した結果を返す。
        ///
        /// # 計算量
        /// O(M log N) (M = |xs|, N = space_size)
        pub fn compress_vec(&self, xs: &[T]) -> Vec<usize> {
            xs.iter().map(|&x| self.compress(x)).collect_vec()
        }
        /// 指定された範囲内の値に対応する座標圧縮後のインデックス範囲を [begin, end) で返す。
        ///
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_range(
            &self,
            range: impl std::ops::RangeBounds<T>,
        ) -> std::ops::Range<usize> {
            use std::ops::Bound::*;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => self.space.lower_bound(&x),
                Excluded(&x) => self.space.upper_bound(&x),
            };
            let end = match range.end_bound() {
                Unbounded => self.space.len(),
                Included(&x) => self.space.upper_bound(&x),
                Excluded(&x) => self.space.lower_bound(&x),
            };
            begin..end
        }
        /// 座標圧縮されたインデックスから元の値を復元する。
        ///
        /// # 計算量
        /// O(1)
        pub fn decompress(&self, i: usize) -> T {
            self.space[i]
        }
        /// 座標圧縮後の空間の大きさ（要素数）を返す。
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
    }
}
