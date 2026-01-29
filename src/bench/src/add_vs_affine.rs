mod bench_util;

use mylib::data_structure::segtree_lib::lazy_segtree::range_add_range_sum::range_add_range_sum::RangeAddRangeSumSegtree;
use mylib::data_structure::segtree_lib::lazy_segtree::range_affine_range_sum::range_affine_range_sum::RangeAffineRangeSumSegtree;
use rand::{Rng, SeedableRng, rngs::StdRng};

enum Query {
    Add { l: usize, r: usize, val: i64 },
    RangeSum { l: usize, r: usize },
}

fn main() {
    let n = 1_000_000;
    let q = 1_000_000;
    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    println!("Generating {} queries for N = {} (Add vs Affine)...", q, n);
    let queries: Vec<Query> = (0..q)
        .map(|_| {
            let op = rng.random_range(0..2);
            let l = rng.random_range(0..n);
            let r = rng.random_range(l..=n);
            match op {
                0 => Query::Add {
                    l,
                    r,
                    val: rng.random_range(-100..=100),
                },
                1 => Query::RangeSum { l, r },
                _ => unreachable!(),
            }
        })
        .collect();

    println!("Running Benchmarks (Release mode required)\n");

    // 1. Range Add Range Sum Segtree
    let add_seg = RangeAddRangeSumSegtree::<i64>::new(n);
    let res_add =
        bench_util::run_bench("Range Add Segtree", add_seg, &queries, |seg, q| match *q {
            Query::Add { l, r, val } => {
                seg.range_add(l..r, val);
                0
            }
            Query::RangeSum { l, r } => seg.range_sum(l..r),
        });

    // 2. Range Affine Range Sum Segtree (Slope = 1)
    let affine_seg = RangeAffineRangeSumSegtree::<i64>::new(n);
    let res_affine = bench_util::run_bench(
        "Range Affine Segtree",
        affine_seg,
        &queries,
        |seg, q| match *q {
            Query::Add { l, r, val } => {
                seg.range_affine(l..r, 1, val); // 傾き1で固定
                0
            }
            Query::RangeSum { l, r } => seg.range_sum(l..r),
        },
    );

    // 結果表示
    bench_util::print_results(&[res_add, res_affine]);
}
