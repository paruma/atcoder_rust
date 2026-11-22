// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: Chars,
    }
    let xs = xs
        .iter()
        .copied()
        .map(|ch| (ch as u8 - b'A') as usize)
        .collect_vec();

    let mut dp = vec![vec![vec![0_i64; 3]; 8]; n + 1];

    dp[0][0][0] = 1;
    dp[1][BitSet::empty().inserted(xs[0])][0] = 1;

    for i in 1..n {
        // dp[i+1] を決める
        for set in BitSet::all_subset(3) {
            for phase in 0..3 {
                // dp[i][set][phase] から遷移する感じ
                // フェーズ変えない
                dp[i + 1][set.inserted(xs[i])][phase] += dp[i][set][phase];
                if set == BitSet::universal_set(3) && phase != 2 {
                    dp[i + 1][BitSet::empty().inserted(xs[i])][phase + 1] += dp[i][set][phase];
                }
            }
        }
    }

    //table!(&dp);
    let ans = dp[n][BitSet::universal_set(3)][2];
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
use bitset::*;
#[allow(clippy::module_inception)]
pub mod bitset {
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor, Index, IndexMut},
    };

    use itertools::Itertools;

    /// `usize` をビットフラグとして用い、要素数 64 までの集合を管理する構造体です。
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct BitSet {
        bit: usize,
    }

    impl BitSet {
        /// 指定されたビット値を持つ `BitSet` を作成します。
        ///
        /// 下から i ビット目 (2^i の位) が 1 であるとき、要素 i が集合に含まれることに対応します。
        #[inline]
        pub fn new(bit: usize) -> BitSet {
            BitSet { bit }
        }

        /// 内部のビット表現を整数として返します。
        pub fn to_bit(self) -> usize {
            self.bit
        }

        /// 範囲 [0, size) で集合に含まれている要素を `Vec<usize>` で返します。
        pub fn to_vec(self, size: usize) -> Vec<usize> {
            (0..size).filter(|i| (self.bit >> i) & 1 == 1).collect_vec()
        }

        /// 範囲 [0, size) で集合に含まれている要素を列挙するイテレータを返します。
        pub fn to_iter(self, size: usize) -> impl Iterator<Item = usize> {
            (0..size).filter(move |i| (self.bit >> i) & 1 == 1)
        }

        /// 指定された要素 `x` が集合に含まれているかを判定します。
        pub fn contains(self, x: usize) -> bool {
            (self.bit >> x) & 1 == 1
        }

        /// 集合に含まれる要素の数を返します。
        pub fn len(self) -> usize {
            self.bit.count_ones() as usize
        }

        /// 集合に含まれる最小の要素を返します。集合が空の場合は `None` を返します。
        pub fn min_element(self) -> Option<usize> {
            if self.is_empty() {
                None
            } else {
                Some(self.bit.trailing_zeros() as usize)
            }
        }

        /// 集合に含まれる最大の要素を返します。集合が空の場合は `None` を返します。
        pub fn max_element(self) -> Option<usize> {
            if self.is_empty() {
                None
            } else {
                Some(usize::BITS as usize - 1 - self.bit.leading_zeros() as usize)
            }
        }

        /// 集合に含まれない最小の非負整数 (MEX) を返します。
        pub fn mex_element(self) -> usize {
            self.bit.trailing_ones() as usize
        }

        /// 要素 `x` を追加した新しい `BitSet` を返します。
        #[must_use]
        pub fn inserted(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }

        /// 要素 `x` を削除した新しい `BitSet` を返します。
        #[must_use]
        pub fn removed(self, x: usize) -> BitSet {
            BitSet::new(self.bit & !(1 << x))
        }

        /// 空集合を作成します。
        pub fn empty() -> BitSet {
            BitSet::new(0)
        }

        /// 全体集合 [0, size) を作成します。
        pub fn universal_set(size: usize) -> BitSet {
            // size = 64 のときオーバーフローするため注意
            BitSet::new((1 << size) - 1)
        }

        /// 全体集合を [0, size) としたときの補集合を返します。
        #[must_use]
        pub fn complement(self, size: usize) -> BitSet {
            // size = 64 のときオーバーフローするため注意
            BitSet::new(self.bit ^ ((1 << size) - 1))
        }

        /// 差集合 `self \ other` を返します。
        #[must_use]
        pub fn set_minus(self, other: BitSet) -> BitSet {
            BitSet::new(self.bit & !other.bit)
        }

        /// 集合が空であるかを判定します。
        pub fn is_empty(self) -> bool {
            self.bit == 0
        }

        /// `self` が `other` の部分集合であるかを判定します。
        pub fn is_subset(self, other: BitSet) -> bool {
            self | other == other
        }

        /// 2つの集合が共通部分を持たない（互いに素である）かを判定します。
        pub fn is_disjoint(self, other: BitSet) -> bool {
            (self.bit & other.bit) == 0
        }

        /// 全体集合 [0, size) のすべての部分集合を列挙するイテレータを返します。
        pub fn all_subset(size: usize) -> impl Iterator<Item = BitSet> {
            // size = 64 のときオーバーフローするため注意
            (0..(1 << size)).map(BitSet::new)
        }

        /// `self` のすべての部分集合を降順に列挙するイテレータを返します。
        pub fn subsets(self) -> impl Iterator<Item = BitSet> {
            std::iter::successors(Some(self.bit), move |x| {
                if *x == 0 {
                    None
                } else {
                    Some((x - 1) & self.bit)
                }
            })
            .map(BitSet::new)
        }

        /// 全体集合 [0, size) の範囲で、self を部分集合として含むすべての集合を降順に列挙するイテレータを返します。
        pub fn supersets(self, size: usize) -> impl Iterator<Item = BitSet> {
            let complement = Self::universal_set(size).set_minus(self);
            complement.subsets().map(move |s| self | s)
        }
    }

    impl BitAnd for BitSet {
        type Output = BitSet;

        fn bitand(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit & rhs.bit)
        }
    }

    impl BitOr for BitSet {
        type Output = BitSet;

        fn bitor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit | rhs.bit)
        }
    }

    impl BitXor for BitSet {
        type Output = BitSet;

        fn bitxor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit ^ rhs.bit)
        }
    }

    use std::fmt::Debug;
    impl Debug for BitSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("{:#b}", self.bit))?;
            Ok(())
        }
    }

    // [T] に対する実装
    impl<T> Index<BitSet> for [T] {
        type Output = T;

        fn index(&self, s: BitSet) -> &Self::Output {
            &self[s.to_bit()]
        }
    }

    impl<T> IndexMut<BitSet> for [T] {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[s.to_bit()]
        }
    }

    impl<T> Index<BitSet> for Vec<T> {
        type Output = T;

        fn index(&self, s: BitSet) -> &Self::Output {
            &self[..][s] // スライスの Index 実装を利用
        }
    }

    impl<T> IndexMut<BitSet> for Vec<T> {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[..][s] // スライスの IndexMut 実装を利用
        }
    }
}
use lg::*;
#[allow(clippy::module_inception)]
#[cfg_attr(test, macro_use)]
pub mod lg {
    //! 出典: <https://github.com/ngtkana/ac-adapter-rs/blob/196670269568041fad92995acb0156823249e0b4/libs/lg/src/lib.rs>
    //! ライセンス: CC0-1.0。上流の内部モジュールをこのファイル内にまとめ、参照パスを調整した。
    //! 独自変更: `char`・`u8` の `Vec`・配列・スライスをダブルクォーテーション付きの文字列として表示する。
    //! `u8` 単体は `char` に変換し、シングルクォート付きの文字として表示する。
    //!
    //! 変数の値を見出し付きで標準エラー出力に出す、競プロ用デバッグユーティリティ。
    //!
    //! `lg!` は式をそのまま `名前 = 値` の形に整形するだけだが、`table!`/`vmap!`/`hmap!`/`hvec!`/`vvec!` は
    //! 2 次元配列やマップを罫線付きの表に組んで出す。見出しには `stringify!` で得たソースコード上の式の文字列を使い、
    //! 値は `format` を通す。`format` は `u32::MAX` や `i64::MIN` のような番兵値を `*` に、`bool` を `#`/`.` に
    //! 変換するため、「未到達を表す番兵値」や「訪問済みフラグ」の配列を目視でそのまま読める。
    //!
    //! # 仕様
    //!
    //! - `lg!`: 複数の式を `行番号 ❯ 式1 = 値1, 式2 = 値2` の形式で 1 行出力
    //! - `table!`: 2 次元配列を罫線付きの表として出力
    //! - `vmap!` / `hmap!`: マップを縦 / 横の表として出力
    //! - `vvec!` / `hvec!`: 複数の `Vec` を並べて縦 / 横の表として出力（`@field` で構造体のフィールドを抽出可能）
    //! - `format`: `Debug` 文字列を番兵値・`bool`・`Option`・引用符について整形
    //! - `bools`: `bool` の列を `[.#..]` のようなビット列表記に変換
    //!
    //! # 例
    //!
    //! ```
    //! use mylib::lg;
    //! let a = 42;
    //! let v = vec![1, 2, 3];
    //! lg!(a, v);
    //! ```

