use crate::data_structure::segtree_lib::segtree::segtree_utils::segtree_debug_string;
use cargo_snippet::snippet;

/// 遅延セグメント木の更新反映後の値を、根から葉の順に標準エラーへ出力する。
///
/// `len` にはセグメント木の要素数を指定する。
/// 値の取得に伴い、遅延更新を子へ伝播する。
#[snippet(include = "lazy_segtree_debug_string")]
pub fn lazy_segtree_debug<F: ac_library::MapMonoid>(
    seg: &mut ac_library::LazySegtree<F>,
    len: usize,
) where
    <F::M as ac_library::Monoid>::S: std::fmt::Debug,
{
    eprint!("{}", lazy_segtree_debug_string(seg, len));
}

/// 遅延セグメント木の更新反映後の値を、区切り付きの文字列にする。
///
/// `len` にはセグメント木の要素数を指定する。
/// 値の取得に伴い、遅延更新を子へ伝播する。
///
/// 葉の数は2の冪に切り上げ、実データを含まない区間は `*` と表示する。
///
/// 値は `Debug` の整形表示を使い、構造体はフィールドごとに改行する。
/// インデントを保ち、表示ブロック全体を各区間の中央に配置する。
#[snippet(include = "segtree_debug_string")]
pub fn lazy_segtree_debug_string<F: ac_library::MapMonoid>(
    seg: &mut ac_library::LazySegtree<F>,
    len: usize,
) -> String
where
    <F::M as ac_library::Monoid>::S: std::fmt::Debug,
{
    let values: Vec<_> = (0..len).map(|i| seg.get(i)).collect();
    let snapshot = ac_library::Segtree::<F::M>::from(values);
    segtree_debug_string(&snapshot, len)
}

#[cfg(test)]
mod tests {
    use super::{lazy_segtree_debug, lazy_segtree_debug_string};
    use crate::data_structure::segtree_lib::lazy_segtree::{
        range_add_range_max::range_add_range_max::{AddAction, RangeAddRangeMax},
        range_affine_range_sum::range_affine_range_sum::{Affine, RangeAffineRangeSum, RangeSum},
    };
    use ac_library::LazySegtree;

    #[test]
    fn test_pending_range_updates() {
        let mut seg = LazySegtree::<RangeAddRangeMax<i64>>::from(vec![1, 2, 3, 4]);
        seg.apply_range(.., AddAction::new(10));
        seg.apply_range(0..2, AddAction::new(100));
        assert_eq!(
            lazy_segtree_debug_string(&mut seg, 4),
            concat!(
                "|          112          |\n",
                "|    112    |    14     |\n",
                "| 111 | 112 | 13  | 14  |\n",
            )
        );
        assert_eq!(seg.prod(..), 112);
        seg.apply_range(.., AddAction::new(1));
        assert_eq!(seg.get(0), 112);
        assert_eq!(seg.prod(2..4), 15);
        lazy_segtree_debug(&mut seg, 4);
    }

    #[test]
    fn test_empty_and_single() {
        let mut empty = LazySegtree::<RangeAddRangeMax<i64>>::new(0);
        assert_eq!(lazy_segtree_debug_string(&mut empty, 0), "");
        let mut single = LazySegtree::<RangeAddRangeMax<i64>>::from(vec![0]);
        single.apply_range(.., AddAction::new(-12));
        assert_eq!(lazy_segtree_debug_string(&mut single, 1), "| -12 |\n");
    }
    #[test]
    fn test_struct_composition_and_padding() {
        let mut seg =
            LazySegtree::<RangeAffineRangeSum<i64>>::from([1, 2, 3].map(RangeSum::unit).to_vec());
        seg.apply_range(
            ..,
            Affine {
                slope: 2,
                intercept: 1,
            },
        );
        seg.apply_range(
            ..,
            Affine {
                slope: 3,
                intercept: 4,
            },
        );
        assert_eq!(
            lazy_segtree_debug_string(&mut seg, 3),
            concat!(
                "|                       RangeSum {                          |\n",
                "|                           sum: 57,                        |\n",
                "|                           len: 3,                         |\n",
                "|                       }                                   |\n",
                "|        RangeSum {           |        RangeSum {           |\n",
                "|            sum: 32,         |            sum: 25,         |\n",
                "|            len: 2,          |            len: 1,          |\n",
                "|        }                    |        }                    |\n",
                "| RangeSum {   | RangeSum {   | RangeSum {   |      *       |\n",
                "|     sum: 13, |     sum: 19, |     sum: 25, |              |\n",
                "|     len: 1,  |     len: 1,  |     len: 1,  |              |\n",
                "| }            | }            | }            |              |\n",
            )
        );
        assert_eq!(seg.all_prod(), RangeSum { sum: 57, len: 3 });
    }
}
