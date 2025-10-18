fn solve1(nv: usize, es: &[(usize, usize)]) -> usize {
    // サイクルを考える
    let es = es
        .iter()
        .copied()
        .map(|(u, v)| (u.min(v), u.max(v)))
        .collect::<HashSet<_>>();

    let ans0 = (0..nv)
        .permutations(nv)
        .map(|ps| {
            let dst_es = ps
                .iter()
                .copied()
                .circular_tuple_windows()
                .map(|(u, v)| (u.min(v), u.max(v)))
                .collect::<HashSet<_>>();

            es.symmetric_difference(&dst_es).count()
        })
        .min()
        .unwrap();

    let ans1 = if nv < 6 {
        None
    } else {
        // ちょっと無駄な計算をしてる
        let tmp = (3..=nv - 3)
            .map(|c1| {
                (0..nv)
                    .permutations(nv)
                    .map(|ps| {
                        let g1 = &ps[0..c1];
                        let g2 = &ps[c1..];

                        let dst_es1 = g1
                            .iter()
                            .copied()
                            .circular_tuple_windows()
                            .map(|(u, v)| (u.min(v), u.max(v)))
                            .collect::<HashSet<_>>();
                        let dst_es2 = g2
                            .iter()
                            .copied()
                            .circular_tuple_windows()
                            .map(|(u, v)| (u.min(v), u.max(v)))
                            .collect::<HashSet<_>>();

                        let dst_es = dst_es1.union(&dst_es2).copied().collect::<HashSet<_>>();
                        es.symmetric_difference(&dst_es).count()
                    })
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap();
        Some(tmp)
    };

    [Some(ans0), ans1].iter().copied().flatten().min().unwrap()
}

fn solve2(nv: usize, es: &[(usize, usize)]) -> usize {
    // 28C8 通り全探索
    let es = es
        .iter()
        .copied()
        .map(|(u, v)| (u.min(v), u.max(v)))
        .collect::<HashSet<_>>();

    let all_es: Vec<(usize, usize)> = (0..nv).tuple_combinations().collect_vec();

    all_es
        .iter()
        .combinations(nv)
        .filter(|target_es| {
            // target_es でできるグラフがすべて次数2の単純無向グラフか？
            let mut degs = vec![0_i64; nv];
            for &&(u, v) in target_es {
                degs[u] += 1;
                degs[v] += 1;
            }

            degs.iter().all(|d| *d == 2)
        })
        .map(|target_es| {
            let target_es = target_es.iter().copied().copied().collect::<HashSet<_>>();

            es.symmetric_difference(&target_es).count()
        })
        .min()
        .unwrap()
}

fn main() {
    input! {
        nv: usize,
        ne: usize,
        es: [(Usize1, Usize1); ne],
    }

    let ans = solve2(nv, &es);

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

// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
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
