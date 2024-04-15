enum Query {
    Update {
        // x 文字目を c に変更する
        x: usize,
        c: u8,
    },
    Find {
        // l 文字目から r文字目までが回分か判定する
        l: usize,
        r: usize,
    },
}
//#[derive_readable]
struct Problem {
    str_len: usize,
    nq: usize,
    str: Vec<u8>,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            str_len: usize,
            nq: usize,
            str: Bytes,
        }
        let qs = (0..nq)
            .map(|_| {
                input! {
                    q: u8
                }
                if q == 1 {
                    input! {
                        x: Usize1,
                        c: Bytes
                    }
                    Query::Update { x, c: c[0] }
                } else {
                    input! {
                        l: Usize1,
                        r: Usize1
                    }
                    Query::Find { l, r }
                }
            })
            .collect_vec();
        Problem {
            str_len,
            nq,
            str,
            qs,
        }
    }

    fn solve3(&self) -> Answer {
        // ロリハの直積
        use ac_library::segtree::Segtree;
        let Problem {
            str_len,
            nq,
            str,
            qs,
        } = self;

        let ctoi = |c: u8| (c - b'a') as i64;

        let rev_i = |i: usize| str_len - 1 - i;
        let normal_xs = str.iter().copied().map(ctoi).collect_vec();

        let reverse_xs = normal_xs.iter().copied().rev().collect_vec();

        let unit = RollingHash::unit(100);

        // 文字列の正順と逆順
        let normal_rh = normal_xs.iter().copied().map(&unit).collect_vec();
        let reverse_rh = reverse_xs.iter().copied().map(&unit).collect_vec();

        let mut normal_seg_tree = Segtree::<RollingHashConcat>::from(normal_rh);
        let mut reverse_seg_tree = Segtree::<RollingHashConcat>::from(reverse_rh);

        let mut ans = vec![];
        for q in qs {
            match q {
                Query::Update { x, c } => {
                    normal_seg_tree.set(*x, unit(ctoi(*c)));
                    reverse_seg_tree.set(rev_i(*x), unit(ctoi(*c)));
                }
                Query::Find { l, r } => {
                    let rev_l = rev_i(*l);
                    let rev_r = rev_i(*r);
                    let is_palindrome =
                        normal_seg_tree.prod(l..=r) == reverse_seg_tree.prod(rev_r..=rev_l);
                    ans.push(is_palindrome);
                }
            }
        }

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<bool>,
}

impl Answer {
    fn print(&self) {
        for a in &self.ans {
            print_yesno(*a);
        }
    }
}

fn main() {
    Problem::read().solve3().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use itertools::izip;
// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
use monoid_rolling_hash::*;
pub mod monoid_rolling_hash {
    use std::convert::Infallible;
    const MOD: i64 = (1 << 61) - 1;
    const MOD_I128: i128 = (1 << 61) - 1;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ModInt261M1 {
        val: i64,
    }
    impl ModInt261M1 {
        #[inline]
        pub fn new(val: i64) -> Self {
            Self { val }
        }
    }
    impl std::ops::Add for ModInt261M1 {
        type Output = Self;
        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            let mut x = self.val + rhs.val;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }
    impl std::ops::Sub for ModInt261M1 {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            let mut x = MOD + self.val - rhs.val;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }
    impl std::ops::Mul for ModInt261M1 {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            let x = (self.val as i128) * (rhs.val as i128);
            let mut x = ((x >> 61) + (x & MOD_I128)) as i64;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }
    use ac_library::Monoid;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RollingHash {
        hash: ModInt261M1,
        base: ModInt261M1,
    }
    impl RollingHash {
        pub fn get_hash(&self) -> i64 {
            self.hash.val as i64
        }
        pub fn unit(base: i64) -> impl (Fn(i64) -> RollingHash) {
            move |x| RollingHash {
                hash: ModInt261M1::new(x),
                base: ModInt261M1::new(base),
            }
        }
        pub fn new(hash: i64, base: i64) -> Self {
            Self {
                hash: ModInt261M1::new(hash),
                base: ModInt261M1::new(base),
            }
        }
    }
    pub struct RollingHashConcat(Infallible);
    impl Monoid for RollingHashConcat {
        type S = RollingHash;
        fn identity() -> Self::S {
            RollingHash {
                hash: ModInt261M1::new(0),
                base: ModInt261M1::new(1),
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RollingHash {
                hash: a.hash * b.base + b.hash,
                base: a.base * b.base,
            }
        }
    }
}
