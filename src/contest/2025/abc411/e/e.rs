use ac_library::{ModInt998244353 as Mint, Segtree};

fn solve1(n: usize, xss: &[Vec<i64>]) -> Mint {
    // 確率をセグ木で計算
    let xss = xss
        .iter()
        .map(|xs| xs.iter().copied().sorted().collect_vec())
        .collect_vec();

    // table! {&xss};

    let mut val_to_pos: HashMap<i64, Vec<(usize, usize)>> = HashMap::new();

    for d in 0..n {
        for i in 0..6 {
            val_to_pos.entry(xss[d][i]).or_default().push((d, i));
        }
    }

    // 各サイコロで、各時点での値以下の目の数
    let mut cnts2 = Segtree::<MintMultiplicative<Mint>>::from(vec![Mint::new(0); n]);
    // cnts[v] = 最大値が v 以下の場合の数
    let mut cnts = HashMap::new();
    for &v in val_to_pos.keys().sorted() {
        for &(d, _i) in &val_to_pos[&v] {
            cnts2.set(d, cnts2.get(d) + 1);
        }
        cnts.insert(v, cnts2.all_prod());
        //
    }

    cnts.insert(0, Mint::new(0));
    // dbg!(&cnts);

    let pow6 = Mint::new(6).pow(n as u64);
    let inv_pow6 = pow6.inv();
    let cnts = cnts
        .iter()
        .map(|(k, v)| (*k, *v))
        .sorted_by_key(|(k, _)| *k)
        .collect_vec();

    // dbg!(val_to_pos);
    // dbg!(min_max);
    // dbg!(&cnts);

    cnts.iter()
        .tuple_windows()
        .map(|((k1, cnt1), (k2, cnt2))| {
            let prob = (cnt2 - cnt1) * inv_pow6;
            Mint::new(*k2) * prob
        })
        .sum::<Mint>()
}

fn solve2(n: usize, xss: &[Vec<i64>]) -> Mint {
    // 確率を差分計算で計算
    let xss = xss
        .iter()
        .map(|xs| xs.iter().copied().sorted().collect_vec())
        .collect_vec();

    // table! {&xss};

    let mut val_to_pos: HashMap<i64, Vec<(usize, usize)>> = HashMap::new();

    for d in 0..n {
        for i in 0..6 {
            val_to_pos.entry(xss[d][i]).or_default().push((d, i));
        }
    }

    let min_max = xss.iter().map(|xs| xs[0]).max().unwrap();

    let mut cnts: HashMap<i64, Mint> = HashMap::new();
    let mut tmp = Mint::new(0);
    for &v in val_to_pos.keys().sorted() {
        if v < min_max {
            tmp = Mint::new(0);
            cnts.insert(v, Mint::new(0));
        } else if v == min_max {
            let cnt = xss
                .iter()
                .map(|xs| {
                    let cnt = xs.iter().copied().filter(|&x| x <= v).count();
                    Mint::new(cnt)
                })
                .product::<Mint>();
            cnts.insert(v, cnt);
            tmp = cnt;
        } else {
            //
            for &(d, i) in &val_to_pos[&v] {
                // dbg!(min_max, v, d, i);
                tmp *= Mint::new(i + 1) / Mint::new(i);
            }
            cnts.insert(v, tmp);
        }
        //
    }

    cnts.insert(0, Mint::new(0));
    // dbg!(&cnts);

    let pow6 = Mint::new(6).pow(n as u64);
    let inv_pow6 = pow6.inv();
    let cnts = cnts
        .iter()
        .map(|(k, v)| (*k, *v))
        .sorted_by_key(|(k, _)| *k)
        .collect_vec();

    // dbg!(val_to_pos);
    // dbg!(min_max);
    // dbg!(&cnts);

    cnts.iter()
        .tuple_windows()
        .map(|((k1, cnt1), (k2, cnt2))| {
            let prob = (cnt2 - cnt1) * inv_pow6;
            Mint::new(*k2) * prob
        })
        .sum::<Mint>()
}

fn main() {
    input! {
        n: usize,
        xss: [[i64; 6]; n],
    }

    let ans = solve1(n, &xss);

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
use monoid_modint::*;
pub mod monoid_modint {
    use ac_library::{Monoid, modint::ModIntBase};
    use std::{convert::Infallible, marker::PhantomData};
    pub struct MintAdditive<Mint: ModIntBase>(Infallible, PhantomData<fn() -> Mint>);
    impl<Mint> Monoid for MintAdditive<Mint>
    where
        Mint: ModIntBase,
    {
        type S = Mint;
        fn identity() -> Self::S {
            Mint::raw(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct MintMultiplicative<Mint: ModIntBase>(Infallible, PhantomData<fn() -> Mint>);
    impl<Mint> Monoid for MintMultiplicative<Mint>
    where
        Mint: ModIntBase,
    {
        type S = Mint;
        fn identity() -> Self::S {
            Mint::raw(1)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a * *b
        }
    }
}
