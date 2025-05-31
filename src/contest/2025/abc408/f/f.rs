fn main() {
    input! {
        n: usize,
        d: usize,
        r: usize,
        hs: [Usize1; n],
    }

    let inv_hs = hs
        .iter()
        .copied()
        .enumerate()
        .fold(vec![usize::MAX; n], |mut acc, (i, x)| {
            acc[x] = i;
            acc
        });

    // dbg!(&inv_hs);

    let mut seg = LazySegtree::<RangeChmaxRangeMax>::from(vec![0; n]);

    let mut dp = vec![i64::MAX; n];

    for h in 0..d {
        if h < n {
            dp[inv_hs[h]] = 0;
            // seg.set(inv_hs[h], 0);
        }
    }

    // dbg!(&dp);

    // dbg!(lazy_segtree_to_vec(&mut dp, n));

    for &i in &inv_hs {
        let current = dp[i];
        // dbg!(current);

        // [i-r, i), [i+1, i+r]

        let begin1 = i.saturating_sub(r);
        let end1 = i;
        let begin2 = usize::min(i + 1, n - 1);
        let end_inclusive2 = usize::min(i + r, n - 1);

        seg.apply_range(begin1..end1, current + 1); // chmax
        seg.apply_range(begin2..=end_inclusive2, current + 1); // chmax

        // if hs[i] >= d {
        //     seg.set(inv_hs[hs[i] - d], dp[inv_hs[hs[i] - d]]);
        // }

        // seg.set(i, i64::MAX);
        if hs[i] + d < n {
            let idx = inv_hs[hs[i] + d];
            dp[idx] = seg.get(idx);
        }

        // dbg!(lazy_segtree_to_vec(&mut seg, n));
    }

    // dbg!(&dp);

    let ans: i64 = dp.iter().copied().max().unwrap();
    println!("{}", ans);
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
}

use ac_library::LazySegtree;
// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
use range_chmax_range_max::*;
pub mod range_chmax_range_max {
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::Max;
    use std::convert::Infallible;

    pub struct RangeChmaxRangeMax(Infallible);
    impl MapMonoid for RangeChmaxRangeMax {
        type M = Max<i64>;
        type F = i64;
        fn identity_map() -> Self::F {
            i64::MIN
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            *f.max(x)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            *f.max(g)
        }
    }
}

pub fn lazy_segtree_to_vec<F: ac_library::MapMonoid>(
    seg: &mut ac_library::LazySegtree<F>,
    len: usize,
) -> Vec<<F::M as ac_library::Monoid>::S> {
    (0..len).map(|i| seg.get(i)).collect()
}
