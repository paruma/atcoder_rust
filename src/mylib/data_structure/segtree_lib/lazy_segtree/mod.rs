//! # Lazy Segment Tree (遅延セグメント木)
//!
//! - `range_add_range_sum`: 加算 / 和
//! - `range_add_range_max`: 加算 / 最大値
//! - `range_add_range_min`: 加算 / 最小値
//! - `range_affine_range_sum`: アフィン変換 ($ax+b$) / 和
//! - `range_affine_range_sum_of_square`: アフィン変換 / 和・2乗和
//! - `range_affine_range_minmax`: アフィン変換 / 最小値・最大値
//! - `range_chmax_range_max`: chmax / 最大値
//! - `range_chmin_range_min`: chmin / 最小値
//! - `range_chmin_chmax_add_range_min_max`: chmin, chmax, 加算 / 最小値・最大値
//! - `range_chmin_chmax_affine_range_min_max`: chmin, chmax, アフィン変換 / 最小値・最大値
//! - `range_chmin_chmax_range_min_max`: chmin, chmax / 最小値・最大値
//! - `range_div_ceil_range_min_max`: $\lceil x/d \rceil$ / 最小値・最大値
//! - `range_div_floor_range_max`: $\lfloor x/d \rfloor$ / 最大値
//! - `range_div_floor_range_min_max`: $\lfloor x/d \rfloor$ / 最小値・最大値
//! - `range_linear_add_range_sum`: 一次式加算 ($x_i \leftarrow x_i + ai + b$) / 和
//! - `range_linear_update_range_sum`: 一次式更新 ($x_i \leftarrow ai + b$) / 和
//! - `range_quadratic_add_range_sum`: 二次式加算 ($x_i \leftarrow x_i + ai^2 + bi + c$) / 和
//! - `range_update_range_sum`: 更新 / 和
//! - `range_update_range_prod`: 更新 / 積
//! - `range_mult_range_prod`: 乗算 / 積
//! - `range_update_range_xor`: 更新 / XOR和
//! - `range_xor_apply_range_xor`: XOR / XOR和
//! - `two_sequence_range_affine_range_sum`: 2変数アフィン変換 ($x \leftarrow ax+b, y \leftarrow cy+d$) / $\sum xy, \sum x, \sum y$
//! - `two_sequence_range_affine_range_sum_of_quadratic`: 2変数アフィン変換 / $\sum xy, \sum x^2, \sum y^2, \sum x, \sum y$

use cargo_snippet::snippet;
pub mod map_monoid_template;
pub mod range_add_range_max;
pub mod range_add_range_min;
pub mod range_add_range_sum;
pub mod range_affine_range_minmax;
pub mod range_affine_range_sum;
pub mod range_affine_range_sum_of_square;
pub mod range_chmax_range_max;
pub mod range_chmin_chmax_add_range_min_max;
pub mod range_chmin_chmax_affine_range_min_max;
pub mod range_chmin_chmax_range_min_max;
pub mod range_chmin_range_min;
pub mod range_div_ceil_range_min_max;
pub mod range_div_floor_range_max;
pub mod range_div_floor_range_min_max;
pub mod range_linear_add_range_sum;
pub mod range_linear_update_range_sum;
pub mod range_mult_range_prod;
pub mod range_quadratic_add_range_sum;
pub mod range_update_range_prod;
pub mod range_update_range_sum;
pub mod range_update_range_xor;
pub mod range_xor_apply_range_xor;
pub mod two_sequence_range_affine_range_sum;
pub mod two_sequence_range_affine_range_sum_of_quadratic;

#[snippet]
pub fn lazy_segtree_to_vec<F: ac_library::MapMonoid>(
    seg: &mut ac_library::LazySegtree<F>,
    len: usize,
) -> Vec<<F::M as ac_library::Monoid>::S> {
    (0..len).map(|i| seg.get(i)).collect()
}
