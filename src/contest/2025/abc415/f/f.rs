use ac_library::Segtree;
use monoid_template::*;
#[allow(unused_variables)]
pub mod monoid_template {
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub len: usize,
        pub prefix_ch: char,
        pub prefix_cnt: usize,
        pub max_cnt: usize,
        pub suffix_ch: char,
        pub suffix_cnt: usize,
    }
    impl RangeXxx {
        pub fn unit(x: char) -> Self {
            Self {
                len: 1,
                prefix_ch: x,
                prefix_cnt: 1,
                max_cnt: 1,
                suffix_ch: x,
                suffix_cnt: 1,
            }
        }
    }
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx {
                len: 0,
                prefix_ch: '?',
                prefix_cnt: 0,
                max_cnt: 0,
                suffix_ch: '?',
                suffix_cnt: 1,
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            if a.prefix_ch == '?' {
                return *b;
            }
            if b.prefix_ch == '?' {
                return *a;
            }

            let len = a.len + b.len;

            let prefix_ch = a.prefix_ch;
            let prefix_cnt = a.prefix_cnt
                + if a.prefix_cnt == a.len && a.prefix_ch == b.prefix_ch {
                    b.prefix_cnt
                } else {
                    0
                };

            let max_cnt = {
                let sub = if a.suffix_ch == b.prefix_ch {
                    a.suffix_cnt + b.prefix_cnt
                } else {
                    0
                };

                usize::max(sub, usize::max(a.max_cnt, b.max_cnt))
            };

            let suffix_ch = b.suffix_ch;
            let suffix_cnt = b.suffix_cnt
                + if b.suffix_cnt == b.len && b.suffix_ch == a.suffix_ch {
                    a.suffix_cnt
                } else {
                    0
                };

            RangeXxx {
                len,
                prefix_ch,
                prefix_cnt,
                max_cnt,
                suffix_ch,
                suffix_cnt,
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xs: Chars,
    }

    let mut seg =
        Segtree::<RangeXxxMonoid>::from(xs.iter().copied().map(RangeXxx::unit).collect_vec());

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                i: Usize1,
                x: char,
            }
            seg.set(i, RangeXxx::unit(x));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            let ans = seg.prod(l..=r).max_cnt;
            println!("{}", ans)
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
