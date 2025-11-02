//! # Lazy Segment Tree (遅延セグメント木)
//!
//! ## 含まれる遅延セグメント木の一部
//!
//! ### `range_affine_range_sum`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素 `x` を `a * x + b` に更新する。
//! - 区間 `[L, R)` の要素の合計値を取得する。
//!
//! ### `range_add_range_sum`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素に値を加算する。
//! - 区間 `[L, R)` の要素の合計値を取得する。
//!
//! ### `range_add_range_max`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素に値を加算する。
//! - 区間 `[L, R)` の要素の最大値を取得する。
//!
//! ### `range_add_range_min`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素に値を加算する。
//! - 区間 `[L, R)` の要素の最小値を取得する。
//!
//! ### `range_affine_range_minmax`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素 `x` を `a * x + b` に更新する。
//! - 区間 `[L, R)` の各要素を特定の値 `x` に更新する。
//! - 区間 `[L, R)` の各要素に値を加算する。
//! - 区間 `[L, R)` の要素の最小値を取得する。
//! - 区間 `[L, R)` の要素の最大値を取得する。
//!
//! ### `range_chmax_range_max`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素を値 `x` との最大値で更新する。
//! - 区間 `[L, R)` の要素の最大値を取得する。
//!
//! ### `range_chmin_range_min`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素を値 `x` との最小値で更新する。
//! - 区間 `[L, R)` の要素の最小値を取得する。
//!
//! ### `range_chmin_chmax_add_range_min`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素を値 `x` との最小値で更新する。
//! - 区間 `[L, R)` の各要素を値 `x` との最大値で更新する。
//! - 区間 `[L, R)` の各要素に値を加算する。
//! - 区間 `[L, R)` の要素の最小値を取得する。
//!
//! ### `range_chmin_chmax_range_min`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素を値 `x` との最小値で更新する。
//! - 区間 `[L, R)` の各要素を値 `x` との最大値で更新する。
//! - 区間 `[L, R)` の要素の最小値を取得する。
//!
//! ### `range_linear_add_range_sum`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素 `i` に対して、`init + diff * (i - L)` を加算する。
//! - 区間 `[L, R)` の要素の合計値を取得する。
//!
//! ### `range_linear_update_range_sum`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素 `i` を `init + diff * (i - L)` に更新する。
//! - 区間 `[L, R)` の要素の合計値を取得する。
//!
//! ### `range_quadratic_add_range_sum`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素 `i` に対して、`coef0 + coef1 * (i - L) + coef2 * (i - L)^2` を加算する。
//! - 区間 `[L, R)` の要素の合計値を取得する。
//!
//! ### `range_update_range_sum`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素を特定の値 `x` に更新する。
//! - 区間 `[L, R)` の要素の合計値を取得する。
//!
//! ### `two_sequence_range_affine_range_sum`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素 `(xs[i], ys[i])` に対して、`xs[i] ← a * xs[i] + b`, `ys[i] ← c * ys[i] + d` のアフィン変換を適用する。
//! - 区間 `[L, R)` の `sum(xs[i] * ys[i])` を取得する。
//! - 区間 `[L, R)` の `sum(xs[i])` を取得する。
//! - 区間 `[L, R)` の `sum(ys[i])` を取得する。
//!
//! ### `two_sequence_range_affine_range_sum_of_quadratic`
//!
//! 次のクエリが処理できます。
//! - 区間 `[L, R)` の各要素 `(xs[i], ys[i])` に対して、`xs[i] ← a * xs[i] + b`, `ys[i] ← c * ys[i] + d` のアフィン変換を適用する。
//! - 区間 `[L, R)` の `sum(xs[i] * ys[i])` を取得する。
//! - 区間 `[L, R)` の `sum(xs[i] * xs[i])` を取得する。
//! - 区間 `[L, R)` の `sum(ys[i] * ys[i])` を取得する。
//! - 区間 `[L, R)` の `sum(xs[i])` を取得する。
//! - 区間 `[L, R)` の `sum(ys[i])` を取得する。
//!
//! <!-- 他の遅延セグメント木の説明がここに追加されます -->

use cargo_snippet::snippet;
pub mod map_monoid_template;
pub mod range_add_range_max;
pub mod range_add_range_min;
pub mod range_add_range_sum;
pub mod range_affine_range_minmax;
pub mod range_affine_range_sum;
pub mod range_affine_range_sum_of_square;
pub mod range_chmax_range_max;
pub mod range_chmin_chmax_add_range_min;
pub mod range_chmin_chmax_range_min;
pub mod range_chmin_range_min;
pub mod range_linear_add_range_sum;
pub mod range_linear_update_range_sum;
pub mod range_quadratic_add_range_sum;
pub mod range_update_range_sum;
pub mod two_sequence_range_affine_range_sum;
pub mod two_sequence_range_affine_range_sum_of_quadratic;

#[snippet]
pub fn lazy_segtree_to_vec<F: ac_library::MapMonoid>(
    seg: &mut ac_library::LazySegtree<F>,
    len: usize,
) -> Vec<<F::M as ac_library::Monoid>::S> {
    (0..len).map(|i| seg.get(i)).collect()
}