    mod map {
        use super::align_of;
        use super::format;
        use super::table::Align;
        use super::table::Cell;
        use super::table::Table;
        use std::collections;
        use std::collections::BTreeMap;
        use std::collections::HashMap;
        use std::fmt;
        use std::iter;
        use std::slice;
        use std::vec;

        /// マップを縦向きの表に変換する。1 行目に見出し、以降 1 行ずつ `キー | 値` を並べる。`vmap!` マクロの実体。
        pub fn vmap<'a, K, V, M>(title: &str, map: M) -> Table
        where
            M: Copy + Map<'a, K = K, V = V>,
            K: fmt::Debug,
            V: fmt::Debug,
        {
            Table {
                table: iter::once(vec![
                    Cell {
                        text: String::new(),
                        align: Align::Left,
                    },
                    Cell {
                        text: title.to_string(),
                        align: Align::Center,
                    },
                ])
                .chain(map.map_iter().map(|(k, v)| {
                    let v = format(&v);
                    vec![
                        Cell {
                            text: format(&k),
                            align: Align::Center,
                        },
                        Cell {
                            align: align_of(&v),
                            text: v,
                        },
                    ]
                }))
                .collect(),
            }
        }

        /// マップを横向きの表に変換する。1 行目にキーを、2 行目に値を並べる。`hmap!` マクロの実体。
        pub fn hmap<'a, K, V, M>(title: &str, map: M) -> Table
        where
            M: Copy + Map<'a, K = K, V = V>,
            K: fmt::Debug,
            V: fmt::Debug,
        {
            Table {
                table: vec![
                    iter::once(Cell {
                        text: String::new(),
                        align: Align::Left,
                    })
                    .chain(map.map_iter().map(|(k, _)| Cell {
                        text: format(&k),
                        align: Align::Center,
                    }))
                    .collect(),
                    iter::once(Cell {
                        text: title.to_string(),
                        align: Align::Left,
                    })
                    .chain(map.map_iter().map(|(_, v)| {
                        let v = format(&v);
                        Cell {
                            align: align_of(&v),
                            text: v,
                        }
                    }))
                    .collect(),
                ],
            }
        }

