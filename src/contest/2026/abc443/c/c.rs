fn solve(n: usize, t: i64, xs: &[i64]) -> i64 {
    let mut close_time = 0;
    let mut closed = false;
    let mut closed_sum = 0; // 閉じている時間の和

    for &x in xs {
        if closed {
            if (close_time..close_time + 100).contains(&x) {
                //
            } else {
                closed_sum += 100;
                close_time = x;
            }
        } else {
            closed = true;
            close_time = x;
        }
    }

    if (close_time..close_time + 100).contains(&t) {
        if closed {
            closed_sum += t - close_time;
        }
        //
    } else if closed {
        closed_sum += 100;
    }
    t - closed_sum
}

fn solve_naive(n: usize, t: i64, xs: &[i64]) -> i64 {
    let mut sum = 0;
    let mut open = true;
    let mut close_time = 0;

    for t in 0..t {
        if open && xs.contains(&t) {
            close_time = t;
            open = false;
        }

        if t - close_time == 100 {
            open = true;
        }
        if open {
            sum += 1;
        }
    }
    sum
}
// #[fastout]
fn main() {
    input! {
        n: usize,
        t: i64,
        xs: [i64; n],
    }
    let ans = solve(n, t, &xs);

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

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(0..=10);

        let xs: Vec<i64> = generate_random_while(
            || {
                //
                let mut xs = generate_random_uniq_sequence(n, || rng.random_range(2..1000));
                xs.sort();
                xs
            },
            |xs| {
                xs.iter()
                    .copied()
                    .tuple_combinations()
                    .all(|(x, y)| i64::abs(x - y) != 100)
            },
        );
        let t = xs.iter().copied().max().unwrap_or(0) + rng.random_range(0..2);

        // ==== 解く ====
        let main_ans = solve(n, t, &xs);
        let naive_ans = solve_naive(n, t, &xs);

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, t, xs));
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
        let num_tests = 10000;
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
use random_test::*;
/// ランダムなテストケースを生成するためのユーティリティモジュール
pub mod random_test {
    use itertools::Itertools;
    use num::Integer;
    use num_integer::Roots;
    use petgraph::unionfind::UnionFind;
    use rand::Rng;
    use std::{collections::HashSet, hash::Hash};
    /// 指定された個数のユニークな値を生成する。
    /// `gen` クロージャが返す値が `n` 種類に達するまで値の生成を繰り返す。
    /// # Arguments
    /// * `n` - 生成するユニークな値の個数
    /// * `gen` - 値を生成するクロージャ
    /// # Examples
    /// ```
    /// use mylib::utils::random::random_test::*;
    /// use rand::{Rng, rngs::SmallRng, SeedableRng};
    /// let mut rng = SmallRng::from_os_rng();
    /// let uniq_seq = generate_random_uniq_sequence(10, || rng.random_range(0..100));
    /// assert_eq!(uniq_seq.len(), 10);
    /// ```
    pub fn generate_random_uniq_sequence<T, F>(n: usize, mut r#gen: F) -> Vec<T>
    where
        T: Hash + PartialEq + Eq,
        F: FnMut() -> T,
    {
        let mut set: HashSet<T> = HashSet::new();
        while set.len() != n {
            set.insert(r#gen());
        }
        set.into_iter().collect_vec()
    }
    /// 条件 `pred` を満たすランダムな値を生成する。
    /// `gen` クロージャで値を生成し、`pred` が `true` を返すまで繰り返す。
    /// # Arguments
    /// * `gen` - 値を生成するクロージャ
    /// * `pred` - 値が満たすべき条件を判定するクロージャ
    /// # Examples
    /// ```
    /// use mylib::utils::random::random_test::*;
    /// use rand::{Rng, rngs::SmallRng, SeedableRng};
    /// let mut rng = SmallRng::from_os_rng();
    /// let even_number = generate_random_while(|| rng.random_range(0..100), |&x| x % 2 == 0);
    /// assert!(even_number % 2 == 0);
    /// ```
    pub fn generate_random_while<T, F, P>(mut r#gen: F, mut pred: P) -> T
    where
        F: FnMut() -> T,
        P: FnMut(&T) -> bool,
    {
        loop {
            let x = r#gen();
            if pred(&x) {
                return x;
            }
        }
    }
    /// `n_vertices` 頂点のランダムな木（辺のリスト）を生成する。
    /// # Arguments
    /// * `rng` - 乱数生成器
    /// * `n_vertices` - 木の頂点数
    /// # Examples
    /// ```
    /// use mylib::utils::random::random_test::*;
    /// use rand::{Rng, rngs::SmallRng, SeedableRng};
    /// let mut rng = SmallRng::from_os_rng();
    /// let tree = generate_random_tree(&mut rng, 5);
    /// assert_eq!(tree.len(), 4);
    /// ```
    pub fn generate_random_tree<R>(rng: &mut R, n_vertices: usize) -> Vec<(usize, usize)>
    where
        R: Rng,
    {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut uf: UnionFind<usize> = UnionFind::new(n_vertices);
        while edges.len() != n_vertices - 1 {
            let x = rng.random_range(0..n_vertices);
            let y = rng.random_range(0..n_vertices);
            if uf.union(x, y) {
                edges.push((x, y));
            }
        }
        edges
    }
    fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=n.sqrt() {
            if n.is_multiple_of(&i) {
                return false;
            }
        }
        true
    }
    /// 指定された範囲 `[begin, end)` 内のランダムな素数を生成する。
    /// # Arguments
    /// * `rng` - 乱数生成器
    /// * `begin` - 範囲の下限（含む）
    /// * `end` - 範囲の上限（含まない）
    /// # Examples
    /// ```
    /// use mylib::utils::random::random_test::*;
    /// use rand::{Rng, rngs::SmallRng, SeedableRng};
    /// let mut rng = SmallRng::from_os_rng();
    /// let prime = generate_random_prime(&mut rng, 0, 100);
    /// ```
    pub fn generate_random_prime<R>(rng: &mut R, begin: i64, end: i64) -> i64
    where
        R: Rng,
    {
        let r#gen = || rng.random_range(begin..end);
        generate_random_while(r#gen, |x| is_prime(*x))
    }
}
