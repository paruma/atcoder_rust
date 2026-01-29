mod bench_util;

use mylib::data_structure::segtree_lib::lazy_segtree::range_chmin_chmax_add_range_min_max::range_chmin_chmax_add_range_min_max::RangeChminChmaxAddRangeMinMaxSegtree;
use mylib::data_structure::segtree_lib::segtree_beats::range_chmin_chmax_add_range_sum_beats::range_chmin_chmax_add_range_sum_beats::RangeChminChmaxAddRangeSumSegtree;
use rand::{rngs::StdRng, Rng, SeedableRng};

enum Query {
    Chmin { l: usize, r: usize, val: i64 },
    Chmax { l: usize, r: usize, val: i64 },
    Add { l: usize, r: usize, val: i64 },
    Range { l: usize, r: usize },
}

fn main() {
    let n = 1_000_000;
    let q = 1_000_000;
    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    println!("Generating {} queries for N = {} (Beats! vs Lazy)...", q, n);
    let queries: Vec<Query> = (0..q)
        .map(|_| {
            let op = rng.random_range(0..4);
            let l = rng.random_range(0..n);
            let r = rng.random_range(l..=n);
            let val = rng.random_range(-100..=100);
            match op {
                0 => Query::Chmin { l, r, val },
                1 => Query::Chmax { l, r, val },
                2 => Query::Add { l, r, val },
                3 => Query::Range { l, r },
                _ => unreachable!(),
            }
        })
        .collect();

    println!("Running Benchmarks (Release mode required)\n");

    // 1. Segtree Beats!
    // Range Chmin / Chmax / Add / Range Sum
    let beats = RangeChminChmaxAddRangeSumSegtree::new(n);
    let res_beats = bench_util::run_bench("Segtree Beats!", beats, &queries, |beats, q| match *q {
        Query::Chmin { l, r, val } => {
            beats.chmin(l..r, val);
            0
        }
        Query::Chmax { l, r, val } => {
            beats.chmax(l..r, val);
            0
        }
        Query::Add { l, r, val } => {
            beats.add(l..r, val);
            0
        }
        Query::Range { l, r } => beats.sum(l..r), // Range Sum
    });

    // 2. Lazy Segment Tree
    // Range Chmin / Chmax / Add / Range Min & Max
    let lazy = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::new(n);
    let res_lazy = bench_util::run_bench("Lazy Segment Tree", lazy, &queries, |lazy, q| match *q {
        Query::Chmin { l, r, val } => {
            lazy.range_chmin(l..r, val);
            0
        }
        Query::Chmax { l, r, val } => {
            lazy.range_chmax(l..r, val);
            0
        }
        Query::Add { l, r, val } => {
            lazy.range_add(l..r, val);
            0
        }
        Query::Range { l, r } => lazy.range_min(l..r), // Range Min
    });

    // 結果表示
    bench_util::print_results(&[res_lazy, res_beats]);
}