        /// `&(K, V)` を `(&K, &V)` に分解する。スライスや `Vec` に対する `Map::map_iter` の実装で使う。
        pub fn deconstruct_ref_tuple<K, V>((k, v): &(K, V)) -> (&K, &V) {
            (k, v)
        }

        /// `vmap`/`hmap` がマップ的なコンテナを一様に走査するためのトレイト。
        ///
        /// `HashMap`, `BTreeMap`, `&[(K, V)]`, `Vec<(K, V)>`, `[(K, V); N]` への参照に実装する。
        pub trait Map<'a>: 'a {
            /// キーの型。
            type K;
            /// 値の型。
            type V;
            /// `(&K, &V)` を返すイテレータの型。
            type I: Iterator<Item = (&'a Self::K, &'a Self::V)>;
            /// キーと値のペアを走査するイテレータを返す。
            fn map_iter(self) -> Self::I;
        }

        impl<'a, K, V, S> Map<'a> for &'a HashMap<K, V, S> {
            type I = collections::hash_map::Iter<'a, K, V>;
            type K = K;
            type V = V;

            fn map_iter(self) -> Self::I {
                self.iter()
            }
        }

        impl<'a, K, V> Map<'a> for &'a BTreeMap<K, V> {
            type I = collections::btree_map::Iter<'a, K, V>;
            type K = K;
            type V = V;

