use cargo_snippet::snippet;

pub mod map_monoid_template;
pub mod range_add_range_max;
pub mod range_add_range_min;
pub mod range_add_range_sum;
pub mod range_affine_range_minmax;
pub mod range_affine_range_sum;
pub mod range_chmax_range_max;
pub mod range_chmin_chmax_add_range_min;
pub mod range_chmin_chmax_range_min;
pub mod range_chmin_range_min;
pub mod range_linear_add_range_sum;
pub mod range_linear_update_range_sum;
pub mod range_quadratic_add_range_sum;
pub mod range_update_range_sum;
pub mod two_sequence_range_affine_range_sum;

#[snippet]
pub fn lazy_segtree_to_vec<F: ac_library::MapMonoid>(
    seg: &mut ac_library::LazySegtree<F>,
    len: usize,
) -> Vec<<F::M as ac_library::Monoid>::S> {
    (0..len).map(|i| seg.get(i)).collect()
}
