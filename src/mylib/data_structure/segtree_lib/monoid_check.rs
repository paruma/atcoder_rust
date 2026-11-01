use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use monoid_check::*;")]
pub mod monoid_check {
    //! 与えられた値と更新の全組合せで、ACL の `Monoid`・`MapMonoid` の公理を検査する。
    //!
    //! 引数 `values` に検査用の値を、`updates` に検査用の更新を渡す。
    //! これらをサンプルと呼ぶ。値には単位元を、更新には恒等更新を自動的に追加する。
    //! 各サンプルには実際に取り得るものを指定する。
    //! 例えば区間和の値が合計 `sum` と要素数 `len` を持つ場合、両者が整合する値を用意する。
    //!
    //! # 記号
    //!
    //! - `x`, `y`, `z`: 値のサンプル。
    //! - `f`, `g`, `h`: 更新のサンプル。
    //! - `e`: 値の単位元。
    //! - `id`: 恒等更新。
    //! - `op(x, y)`: 左の区間の値 `x` と右の区間の値 `y` の結合。
    //! - `mapping(f, x)`: 値 `x` に更新 `f` を適用した結果。
    //! - `composition(f, g)`: 先に `g`、次に `f` を適用する更新の合成。
    //!
    //! # モノイドの検査
    //!
    //! [`check_monoid`] は次を検査する。
    //!
    //! | 公理 | 検査する式 |
    //! | --- | --- |
    //! | 左単位元 | `op(e, x) == x` |
    //! | 右単位元 | `op(x, e) == x` |
    //! | 結合則 | `op(op(x, y), z) == op(x, op(y, z))` |
    //!
    //! # MapMonoid の検査
    //!
    //! [`check_map_monoid`] は上の検査に加え、次を検査する。
    //! 値の単位元と結合には、`MapMonoid` 側で上書きされた定義も使用する。
    //!
    //! | 公理 | 検査する式 |
    //! | --- | --- |
    //! | 恒等更新 | `mapping(id, x) == x` |
    //! | 更新合成の左単位元 | `mapping(composition(id, f), x) == mapping(f, x)` |
    //! | 更新合成の右単位元 | `mapping(composition(f, id), x) == mapping(f, x)` |
    //! | 合成と適用の整合性 | `mapping(composition(f, g), x) == mapping(f, mapping(g, x))` |
    //! | 更新合成の結合則 | `mapping(composition(composition(f, g), h), x) == mapping(composition(f, composition(g, h)), x)` |
    //! | 更新と結合の整合性 | `mapping(f, op(x, y)) == op(mapping(f, x), mapping(f, y))` |
    //!
    //! 更新合成の単位元と結合則は、値へ適用した結果を比較して検査する。
    //!
    //! # 結果の読み方
    //!
    //! 成功時は `Ok(())`、最初の違反が見つかった場合は `Err(LawViolation)` を返す。
    //! 違反には式、入力、左右の値が含まれ、`Display` で表示できる。

    use ac_library::{MapMonoid, Monoid};
    use std::fmt::{self, Debug};

    /// 公理に違反した入力と、その式の左右の値。
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct LawViolation {
        pub law: &'static str,
        pub inputs: String,
        pub lhs: String,
        pub rhs: String,
    }

