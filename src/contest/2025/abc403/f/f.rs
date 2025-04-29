#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Expr {
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Num(i64),
}

impl Expr {
    fn to_chars(&self, memo2: &mut HashMap<Expr, Vec<char>>) -> Vec<char> {
        if let Some(ans) = memo2.get(self) {
            return ans.clone();
        }
        let ans = match self {
            Expr::Add(exprs) => exprs
                .iter()
                .map(|expr| {
                    let x = expr.to_chars(memo2);
                    x.iter().copied().collect::<String>()
                })
                .join("+")
                .chars()
                .collect_vec(),
            Expr::Mul(exprs) => exprs
                .iter()
                .map(|expr| {
                    let x = match expr {
                        Expr::Add(_) => {
                            let mut chars = vec![];
                            chars.push('(');
                            chars.extend(expr.to_chars(memo2));
                            chars.push(')');
                            chars
                        }
                        Expr::Mul(_) => expr.to_chars(memo2),
                        Expr::Num(_) => expr.to_chars(memo2),
                    };
                    x.iter().copied().collect::<String>()
                })
                .join("*")
                .chars()
                .collect_vec(),
            Expr::Num(n) => format!("{}", n).chars().collect_vec(),
        };
        memo2.insert(self.clone(), ans.clone());
        ans
    }
    fn char_len(&self, memo2: &mut HashMap<Expr, Vec<char>>) -> usize {
        self.to_chars(memo2).len()
    }
}
fn solve(n: i64, memo1: &mut HashMap<i64, Expr>, memo2: &mut HashMap<Expr, Vec<char>>) -> Expr {
    if let Some(ans) = memo1.get(&n) {
        return ans.clone();
    }
    if [1, 11, 111, 1111].contains(&n) {
        return Expr::Num(n);
    }

    let mut cands = vec![];

    for i in 1..n {
        let expr1 = solve(i, memo1, memo2);
        let expr2 = solve(n - i, memo1, memo2);
        let next_expr = match (&expr1, &expr2) {
            (Expr::Add(expr1s), Expr::Add(expr2s)) => {
                let x = chain!(expr1s, expr2s).cloned().collect_vec();
                Expr::Add(x)
            }
            _ => Expr::Add(vec![expr1, expr2]),
        };
        cands.push(next_expr);
    }

    for i in 2..n {
        if n % i != 0 {
            continue;
        }
        let expr1 = solve(i, memo1, memo2);
        let expr2 = solve(n / i, memo1, memo2);
        let next_expr = match (&expr1, &expr2) {
            (Expr::Mul(expr1s), Expr::Mul(expr2s)) => {
                //
                let x = chain!(expr1s, expr2s).cloned().collect_vec();
                Expr::Mul(x)
            }
            _ => Expr::Mul(vec![expr1, expr2]),
        };
        cands.push(next_expr);
    }
    let ans = cands
        .iter()
        .min_by_key(|expr| expr.char_len(memo2))
        .unwrap()
        .clone();
    memo1.insert(n, ans.clone());
    ans
}
fn main() {
    input! {
        n: i64,
    }
    let mut memo1 = HashMap::new();
    let mut memo2 = HashMap::new();
    let ans: Expr = solve(n, &mut memo1, &mut memo2);
    let ans = ans.to_chars(&mut memo2);
    print_chars(&ans);
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
use print_vec::*;
pub mod print_vec {

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
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
