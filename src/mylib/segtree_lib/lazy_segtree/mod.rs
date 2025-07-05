use cargo_snippet::snippet;

pub mod map_monoid_template;
pub mod range_affine_range_minmax;
pub mod range_affine_range_sum;
pub mod range_chmax_range_max;
pub mod range_chmin_range_min;

#[snippet]
pub fn lazy_segtree_to_vec<F: ac_library::MapMonoid>(
    seg: &mut ac_library::LazySegtree<F>,
    len: usize,
) -> Vec<<F::M as ac_library::Monoid>::S> {
    (0..len).map(|i| seg.get(i)).collect()
}