    impl fmt::Display for LawViolation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "違反: {}\n{}\n左辺: {}\n右辺: {}",
                self.law, self.inputs, self.lhs, self.rhs
            )
        }
    }

    impl std::error::Error for LawViolation {}

    fn check_equal<S: PartialEq + Debug>(
        law: &'static str,
        lhs: &S,
        rhs: &S,
        inputs: impl FnOnce() -> String,
    ) -> Result<(), LawViolation> {
        if lhs == rhs {
            Ok(())
        } else {
            Err(LawViolation {
                law,
                inputs: inputs(),
                lhs: format!("{lhs:?}"),
                rhs: format!("{rhs:?}"),
            })
        }
    }

    fn check_monoid_laws<S: PartialEq + Debug>(
        values: &[S],
        identity: &S,
        op: impl Fn(&S, &S) -> S,
    ) -> Result<(), LawViolation> {
        // 左単位元
        for x in values {
            check_equal("op(e, x) == x", &op(identity, x), x, || {
                format!("e = {identity:?}, x = {x:?}")
            })?;
        }

        // 右単位元
        for x in values {
            check_equal("op(x, e) == x", &op(x, identity), x, || {
                format!("e = {identity:?}, x = {x:?}")
            })?;
        }

        // 結合則
        for x in values {
            for y in values {
                for z in values {
                    check_equal(
                        "op(op(x, y), z) == op(x, op(y, z))",
                        &op(&op(x, y), z),
                        &op(x, &op(y, z)),
                        || format!("x = {x:?}, y = {y:?}, z = {z:?}"),
                    )?;
                }
            }
        }
        Ok(())
    }

    /// `values` に与えられた値の全組合せでモノイドの単位元と結合則を検査する。
    ///
    /// 単位元を自動的に追加し、最初の違反を返す。
    /// `values` には、そのモノイドで実際に取り得る値を指定する。
    ///
    /// `values` の要素数を N として、演算・比較の回数は `O(N^3)`。
    pub fn check_monoid<M: Monoid>(values: &[M::S]) -> Result<(), LawViolation>
    where
        M::S: PartialEq + Debug,
    {
        let identity = M::identity();
        let values: Vec<_> = values
            .iter()
            .cloned()
            .chain(std::iter::once(identity.clone()))
            .collect();
        check_monoid_laws(&values, &identity, M::binary_operation)
    }

    /// `values` と `updates` に与えられた値と更新の全組合せで公理を検査する。
    ///
    /// 単位元と恒等更新を自動的に追加し、最初の違反を返す。
    /// 値の単位元・結合則、恒等更新、更新合成の単位元・結合則、
    /// 合成と適用の整合性、更新と値の結合の整合性を検査する。
    /// 更新合成の単位元と結合則は、値へ適用した結果を比較して検査する。
    ///
    /// `values` と `updates` には、実際に取り得る値と更新を指定する。
    ///
    /// `values` の要素数を N、`updates` の要素数を K として、
    /// 演算・比較の回数は `O(N^3 + NK^3 + N^2K)`。
    pub fn check_map_monoid<F: MapMonoid>(
        values: &[<F::M as Monoid>::S],
        updates: &[F::F],
    ) -> Result<(), LawViolation>
    where
        <F::M as Monoid>::S: PartialEq + Debug,
        F::F: Debug,
    {
        let identity = F::identity_element();
        let identity_map = F::identity_map();
        let values: Vec<_> = values
            .iter()
            .cloned()
            .chain(std::iter::once(identity.clone()))
            .collect();
        let updates: Vec<_> = updates
            .iter()
            .cloned()
            .chain(std::iter::once(identity_map.clone()))
            .collect();
        // MapMonoid 側で上書きされた演算も、ACL が実際に使う定義で検査する。
        check_monoid_laws(&values, &identity, F::binary_operation)?;

        // 恒等更新
        for x in &values {
            check_equal(
                "mapping(id, x) == x",
                &F::mapping(&identity_map, x),
                x,
                || format!("id = {identity_map:?}, x = {x:?}"),
            )?;
        }

        // 更新合成の左単位元
        for x in &values {
            for f in &updates {
                check_equal(
                    "mapping(composition(id, f), x) == mapping(f, x)",
                    &F::mapping(&F::composition(&identity_map, f), x),
                    &F::mapping(f, x),
                    || format!("id = {identity_map:?}, f = {f:?}, x = {x:?}"),
                )?;
            }
        }

        // 更新合成の右単位元
        for x in &values {
            for f in &updates {
                check_equal(
                    "mapping(composition(f, id), x) == mapping(f, x)",
                    &F::mapping(&F::composition(f, &identity_map), x),
                    &F::mapping(f, x),
                    || format!("id = {identity_map:?}, f = {f:?}, x = {x:?}"),
                )?;
            }
        }

        // 合成と適用の整合性
        for x in &values {
            for f in &updates {
                for g in &updates {
                    check_equal(
                        "mapping(composition(f, g), x) == mapping(f, mapping(g, x))",
                        &F::mapping(&F::composition(f, g), x),
                        &F::mapping(f, &F::mapping(g, x)),
                        || format!("f = {f:?}, g = {g:?}, x = {x:?}"),
                    )?;
                }
            }
        }

        // 更新合成の結合則
        for x in &values {
            for f in &updates {
                for g in &updates {
                    for h in &updates {
                        check_equal(
                            "mapping(composition(composition(f, g), h), x) == mapping(composition(f, composition(g, h)), x)",
                            &F::mapping(&F::composition(&F::composition(f, g), h), x),
                            &F::mapping(&F::composition(f, &F::composition(g, h)), x),
                            || format!("f = {f:?}, g = {g:?}, h = {h:?}, x = {x:?}"),
                        )?;
                    }
                }
            }
        }

        // 更新と結合の整合性
        for x in &values {
            for f in &updates {
                for y in &values {
                    check_equal(
                        "mapping(f, op(x, y)) == op(mapping(f, x), mapping(f, y))",
                        &F::mapping(f, &F::binary_operation(x, y)),
                        &F::binary_operation(&F::mapping(f, x), &F::mapping(f, y)),
                        || format!("f = {f:?}, x = {x:?}, y = {y:?}"),
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::monoid_check::{check_map_monoid, check_monoid};
    use crate::data_structure::segtree_lib::lazy_segtree::{
        range_add_range_sum::range_add_range_sum::{RangeAddRangeSum, RangeSum, ValueLenSum},
        range_affine_range_sum::range_affine_range_sum::{
            Affine, RangeAffineRangeSum, RangeSum as AffineSum,
        },
    };
    use ac_library::{Additive, MapMonoid, Max, Monoid};

    #[test]
    fn test_valid_monoids_and_empty_samples() {
        assert_eq!(check_monoid::<Additive<i64>>(&[-3, 0, 2, 2]), Ok(()));
        assert_eq!(check_monoid::<Max<i64>>(&[-3, 0, 2]), Ok(()));
        assert_eq!(check_monoid::<Additive<i64>>(&[]), Ok(()));
    }

    #[test]
    fn test_noncommutative_monoid() {
        enum Concat {}
        impl Monoid for Concat {
            type S = String;
            fn identity() -> String {
                String::new()
            }
            fn binary_operation(a: &String, b: &String) -> String {
                format!("{a}{b}")
            }
        }
        assert_eq!(check_monoid::<Concat>(&["ab".into(), "c".into()]), Ok(()));
    }

    #[test]
    fn test_incorrect_left_identity() {
        enum BadMax {}
        impl Monoid for BadMax {
            type S = i64;
            fn identity() -> i64 {
                0
            }
            fn binary_operation(a: &i64, b: &i64) -> i64 {
                (*a).max(*b)
            }
        }
        let error = check_monoid::<BadMax>(&[-3]).unwrap_err();
        assert_eq!(error.law, "op(e, x) == x");
        assert_eq!(error.inputs, "e = 0, x = -3");
        assert_eq!(error.lhs, "0");
        assert_eq!(error.rhs, "-3");
        assert_eq!(
            error.to_string(),
            "\
違反: op(e, x) == x
e = 0, x = -3
左辺: 0
右辺: -3"
        );
    }

    #[test]
    fn test_incorrect_right_identity() {
        enum BadRight {}
        impl Monoid for BadRight {
            type S = i64;
            fn identity() -> i64 {
                0
            }
            fn binary_operation(a: &i64, b: &i64) -> i64 {
                if *a != 0 && *b == 0 { a + 1 } else { a + b }
            }
        }
        assert_eq!(
            check_monoid::<BadRight>(&[1]).unwrap_err().law,
            "op(x, e) == x"
        );
    }

    #[test]
    fn test_incorrect_associativity() {
        enum BadAverage {}
        impl Monoid for BadAverage {
            type S = i64;
            fn identity() -> i64 {
                0
            }
            fn binary_operation(a: &i64, b: &i64) -> i64 {
                match (*a, *b) {
                    (0, _) => *b,
                    (_, 0) => *a,
                    _ => (a + b) / 2,
                }
            }
        }
        assert_eq!(
            check_monoid::<BadAverage>(&[1, 3, 5]).unwrap_err().law,
            "op(op(x, y), z) == op(x, op(y, z))"
        );
    }

    #[test]
    fn test_range_add_range_sum() {
        let values = [
            RangeSum { sum: 0, len: 0 },
            RangeSum::unit(-2),
            RangeSum { sum: 7, len: 3 },
        ];
        assert_eq!(
            check_map_monoid::<RangeAddRangeSum<i64>>(&values, &[-3, 0, 2]),
            Ok(())
        );
        assert_eq!(check_map_monoid::<RangeAddRangeSum<i64>>(&[], &[]), Ok(()));
    }

    #[test]
    fn test_missing_length_factor() {
        enum MissingLength {}
        impl MapMonoid for MissingLength {
            type M = ValueLenSum<i64>;
            type F = i64;
            fn identity_map() -> i64 {
                0
            }
            fn composition(f: &i64, g: &i64) -> i64 {
                f + g
            }
            fn mapping(f: &i64, x: &RangeSum<i64>) -> RangeSum<i64> {
                // 正しくは sum: x.sum + f * x.len。
                RangeSum {
                    sum: x.sum + f,
                    len: x.len,
                }
            }
        }
        let error =
            check_map_monoid::<MissingLength>(&[RangeSum { sum: 4, len: 2 }], &[3]).unwrap_err();
        assert_eq!(
            error.law,
            "mapping(f, op(x, y)) == op(mapping(f, x), mapping(f, y))"
        );
        assert_eq!(
            error.inputs,
            "f = 3, x = RangeSum { sum: 4, len: 2 }, y = RangeSum { sum: 4, len: 2 }"
        );
        assert_eq!(error.lhs, "RangeSum { sum: 11, len: 4 }");
        assert_eq!(error.rhs, "RangeSum { sum: 14, len: 4 }");
    }

    #[test]
    fn test_affine_updates_and_composition_order() {
        let values = [AffineSum::unit(-2), AffineSum { sum: 7, len: 3 }];
        let updates = [
            Affine {
                slope: 2,
                intercept: 1,
            },
            Affine {
                slope: 3,
                intercept: 4,
            },
            Affine::constant_func(0),
        ];
        assert_eq!(
            check_map_monoid::<RangeAffineRangeSum<i64>>(&values, &updates),
            Ok(())
        );

        enum ReversedComposition {}
        impl MapMonoid for ReversedComposition {
            type M = <RangeAffineRangeSum<i64> as MapMonoid>::M;
            type F = Affine<i64>;
            fn identity_map() -> Self::F {
                RangeAffineRangeSum::identity_map()
            }
            fn composition(f: &Self::F, g: &Self::F) -> Self::F {
                RangeAffineRangeSum::composition(g, f)
            }
            fn mapping(f: &Self::F, x: &AffineSum<i64>) -> AffineSum<i64> {
                RangeAffineRangeSum::mapping(f, x)
            }
        }
        assert_eq!(
            check_map_monoid::<ReversedComposition>(&values, &updates)
                .unwrap_err()
                .law,
            "mapping(composition(f, g), x) == mapping(f, mapping(g, x))"
        );
    }

    #[test]
    fn test_updates_need_not_implement_partial_eq() {
        #[derive(Clone, Debug)]
        struct Add(i64);
        enum AddToMax {}
        impl MapMonoid for AddToMax {
            type M = Max<i64>;
            type F = Add;
            fn identity_map() -> Add {
                Add(0)
            }
            fn composition(f: &Add, g: &Add) -> Add {
                Add(f.0 + g.0)
            }
            fn mapping(f: &Add, x: &i64) -> i64 {
                if *x == i64::MIN { *x } else { x + f.0 }
            }
        }
        assert_eq!(
            check_map_monoid::<AddToMax>(&[-2, 3], &[Add(-1), Add(2)]),
            Ok(())
        );
    }

    #[test]
    fn test_incorrect_update_identities_and_associativity() {
        enum BrokenAdd<const BUG: u8> {}
        impl<const BUG: u8> MapMonoid for BrokenAdd<BUG> {
            type M = Max<i64>;
            type F = i64;
            fn identity_map() -> i64 {
                i64::from(BUG == 0)
            }
            fn composition(f: &i64, g: &i64) -> i64 {
                let incorrect = match BUG {
                    1 => *f == 0,
                    2 => *g == 0 && *f != 0,
                    // 元のサンプル同士の合成は正しいが、合成結果を再び合成すると壊れる。
                    _ => *f > 4,
                };
                f + g + i64::from(incorrect)
            }
            fn mapping(f: &i64, x: &i64) -> i64 {
                if *x == i64::MIN { *x } else { x + f }
            }
        }

        assert_eq!(
            check_map_monoid::<BrokenAdd<0>>(&[1], &[2, 4])
                .unwrap_err()
                .law,
            "mapping(id, x) == x"
        );
        assert_eq!(
            check_map_monoid::<BrokenAdd<1>>(&[1], &[2, 4])
                .unwrap_err()
                .law,
            "mapping(composition(id, f), x) == mapping(f, x)"
        );
        assert_eq!(
            check_map_monoid::<BrokenAdd<2>>(&[1], &[2, 4])
                .unwrap_err()
                .law,
            "mapping(composition(f, id), x) == mapping(f, x)"
        );
        assert_eq!(
            check_map_monoid::<BrokenAdd<3>>(&[1], &[2, 4])
                .unwrap_err()
                .law,
            "mapping(composition(composition(f, g), h), x) == mapping(composition(f, composition(g, h)), x)"
        );
    }

    #[test]
    fn test_map_monoid_overridden_operations() {
        enum Override<const IDENTITY: i64, const OFFSET: i64> {}
        impl<const IDENTITY: i64, const OFFSET: i64> MapMonoid for Override<IDENTITY, OFFSET> {
            type M = Additive<i64>;
            type F = ();
            fn identity_element() -> i64 {
                IDENTITY
            }
            fn binary_operation(a: &i64, b: &i64) -> i64 {
                a + b + OFFSET
            }
            fn identity_map() {}
            fn composition(_: &(), _: &()) {}
            fn mapping(_: &(), x: &i64) -> i64 {
                *x
            }
        }
        assert_eq!(
            check_map_monoid::<Override<1, 0>>(&[2], &[()])
                .unwrap_err()
                .law,
            "op(e, x) == x"
        );
        assert_eq!(
            check_map_monoid::<Override<0, 1>>(&[2], &[()])
                .unwrap_err()
                .law,
            "op(e, x) == x"
        );
        assert_eq!(check_map_monoid::<Override<0, 0>>(&[2], &[()]), Ok(()));
    }
}
