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
    fn solve(&self) -> Answer {
        use ac_library::segtree::Segtree;
        use ac_library::ModInt998244353 as Mint;
        let Problem {
            str_len,
            nq,
            str,
            qs,
        } = self;

        let ctoi = |c: u8| c - b'a';

        let base_list: [i64; 3] = [100_000_001, 321_432_543, 987_654_321];
        let make_rh = |ch: u8, base: i64| RollingHash::new(Mint::new(ctoi(ch)), Mint::new(base));

        let rev_i = |i: usize| str_len - 1 - i;

        // 文字列の正順と逆順
        let normal_rh = base_list.map(|base| {
            str.iter()
                .copied()
                .map(|ch| make_rh(ch, base))
                .collect_vec()
        });
        let reverse_rh = base_list.map(|base| {
            str.iter()
                .copied()
                .rev()
                .map(|ch| make_rh(ch, base))
                .collect_vec()
        });
        let mut normal_seg_trees = normal_rh.map(Segtree::<RollingHashConcat<Mint>>::from);
        let mut reverse_seg_trees = reverse_rh.map(Segtree::<RollingHashConcat<Mint>>::from);

        let mut ans = vec![];
        for q in qs {
            match q {
                Query::Update { x, c } => {
                    for (seg_tree, base) in izip!(&mut normal_seg_trees, base_list) {
                        seg_tree.set(*x, make_rh(*c, base));
                    }
                    for (seg_tree, base) in izip!(&mut reverse_seg_trees, base_list) {
                        seg_tree.set(rev_i(*x), make_rh(*c, base));
                    }
                }
                Query::Find { l, r } => {
                    let is_palindrome = izip!(&normal_seg_trees, &reverse_seg_trees).all(
                        |(normal_seg_tree, reverse_seg_tree)| {
                            let rev_l = rev_i(*l);
                            let rev_r = rev_i(*r);
                            normal_seg_tree.prod(l..=r) == reverse_seg_tree.prod(rev_r..=rev_l)
                        },
                    );
                    ans.push(is_palindrome);
                }
            }
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // ロリハの直積
        use ac_library::segtree::Segtree;
        use ac_library::ModInt998244353 as Mint;
        let Problem {
            str_len,
            nq,
            str,
            qs,
        } = self;

        let ctoi = |c: u8| c - b'a';

        let base_list: [i64; 3] = [100_000_001, 321_432_543, 987_654_321];
        let make_rh =
            |ch: u8| base_list.map(|base| RollingHash::new(Mint::new(ctoi(ch)), Mint::new(base)));

        let rev_i = |i: usize| str_len - 1 - i;

        // 文字列の正順と逆順
        let normal_rh = str.iter().copied().map(|ch| make_rh(ch)).collect_vec();
        let reverse_rh = str
            .iter()
            .copied()
            .rev()
            .map(|ch| make_rh(ch))
            .collect_vec();

        let mut normal_seg_tree = Segtree::<RollingHash3Concat<Mint>>::from(normal_rh);
        let mut reverse_seg_tree = Segtree::<RollingHash3Concat<Mint>>::from(reverse_rh);

        let mut ans = vec![];
        for q in qs {
            match q {
                Query::Update { x, c } => {
                    normal_seg_tree.set(*x, make_rh(*c));
                    reverse_seg_tree.set(rev_i(*x), make_rh(*c));
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
    Problem::read().solve2().print();
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

use ac_library::segtree;
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
    use ac_library::Monoid;
    use std::{
        convert::Infallible,
        marker::PhantomData,
        ops::{Add, Mul},
    };
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RollingHash<T> {
        hash: T,
        base: T,
    }
    impl<T> RollingHash<T> {
        pub fn new(hash: T, base: T) -> Self {
            Self { hash, base }
        }
        pub fn identity() -> Self
        where
            T: From<i64>,
        {
            Self {
                hash: 0.into(),
                base: 1.into(),
            }
        }
        pub fn concat(&self, rhs: &Self) -> Self
        where
            T: Copy + Mul<Output = T> + Add<Output = T>,
        {
            Self {
                hash: self.hash * rhs.base + rhs.hash,
                base: self.base * rhs.base,
            }
        }
    }
    pub struct RollingHashConcat<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RollingHashConcat<T>
    where
        T: Copy + From<i64> + Add<Output = T> + Mul<Output = T>,
    {
        type S = RollingHash<T>;
        fn identity() -> Self::S {
            RollingHash::identity()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.concat(b)
        }
    }

    pub struct RollingHash3Concat<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RollingHash3Concat<T>
    where
        T: Copy + From<i64> + Add<Output = T> + Mul<Output = T>,
    {
        type S = [RollingHash<T>; 3];
        fn identity() -> Self::S {
            [RollingHash::identity(); 3]
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            [0, 1, 2].map(|i| a[i].concat(&b[i]))
        }
    }
}
