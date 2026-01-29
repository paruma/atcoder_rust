mod bench_util;

use mylib::data_structure::segtree_lib::fenwick_tree::range_sum_fenwick_tree::range_sum_fenwick_tree::RangeSumFenwickTree;
use mylib::data_structure::segtree_lib::segtree::range_sum_segtree::range_sum_segtree::RangeSumSegtree;
use rand::{Rng, SeedableRng, rngs::StdRng};

enum Query {
    Set { p: usize, val: i64 },
    Get { p: usize },
    RangeSum { l: usize, r: usize },
}

fn main() {
    let n = 1_000_000;
    let q = 1_000_000;
    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    println!(
        "Generating {} queries for N = {} (Range Sum Comparison)...",
        q, n
    );
    let queries: Vec<Query> = (0..q)
        .map(|_| {
            let op = rng.random_range(0..3);
            let l = rng.random_range(0..n);
            let r = rng.random_range(l..=n);
            match op {
                0 => Query::Set {
                    p: rng.random_range(0..n),
                    val: rng.random_range(-100..=100),
                },
                1 => Query::Get {
                    p: rng.random_range(0..n),
                },
                2 => Query::RangeSum { l, r },
                _ => unreachable!(),
            }
        })
        .collect();

    println!("Running Benchmarks (Release mode required)\n");

    // 1. Fenwick Tree (BIT)
    // BIT の set は get + add で実装されている
    let bit = RangeSumFenwickTree::<i64>::new(n);
    let res_bit = bench_util::run_bench("Fenwick Tree (BIT)", bit, &queries, |bit, q| match *q {
        Query::Set { p, val } => {
            bit.set(p, val);
            0
        }
        Query::Get { p } => bit.get(p),
        Query::RangeSum { l, r } => bit.range_sum(l..r),
    });

    // 2. Segment Tree
    let seg = RangeSumSegtree::<i64>::new(n);
    let res_seg = bench_util::run_bench("Segment Tree", seg, &queries, |seg, q| match *q {
        Query::Set { p, val } => {
            seg.set(p, val);
            0
        }
        Query::Get { p } => seg.get(p),
        Query::RangeSum { l, r } => seg.range_sum(l..r),
    });

    // 結果表示
    bench_util::print_results(&[res_bit, res_seg]);
}
