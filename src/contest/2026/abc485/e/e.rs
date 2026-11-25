// #[fastout]
fn main() {
    input! {
        nv: usize,
        ne: usize,
        es: [(Usize1, Usize1, i64); ne],
    }

    let adj = es
        .iter()
        .copied()
        .fold(vec![vec![]; nv], |mut acc, (from, to, c)| {
            acc[from].push((to, c));
            acc
        });

    let mut open: Queue<(usize, i64, i64)> = Queue::new();
    // dist1[v]: v までの最短路とその直前の辺の色
    let mut dist1: Vec<Option<(i64, i64)>> = vec![None; nv];
    // dist2[v]: v までの 2nd 最短路とその直前の辺の色 (同じ色は1回まで)
    let mut dist2: Vec<Option<(i64, i64)>> = vec![None; nv];

    open.push((0, 0, -1));

    while let Some((cur_v, cur_d, prev_color)) = open.pop() {
        // すでにその頂点に2色から到着済 or 今の色が到着済
        if dist2[cur_v].is_some() {
            continue;
        }
        if let Some((_recorded_d, recorded_c)) = dist1[cur_v] {
            if recorded_c != prev_color {
                dist2[cur_v] = Some((cur_d, prev_color));
            } else {
                continue;
            }
        } else {
            dist1[cur_v] = Some((cur_d, prev_color));
        }
        for &(next_v, next_color) in &adj[cur_v] {
            if next_color != prev_color {
                open.push((next_v, cur_d + 1, next_color));
            }
        }
    }
    let ans = dist1[nv - 1].map(|(d, _)| d);
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
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
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
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
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
// 使用したテンプレート・スニペットの公開元:
// https://github.com/paruma/atcoder_rust
use {maplit::hashmap, mod_queue::*};
pub mod mod_queue {
    use std::collections::VecDeque;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_back(value)
        }

        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_front()
        }

        pub fn peek(&self) -> Option<&T> {
            self.raw.front()
        }

        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }

        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }

    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