            fn map_iter(self) -> Self::I {
                self.iter()
            }
        }

        impl<'a, K, V> Map<'a> for &'a [(K, V)] {
            type I = iter::Map<slice::Iter<'a, (K, V)>, fn(&(K, V)) -> (&K, &V)>;
            type K = K;
            type V = V;

            fn map_iter(self) -> Self::I {
                self.iter().map(deconstruct_ref_tuple)
            }
        }

        impl<'a, K, V> Map<'a> for &'a Vec<(K, V)> {
            type I = iter::Map<slice::Iter<'a, (K, V)>, fn(&(K, V)) -> (&K, &V)>;
            type K = K;
            type V = V;

            fn map_iter(self) -> Self::I {
                self.iter().map(deconstruct_ref_tuple)
            }
        }

        impl<'a, const N: usize, K, V> Map<'a> for &'a [(K, V); N] {
            type I = iter::Map<slice::Iter<'a, (K, V)>, fn(&(K, V)) -> (&K, &V)>;
            type K = K;
            type V = V;

            fn map_iter(self) -> Self::I {
                self.iter().map(deconstruct_ref_tuple)
            }
        }
    }
    mod table {
        use core::fmt;

        const GRAY: &str = "\x1b[48;2;127;127;127;37m";
        const RESET: &str = "\x1b[0m";

        /// 罫線付きの表。`Display` でヘッダー行をグレー背景にして整形出力する。
        ///
        /// 各行は同じ列数を持つ必要がある（`Display` の実装内で `assert!` する）。
        pub struct Table {
            /// 行の集まり。各行は `Cell` の列。
            pub table: Vec<Vec<Cell>>,
        }

        /// `Table` の 1 セル。表示テキストと寄せ方向を持つ。
        pub struct Cell {
            /// 表示テキスト。
            pub text: String,
            /// 寄せ方向。
            pub align: Align,
        }

        /// セルの寄せ方向。
        pub enum Align {
            /// 左寄せ。
            Left,
            /// 中央寄せ。
            Center,
            /// 右寄せ（数値のセルに使う）。
            Right,
        }

        impl fmt::Display for Table {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                struct ColumnFormat<'a> {
                    pre: &'a str,
                    width: usize,
                    post: &'a str,
                }
                let Self { table } = self;
                let w = table[0].len();
                assert!(table.iter().all(|row| row.len() == w));
                let column_format = (0..w)
                    .map(|j| ColumnFormat {
                        pre: " ",
                        width: table
                            .iter()
                            .map(|row| row[j].text.len().max(1))
                            .max()
                            .unwrap(),
                        post: if j == 0 { " │" } else { " " },
                    })
                    .collect::<Vec<_>>();
                for (i, row) in table.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{GRAY}")?;
                    }
                    for (&ColumnFormat { pre, width, post }, Cell { text, align }) in
                        column_format.iter().zip(row)
                    {
                        write!(f, "{pre}")?;
                        match align {
                            Align::Left => write!(f, "{text:<width$}")?,
                            Align::Center => write!(f, "{text:^width$}")?,
                            Align::Right => write!(f, "{text:>width$}")?,
                        }
                        write!(f, "{post}")?;
                    }
                    if i == 0 {
                        write!(f, "{RESET}")?;
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }
    }
    mod vec2 {
        use super::align_of;
        use super::format;
        use super::table::Align;
        use super::table::Cell;
        use super::table::Table;
        use std::fmt;
        use std::iter;

        /// 2 次元配列を `Table` に変換する。1 行目に列番号、1 列目に行番号を振り、各セルは値を `format` で整形した文字列にする。`table!` マクロの実体。
        ///
        /// 行ごとの長さが揃っていなくてもよい。足りないセルは空文字で埋める。
        pub fn vec2<'a, T, R, S>(title: &str, vec2: &'a S) -> Table
        where
            T: fmt::Debug + 'a,
            R: ?Sized,
            &'a R: Copy + IntoIterator<Item = &'a T> + 'a,
            &'a S: Copy + IntoIterator<Item = &'a R>,
        {
            let w = vec2
                .into_iter()
                .map(|row| row.into_iter().count())
                .max()
                .unwrap();
            Table {
                table: iter::once(
                    iter::once(Cell {
                        text: title.to_string(),
                        align: Align::Left,
                    })
                    .chain((0..w).map(|i| Cell {
                        text: i.to_string(),
                        align: Align::Center,
                    }))
                    .collect(),
                )
                .chain(vec2.into_iter().enumerate().map(|(j, row)| {
                    iter::once(Cell {
                        text: j.to_string(),
                        align: Align::Center,
                    })
                    .chain(row.into_iter().map(|v| {
                        let v = format(&v);
                        Cell {
                            align: align_of(&v),
                            text: v,
                        }
                    }))
                    .chain(iter::repeat_with(|| Cell {
                        text: String::new(),
                        align: Align::Left,
                    }))
                    .take(1 + w)
                    .collect()
                }))
                .collect(),
            }
        }
    }
    mod vecs {
        use super::align_of;
        use super::table::Align;
        use super::table::Cell;
        use super::table::Table;
        use std::iter;

        /// 名前付きの文字列列 `(名前, 値の列)` を、各列を 1 行とする横向きの `Table` に変換する。`hvec!` マクロの実体。
        ///
        /// 列ごとに長さが異なってもよい。足りないセルは空文字で埋める。
        pub fn hvec(vecs: &[(String, Vec<String>)]) -> Table {
            let w = vecs.iter().map(|(_, row)| row.len()).max().unwrap();
            Table {
                table: iter::once(
                    iter::once(Cell {
                        text: String::new(),
                        align: Align::Left,
                    })
                    .chain((0..w).map(|i| Cell {
                        text: i.to_string(),
                        align: Align::Center,
                    }))
                    .collect(),
                )
                .chain(vecs.iter().map(|(title, row)| {
                    iter::once(Cell {
                        text: title.to_string(),
                        align: Align::Center,
                    })
                    .chain(row.iter().map(|v| Cell {
                        align: align_of(v),
                        text: v.clone(),
                    }))
                    .chain(iter::repeat_with(|| Cell {
                        text: String::new(),
                        align: Align::Left,
                    }))
                    .take(1 + w)
                    .collect()
                }))
                .collect(),
            }
        }

        /// 名前付きの文字列列 `(名前, 値の列)` を、各列を 1 列とする縦向きの `Table` に変換する。`vvec!` マクロの実体。
        ///
        /// 列ごとに長さが異なってもよい。足りないセルは空文字で埋める。
        pub fn vvec(vecs: &[(String, Vec<String>)]) -> Table {
            let h = vecs.iter().map(|(_, col)| col.len()).max().unwrap();
            Table {
                table: iter::once(
                    iter::once(Cell {
                        text: String::new(),
                        align: Align::Center,
                    })
                    .chain(vecs.iter().map(|(title, _)| Cell {
                        text: title.to_string(),
                        align: Align::Center,
                    }))
                    .collect(),
                )
                .chain((0..h).map(|i| {
                    iter::once(Cell {
                        text: i.to_string(),
                        align: Align::Center,
                    })
                    .chain(vecs.iter().map(|(_, vec)| {
                        let v = vec.get(i).map_or("", String::as_str);
                        Cell {
                            align: align_of(v),
                            text: v.to_string(),
                        }
                    }))
                    .collect()
                }))
                .collect(),
            }
        }
    }

    pub use map::hmap;
    pub use map::vmap;
    use std::borrow::Borrow;
    use std::fmt;
    use table::Align;
    pub use vec2::vec2;
    pub use vecs::hvec;
    pub use vecs::vvec;

    /// `bool` の列を `[.#..]` の形式に変換する。真を `#`、偽を `.` で表す。
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::lg::bools;
    /// assert_eq!(bools([true, false, true]), "[#.#]");
    /// ```
    pub fn bools<B, I>(iter: I) -> String
    where
        B: Borrow<bool>,
        I: IntoIterator<Item = B>,
    {
        format!(
            "[{}]",
            iter.into_iter()
                .map(|b| ['.', '#'][usize::from(*(b.borrow()))])
                .collect::<String>(),
        )
    }

    /// 表のセルの寄せ方向を、文字列が数値として解釈できるかで決める。
    ///
    /// 浮動小数点として parse できれば右寄せ、できなければ左寄せにする。数値の列を右揃えで見やすくするための判定。
    pub fn align_of(s: &str) -> Align {
        // To improve this: https://doc.rust-lang.org/reference/tokens.html#floating-point-literals
        match s.parse::<f64>() {
            Ok(_) => Align::Right,
            Err(_) => Align::Left,
        }
    }

    /// 複数の式を評価し、`行番号 ❯ 式1 = 値1, 式2 = 値2` の形式で標準エラー出力に 1 行出す。
    ///
    /// 各値は `format` で整形する。競プロのデバッグで、複数の変数を毎回 `eprintln!` を書かずに確認する用途に使う。
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::lg;
    /// let a = 1;
    /// let b = vec![2, 3];
    /// lg!(a, b); // 標準エラーに `<行番号> ❯ a = 1, b = [2, 3]` を出力
    /// ```
    #[macro_export]
    macro_rules! lg {
    (@contents $head:expr $(, $tail:expr)*) => {{
        $crate::lg::__lg_internal!($head);
        $(
            eprint!(",");
            $crate::lg::__lg_internal!($tail);
        )*
        eprintln!();
    }};
    ($($expr:expr),* $(,)?) => {{
        eprint!("{} \u{276f}", line!());
        $crate::lg::lg!(@contents $($expr),*)
    }};
}

    /// `lg!` の内部実装。1 個の式を `式 = 値` の形式で出力する。外部から直接使わない。
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __lg_internal {
        ($value:expr) => {{
            match $value {
                head => {
                    eprint!(" {} = {}", stringify!($value), $crate::lg::format(&head));
                }
            }
        }};
    }

    /// 2 次元配列（`Vec<Vec<T>>` など、行の集まりを 2 重にイテレートできる型）を罫線付きの表として標準エラー出力に出す。
    ///
    /// 見出しには式のソース文字列（先頭の `&` は除く）を使い、各セルの値は `format` で整形する。
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::table;
    /// let v = vec![vec![1, 2], vec![3, 4]];
    /// table!(&v);
    /// ```
    #[macro_export]
    macro_rules! table {
        ($vec2:expr) => {
            eprint!(
                "{}",
                $crate::lg::vec2($crate::lg::remove_ampersand(stringify!($vec2)), $vec2)
            );
        };
    }

    /// マップ（`HashMap`, `BTreeMap`, `&[(K, V)]` など）を表にして標準エラー出力に出す。要素ごとに 1 行、`キー | 値` の形式で縦に並べる。
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::vmap;
    /// let m = [(0, "a"), (1, "b")];
    /// vmap!(&m);
    /// ```
    #[macro_export]
    macro_rules! vmap {
        ($map:expr) => {
            eprint!(
                "{}",
                $crate::lg::vmap($crate::lg::remove_ampersand(stringify!($map)), $map)
            );
        };
    }

    /// マップ（`HashMap`, `BTreeMap`, `&[(K, V)]` など）を表にして標準エラー出力に出す。1 行目にキーを、2 行目に値を横に並べる。
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::hmap;
    /// let m = [(0, "a"), (1, "b")];
    /// hmap!(&m);
    /// ```
    #[macro_export]
    macro_rules! hmap {
        ($map:expr) => {
            eprint!(
                "{}",
                $crate::lg::hmap($crate::lg::remove_ampersand(stringify!($map)), $map)
            );
        };
    }

    /// 複数の `Vec`（またはイテレート可能な列）を、それぞれ 1 列とする表にして標準エラー出力に出す。列見出しには式のソース文字列を、行見出しにはインデックスを使う。
    ///
    /// `@field name expr` を前置すると、要素が構造体のときに指定フィールドだけを抽出して列にできる。
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::vvec;
    /// let a = [1, 2, 3];
    /// let b = [4, 5, 6];
    /// vvec!(&a, &b);
    /// ```
    #[macro_export]
    macro_rules! vvec {
    ($($(@field $field:ident)* $vecs:expr),+ $(,)?) => {
        let mut vecs = Vec::new();
        $(
            let name = $crate::lg::remove_ampersand(stringify!($vecs));
            #[allow(unused_mut, unused_assignments)]
            let mut has_field = false;
            $(
                #[allow(unused_mut, unused_assignments)]
                {
                    let mut name = name.to_owned();
                    has_field = true;
                    name.push_str(".");
                    name.push_str(stringify!($field));
                    let values = (&$vecs).into_iter().map(|v| $crate::lg::format(&v.$field)).collect::<Vec<_>>();
                    vecs.push((name, values))
                }
            )*
            if !has_field {
                let values = (&$vecs).into_iter().map(|v| $crate::lg::format(&v)).collect::<Vec<_>>();
                vecs.push((name.to_owned(), values))
            }
        )+
        eprint!("{}", $crate::lg::vvec(&vecs));
    };
}

    /// 複数の `Vec`（またはイテレート可能な列）を、それぞれ 1 行とする表にして標準エラー出力に出す。行見出しには式のソース文字列を、列見出しにはインデックスを使う。
    ///
    /// `@field name expr` を前置すると、要素が構造体のときに指定フィールドだけを抽出して行にできる。
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::hvec;
    /// let a = [1, 2, 3];
    /// let b = [4, 5, 6];
    /// hvec!(&a, &b);
    /// ```
    #[macro_export]
    macro_rules! hvec {
    ($($(@field $field:ident)* $vecs:expr),+ $(,)?) => {
        let mut vecs = Vec::new();
        $(
            let name = $crate::lg::remove_ampersand(stringify!($vecs));
            #[allow(unused_mut, unused_assignments)]
            let mut has_field = false;
            $(
                #[allow(unused_mut, unused_assignments)]
                {
                    let mut name = name.to_owned();
                    has_field = true;
                    name.push_str(".");
                    name.push_str(stringify!($field));
                    let values = (&$vecs).into_iter().map(|v| $crate::lg::format(&v.$field)).collect::<Vec<_>>();
                    vecs.push((name, values))
                }
            )*
            if !has_field {
                let values = (&$vecs).into_iter().map(|v| $crate::lg::format(&v)).collect::<Vec<_>>();
                vecs.push((name.to_owned(), values))
            }
        )+
        eprint!("{}", $crate::lg::hvec(&vecs));
    };
}

    /// 文字列先頭の `&` をすべて取り除く。
    ///
    /// マクロが `stringify!(&&x)` のような参照付きの式を見出しに使う際、表示を `x` に揃えるために使う。
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::lg::remove_ampersand;
    /// assert_eq!(remove_ampersand("&&x"), "x");
    /// ```
    pub fn remove_ampersand(mut s: &str) -> &str {
        while let Some(t) = s.strip_prefix('&') {
            s = t;
        }
        s
    }

    /// 値を `Debug` で文字列化し、読みやすく整形する。
    ///
    /// 次の変換を順に適用する。
    ///
    /// - `u8` 単体は `as char` で変換し、`char` と同じシングルクォート付きの文字として表示（参照も同様）
    /// - `char` / `u8` の `Vec`・配列・スライスはダブルクォーテーション付きの文字列として表示（参照も同様）。制御文字はエスケープし、不正な UTF-8 は `�` に置換
    /// - `u32::MAX`, `u64::MAX`, `u128::MAX`, `i32::MIN`/`MAX`, `i64::MIN`/`MAX`, `i128::MAX` を番兵値とみなし `*` に置換
    /// - `None` を `*` に、`Some(x)` を `x` に置換（入れ子も再帰的に剥がす）
    /// - `true`/`false` を `#`/`.` に置換
    /// - 文字列を囲む `"` を除去
    ///
    /// # 例
    ///
    /// ```
    /// use mylib::lg::format;
    /// assert_eq!(format(&Some(3)), "3");
    /// assert_eq!(format(&u32::MAX), "*");
    /// assert_eq!(format(&true), "#");
    /// ```
    pub fn format<T: fmt::Debug + ?Sized>(t: &T) -> String {
        let debug = format!("{t:?}");
        let mut name = std::any::type_name::<T>();
        while let Some(inner) = name.strip_prefix('&') {
            name = inner.strip_prefix("mut ").unwrap_or(inner);
        }
        if name == "u8" {
            return format!("{:?}", debug.parse::<u8>().unwrap() as char);
        }
        // Debug 表現だけでは整数列とバイト列を区別できないため、型も確認する。
        if name == std::any::type_name::<Vec<char>>()
            || name == "[char]"
            || name.starts_with("[char; ")
        {
            return if debug == "[]" {
                "\"\"".to_owned()
            } else {
                let text = debug
                    .strip_prefix("['")
                    .unwrap()
                    .strip_suffix("']")
                    .unwrap()
                    .split("', '")
                    .map(|s| s.replace(r"\'", "'").replace('"', r#"\""#))
                    .collect::<String>();
                format!("\"{text}\"")
            };
        }
        if name == std::any::type_name::<Vec<u8>>() || name == "[u8]" || name.starts_with("[u8; ") {
            let bytes = debug[1..debug.len() - 1]
                .split(", ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            return format!("{:?}", String::from_utf8_lossy(&bytes));
        }
        let s = debug
            .replace("340282366920938463463374607431768211455", "*") // u128
            .replace("170141183460469231731687303715884105727", "*") // i128
            .replace("18446744073709551615", "*") // u64
            .replace("9223372036854775807", "*") // i64
            .replace("-9223372036854775808", "*") // i64
            .replace("4294967295", "*") // u32
            .replace("2147483647", "*") // i32
            .replace("-2147483648", "*") // i32
            .replace("None", "*")
            .replace("true", "#")
            .replace("false", ".");
        let mut s = s.as_str();
        while s.starts_with("Some(") {
            s = s.strip_prefix("Some(").unwrap();
            s = s.strip_suffix(')').unwrap();
        }
        while s.len() > 2 && s.starts_with('"') && s.ends_with('"') {
            s = s.strip_prefix('"').unwrap();
            s = s.strip_suffix('"').unwrap();
        }
        s.to_owned()
    }

    // 属性マクロで展開されたマクロ同士も、再公開パス経由で呼び出せるようにする。
    #[doc(hidden)]
    pub use {__lg_internal, lg};
}
