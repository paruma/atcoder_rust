#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Query {
    Update { x: usize, y: usize },
    Sum { l: usize, r: usize },
}
// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut xs: [usize; n],
    }
    let qs = (0..q)
        .map(|_| {
            input! {
                t: usize,
            }
            if t == 1 {
                input! {
                    x: Usize1,
                    y: usize,
                }
                Query::Update { x, y }
            } else {
                input! {
                    l: usize,
                    r: usize,
                }

                Query::Sum { l, r }
            }
        })
        .collect_vec();
    // y を考慮する必要がある
    let xs_max = {
        let mut xs_max = 0;
        for &x in &xs {
            xs_max = usize::max(xs_max, x);
        }
        for &q in &qs {
            match q {
                Query::Update { x, y } => {
                    xs_max = usize::max(xs_max, y);
                }
                Query::Sum { l, r } => {
                    xs_max = usize::max(xs_max, l);
                    xs_max = usize::max(xs_max, r);
                }
            }
        }
        xs_max
    };

    // FTree、配列外参照のえらーでないことがある？

    let mut cnts = FenwickTree::new(xs_max + 1, 0_i64);
    let mut vals = FenwickTree::new(xs_max + 1, 0_i64);

    for &x in &xs {
        cnts.add(x, 1);
        vals.add(x, x as i64);
    }
    for q in qs {
        // lg!(q);
        // lg!(&xs);
        // lg!(fenwick_tree_to_vec(&cnts, xs_max + 1));
        // lg!(fenwick_tree_to_vec(&vals, xs_max + 1));
        match q {
            Query::Update { x, y } => {
                //
                cnts.add(xs[x], -1);
                vals.add(xs[x], -(xs[x] as i64));

                xs[x] = y;

                cnts.add(xs[x], 1);
                vals.add(xs[x], xs[x] as i64);
            }
            Query::Sum { l, r } => {
                let ans = if l > r {
                    cnts.range_sum(..) * (l as i64)
                } else {
                    let term1 = cnts.range_sum(0..l) * (l as i64);
                    let term2 = vals.range_sum(l..r);
                    let term3 = cnts.range_sum(r..) * (r as i64);
                    term1 + term2 + term3
                };
                println!("{}", ans);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
use fenwick_tree::*;
#[allow(clippy::module_inception)]
pub mod fenwick_tree {
    use std::ops::{Bound, RangeBounds};
    #[derive(Clone, Debug)]
    pub struct FenwickTree<T> {
        n: usize,
        ary: Vec<T>,
        e: T,
    }
    impl<T: Clone + std::ops::AddAssign<T>> FenwickTree<T> {
        /// サイズ `n` の `FenwickTree` を作成します。
        /// # 計算量
        /// O(N)
        pub fn new(n: usize, e: T) -> Self {
            FenwickTree {
                n,
                ary: vec![e.clone(); n],
                e,
            }
        }
        /// スライスから `FenwickTree` を作成します。
        /// # 計算量
        /// O(N)
        pub fn from_slice(slice: &[T], e: T) -> Self {
            let n = slice.len();
            let mut ary = slice.to_vec();
            for i in 0..n {
                let j = i | (i + 1);
                if j < n {
                    let val_i = ary[i].clone();
                    ary[j] += val_i;
                }
            }
            FenwickTree { n, ary, e }
        }
        /// `[0, idx)` の区間の累積和を計算します。
        /// # 計算量
        /// O(log N)
        pub fn accum(&self, mut idx: usize) -> T {
            assert!(
                idx <= self.n,
                "FenwickTree::accum: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let mut sum = self.e.clone();
            while idx > 0 {
                sum += self.ary[idx - 1].clone();
                idx &= idx - 1;
            }
            sum
        }
        /// `idx`番目の要素に`val`を加算します。
        /// # 計算量
        /// O(log N)
        pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where
            T: std::ops::AddAssign<U>,
        {
            assert!(
                idx < self.n,
                "FenwickTree::add: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let n = self.n;
            idx += 1;
            while idx <= n {
                self.ary[idx - 1] += val.clone();
                idx += idx & idx.wrapping_neg();
            }
        }
        /// `[l, r)` の区間和を計算します。
        /// # 計算量
        /// O(log N)
        pub fn range_sum<R>(&self, range: R) -> T
        where
            T: std::ops::Sub<Output = T>,
            R: RangeBounds<usize>,
        {
            let r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                Bound::Unbounded => return self.accum(r),
            };
            assert!(
                l <= r && r <= self.n,
                "FenwickTree::range_sum: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            self.accum(r) - self.accum(l)
        }
        /// `idx`番目の要素の値を取得します。
        /// # 計算量
        /// O(log N)
        pub fn get(&self, idx: usize) -> T
        where
            T: std::ops::Sub<Output = T>,
        {
            assert!(
                idx < self.n,
                "FenwickTree::get: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            self.range_sum(idx..=idx)
        }
        /// `idx`番目の要素の値を`val`に設定します。
        /// # 計算量
        /// O(log N)
        pub fn set(&mut self, idx: usize, val: T)
        where
            T: std::ops::Sub<Output = T>,
        {
            assert!(
                idx < self.n,
                "FenwickTree::set: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let old_val = self.get(idx);
            self.add(idx, val - old_val);
        }
        /// Fenwick Treeの現在の状態を`Vec<T>`として返します。
        /// # 計算量
        /// O(N log N)
        pub fn to_vec(&self) -> Vec<T>
        where
            T: std::ops::Sub<Output = T>,
        {
            (0..self.n).map(|i| self.get(i)).collect()
        }
        /// Fenwick Treeが保持している要素の数を返します。
        /// # 計算量
        /// O(1)
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.n
        }
    }
}
