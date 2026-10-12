// 問題文と制約は読みましたか？
// #[fastout]
define_queries! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Query: usize {
        1 => Add { x: i64 },
        2 => Remove { x: i64 },
        3 => Sum {  },
    }
}
fn main() {
    input! {
        n: usize,
        nq: usize,
        xs: [i64; n],
        qs: [Query; nq]
    }

    let mut space = vec![];
    for &x in &xs {
        space.push(x)
    }

    for &q in &qs {
        match q {
            Query::Add { x } => space.push(x),
            Query::Remove { x } => space.push(x),
            Query::Sum {} => {}
        }
    }

    let cc = CoordinateCompression::new(&space);
    // 0乗和
    // 1乗和

    use ac_library::ModInt998244353 as Mint;

    let mut seg0: RangeSumSegtree<Mint> = RangeSumSegtree::new(cc.space_size());
    let mut seg1: RangeSumSegtree<Mint> = RangeSumSegtree::new(cc.space_size());
    let mut sum = xs
        .iter()
        .copied()
        .sorted()
        .enumerate()
        .map(|(i, x)| Mint::new((i + 1) as i64 * x))
        .sum::<Mint>();
    for &x in &xs {
        seg0.add(cc.compress(x), Mint::new(1));
        seg1.add(cc.compress(x), Mint::new(x));
    }

    for q in qs {
        match q {
            Query::Add { x } => {
                let ccx = cc.compress(x);
                seg0.add(ccx, Mint::new(1));
                seg1.add(ccx, Mint::new(x));
                let diff = Mint::new(x) * seg0.range_sum(..=ccx) + seg1.range_sum(ccx + 1..);
                sum += diff;
            }
            Query::Remove { x } => {
                let ccx = cc.compress(x);

                let diff = Mint::new(x) * seg0.range_sum(..=ccx) + seg1.range_sum(ccx + 1..);
                sum -= diff;
                seg0.add(ccx, Mint::new(-1));
                seg1.add(ccx, Mint::new(-x));
            }
            Query::Sum {} => {
                println!("{}", sum);
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
use range_sum_segtree::*;
#[allow(clippy::module_inception)]
pub mod range_sum_segtree {
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::Sum;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};
    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }
    /// 汎用的な加算モノイド。
    ///
    /// `std::ops::Add` と `std::iter::Sum` を実装している型に対応。
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct GeneralAdditive<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for GeneralAdditive<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        type S = T;
        #[inline]
        fn identity() -> Self::S {
            zero()
        }
        #[inline]
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    /// ACL の Segtree を使用した区間和セグメント木。
    ///
    /// 数値型 T に対して点更新・区間和取得を行う。
    #[derive(Clone)]
    pub struct RangeSumSegtree<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        segtree: Segtree<GeneralAdditive<T>>,
        len: usize,
    }
    impl<T> RangeSumSegtree<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        /// 単位元で初期化されたセグメント木を構築する
        pub fn new(n: usize) -> Self {
            Self {
                segtree: Segtree::<GeneralAdditive<T>>::new(n),
                len: n,
            }
        }
        /// 配列からセグメント木を構築する
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: Segtree::<GeneralAdditive<T>>::from(xs.to_vec()),
                len,
            }
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        /// p 番目の要素を x に更新する
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }
        /// p 番目の要素を取得する
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p)
        }
        /// 指定した範囲の和を取得する
        pub fn range_sum<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }
        /// 全要素の和を取得する
        pub fn all_sum(&self) -> T {
            self.segtree.all_prod()
        }
        /// p 番目の要素に x を加算する
        pub fn add(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, current + x);
        }
        /// セグメント木上の二分探索。
        ///
        /// [l, r) の和 s について f(&s) が true となる最大の r を返す。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * 加法の単位元に対して `f` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.max_right(l, f)
        }
        /// セグメント木上の二分探索。
        ///
        /// [l, r) の和 s について f(&s) が true となる最小の l を返す。
        ///
        /// # 前提条件
        /// * `r <= n`
        /// * 加法の単位元に対して `f` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.min_left(r, f)
        }
        /// 現在の状態を Vec として返す
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
