define_queries! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Query: usize {
        1 => Change { p: Usize1, x: i64 },
        2 => Output { l: Usize1, r: Usize1 },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ValueCount {
    pub value: i64,
    pub count: i64,
}

impl ValueCount {
    fn new(value: i64, count: i64) -> ValueCount {
        ValueCount { value, count }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SecondLargestCount {
    first: Option<ValueCount>,
    second: Option<ValueCount>,
}

impl SecondLargestCount {
    // fn empty() -> SecondLargestCount {}
    fn unit(x: i64) -> SecondLargestCount {
        SecondLargestCount {
            first: Some(ValueCount { value: x, count: 1 }),
            second: None,
        }
    }

    fn second_count(&self) -> i64 {
        self.second.map(|x| x.count).unwrap_or(0)
    }

    fn inserted3(mut self, value: i64, count: i64) -> SecondLargestCount {
        if let Some(first) = &mut self.first {
            if value > first.value {
                self.second = self.first;
                self.first = Some(ValueCount::new(value, count));
            } else if value == first.value {
                first.count += count;
            } else {
                if let Some(second) = &mut self.second {
                    if value > second.value {
                        self.second = Some(ValueCount::new(value, count));
                    } else if value == second.value {
                        second.count += count;
                    }
                } else {
                    self.second = Some(ValueCount::new(value, count));
                }
            }
        } else {
            self.first = Some(ValueCount::new(value, count));
        }
        self
    }

    fn inserted2(self, value: i64, count: i64) -> SecondLargestCount {
        // 挿入ソート風 (正しく実装するのが難しい)
        let mut arr = [self.first, self.second, Some(ValueCount::new(value, count))];

        for i in [1, 0] {
            // None は最小元扱い
            if arr[i].map(|x| x.value) < arr[i + 1].map(|x| x.value) {
                arr.swap(i, i + 1);
            } else if let (Some(x), Some(y)) = (arr[i], arr[i + 1])
                && x.value == y.value
            {
                arr[i].as_mut().unwrap().count += y.count;

                // i + 1 を消す処理が微妙。
                // 長さが2だからこれでいいが、もっと長い場合は i+2 を i+1 に持っていくだけでなく、i+3, i+4 ... を i+2, i+3,... に持っていく処理が必要
                arr[i + 1] = arr.get(i + 2).copied().flatten();
            }
        }
        SecondLargestCount {
            first: arr[0],
            second: arr[1],
        }
    }

    fn inserted(self, value: i64, count: i64) -> SecondLargestCount {
        // 2 pointer
        // 参考: https://atcoder.jp/contests/abc343/submissions/50794108
        let mut iter1 = [self.first, self.second].into_iter().peekable();
        let mut iter2 = std::iter::once(Some(ValueCount::new(value, count))).peekable();

        let mut arr: [Option<ValueCount>; 2] = [None, None];

        for i in 0..2 {
            let x1 = iter1.peek().copied().flatten();
            let x2 = iter2.peek().copied().flatten();
            let x1_val = x1.map(|x| x.value);
            let x2_val = x2.map(|x| x.value);
            // None は最小元扱い
            if x1_val < x2_val {
                arr[i] = x2;
                iter2.next();
            } else if x1_val > x2_val {
                arr[i] = x1;
                iter1.next();
            } else if let (Some(x1), Some(x2)) = (x1, x2)
                && x1.value == x2.value
            {
                arr[i] = Some(ValueCount::new(x1.value, x1.count + x2.count));
                iter1.next();
                iter2.next();
            }
        }
        SecondLargestCount {
            first: arr[0],
            second: arr[1],
        }
    }
}

struct Concat(Infallible);
impl Monoid for Concat {
    type S = SecondLargestCount;
    fn identity() -> Self::S {
        SecondLargestCount {
            first: None,
            second: None,
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let mut tmp = a.clone();
        if let Some(b_first) = b.first {
            tmp = tmp.inserted(b_first.value, b_first.count);
        }
        if let Some(b_second) = b.second {
            tmp = tmp.inserted(b_second.value, b_second.count);
        }

        tmp
    }
}

struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<i64>,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: [i64; n],
            qs: [Query; nq]
        }
        Problem { n, nq, xs, qs }
    }
    fn solve(&self) -> Answer {
        let mut xs_segtree = Segtree::<Concat>::from(
            self.xs
                .iter()
                .copied()
                .map(SecondLargestCount::unit)
                .collect_vec(),
        );

        let mut ans = vec![];
        for &q in &self.qs {
            match q {
                Query::Change { p, x } => {
                    let e = SecondLargestCount::unit(x);
                    xs_segtree.set(p, e);
                }
                Query::Output { l, r } => {
                    let e = xs_segtree.prod(l..=r);
                    ans.push(e.second_count())
                }
            }
        }
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
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

use std::convert::Infallible;

use ac_library::{Monoid, Segtree};
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
#[macro_use]
pub mod define_queries {
    /// クエリ形式の入力を proconio::input! で読み込める enum を定義するマクロ。
    /// 出典： <https://zenn.dev/magurofly/articles/6ee845bd5e385e>
    /// # 利用例
    /// ```
    /// use mylib::define_queries;
    /// use proconio::marker::Usize1;
    /// define_queries! {
    ///     #[derive(Debug, PartialEq)]
    ///     enum Query: usize {
    ///         1 => Add { a: i64, b: i64 },
    ///         2 => Show { k: Usize1 },
    ///     }
    /// }
    /// ```
    #[macro_export]
    macro_rules ! define_queries {($ ($ (# [$ attr : meta ] ) * enum $ enum_name : ident : $ sig : ty {$ ($ pattern : pat => $ variant : ident $ ({$ ($ name : ident : $ marker : ty $ (, ) ? ) ,* } ) ? $ (, ) ? ) ,* } ) * ) => {$ ($ (# [$ attr ] ) * enum $ enum_name {$ ($ variant $ ({$ ($ name : <$ marker as proconio :: source :: Readable >:: Output ) ,* } ) ? ) ,* } impl proconio :: source :: Readable for $ enum_name {type Output = Self ; fn read < R : std :: io :: BufRead , S : proconio :: source :: Source < R >> (source : & mut S ) -> Self {#! [allow (unreachable_patterns ) ] match <$ sig as proconio :: source :: Readable >:: read (source ) {$ ($ pattern => $ enum_name ::$ variant $ ({$ ($ name : <$ marker as proconio :: source :: Readable >:: read (source ) ) ,* } ) ? ) ,* , _ => unreachable ! () } } } ) * } }
}
