use std::hint::black_box;
use std::time::{Duration, Instant};

use mylib::data_structure::{
    bucket_multiset::bucket_multiset::BucketMultiSet,
    fenwick_tree::fenwick_tree_sparse_multiset::fenwick_tree_sparse_multiset::FenwickTreeSparseMultiset,
    treap_multiset::treap_multiset::TreapMultiSet,
};

trait Multiset {
    fn insert(&mut self, value: i64);
    fn remove(&mut self, value: i64) -> bool;
    fn nth(&self, rank: usize) -> i64;
    fn count_range(&self, start: i64, end: i64) -> usize;
}

impl Multiset for BucketMultiSet<i64> {
    fn insert(&mut self, value: i64) { self.insert(value); }
    fn remove(&mut self, value: i64) -> bool { self.remove1(&value) }
    fn nth(&self, rank: usize) -> i64 { *self.nth_min(rank).unwrap() }
    fn count_range(&self, start: i64, end: i64) -> usize { self.count_in_range(start..end) }
}

impl Multiset for TreapMultiSet<i64> {
    fn insert(&mut self, value: i64) { self.insert(value); }
    fn remove(&mut self, value: i64) -> bool { self.remove1(&value) }
    fn nth(&self, rank: usize) -> i64 { *self.nth_min(rank).unwrap() }
    fn count_range(&self, start: i64, end: i64) -> usize { self.count_in_range(start..end) }
}

impl Multiset for FenwickTreeSparseMultiset {
    fn insert(&mut self, value: i64) { self.insert(value); }
    fn remove(&mut self, value: i64) -> bool { self.remove1(value) }
    fn nth(&self, rank: usize) -> i64 { self.nth_min(rank).unwrap() }
    fn count_range(&self, start: i64, end: i64) -> usize { self.count_in_range(start..end) }
}

fn run<T: Multiset>(
    init: impl FnOnce() -> T,
    values: &[i64],
    window: usize,
    range: bool,
) -> (Duration, Duration, Vec<i64>) {
    let mut output = Vec::with_capacity(values.len());
    let start = Instant::now();
    let mut set = init();
    let setup = start.elapsed();
    let start = Instant::now();
    for (i, &v) in values.iter().enumerate() {
        if i >= window {
            assert!(set.remove(values[i - window]));
        }
        set.insert(v);
        let answer = if range {
            set.count_range(v / 2, v / 2 + 256) as i64
        } else {
            set.nth((i + 1).min(window) / 2)
        };
        output.push(answer);
    }
    let operations = start.elapsed();
    black_box(&set);
    (setup, operations, output)
}

fn main() {
    const N: usize = 1_000_000;
    const TRIALS: usize = 7;
    for (name, window, dense, range) in [
        ("grow-wide-median", N, false, false),
        ("grow-1024-median", N, true, false),
        ("window-1024-wide-median", 1024, false, false),
        ("grow-1024-range", N, true, true),
    ] {
        let mut state = 7829u64;
        let values: Vec<_> = (0..N).map(|_| {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let value = (state >> 33) as i64;
            if dense { value % 1024 } else { value }
        }).collect();
        let mut timings = [const { Vec::new() }; 3];
        for trial in 0..TRIALS {
            let mut reference = None;
            for offset in 0..3 {
                let index = (trial + offset) % 3;
                let (setup, ops, output) = match index {
                    0 => run(TreapMultiSet::new, &values, window, range),
                    1 => run(|| FenwickTreeSparseMultiset::new(&values), &values, window, range),
                    _ => run(BucketMultiSet::new, &values, window, range),
                };
                if let Some(reference) = &reference {
                    assert_eq!(reference, &output, "{name}: trial {trial}, implementation {index}");
                } else {
                    reference = Some(output);
                }
                timings[index].push([setup, ops, setup + ops]);
            }
        }
        for (name_impl, times) in ["Treap", "Fenwick", "Bucket"].into_iter().zip(timings) {
            let medians = [0, 1, 2].map(|column| {
                let mut sorted: Vec<_> = times.iter().map(|t| t[column]).collect();
                sorted.sort_unstable();
                sorted[TRIALS / 2].as_secs_f64() * 1000.0
            });
            println!("{name} {name_impl}: build={:.2} ms ops={:.2} ms total={:.2} ms", medians[0], medians[1], medians[2]);
        }
    }
}
