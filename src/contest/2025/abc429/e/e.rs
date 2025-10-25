fn main() {
    input! {
        nv: usize,
        ne: usize,
        es: [(Usize1, Usize1); ne],
        s: Chars,
    }

    let is_safe = s.iter().copied().map(|s| s == 'S').collect_vec();
    let adj = es
        .iter()
        .copied()
        .fold(vec![vec![]; nv], |mut acc, (u, v)| {
            acc[u].push(v);
            acc[v].push(u);
            acc
        });

    // (初期点, 現在の点)
    let mut open: Queue<(usize, usize, i64)> = Queue::new();
    // visited1[v]: 最初にvに訪問した安全点
    let mut visited1 = vec![usize::MAX; nv];
    // visited2[v]: 2番目にvに訪問した安全点
    let mut visited2 = vec![usize::MAX; nv];

    let mut dist1 = vec![i64::MAX; nv];
    let mut dist2 = vec![i64::MAX; nv];

    for v in 0..nv {
        if is_safe[v] {
            open.push((v, v, 0));

            visited1[v] = v;
            dist1[v] = 0;
        }
    }

    while let Some((init, current, d)) = open.pop() {
        for &next in &adj[current] {
            if visited1[next] != usize::MAX && visited2[next] != usize::MAX {
                continue;
            }

            if visited1[next] == usize::MAX {
                visited1[next] = init;
                dist1[next] = d + 1;
            } else {
                assert!(visited2[next] == usize::MAX);
                if visited1[next] == init {
                    continue;
                }
                visited2[next] = init;
                dist2[next] = d + 1;
            }
            open.push((init, next, d + 1));
        }
    }

    // for v in 0..nv {
    //     assert!(visited1[v] != visited2[v]);
    // }

    // dbg!(&visited1);
    // dbg!(&visited2);
    // dbg!(&dist1);
    // dbg!(&dist2);

    let ans: Vec<i64> = (0..nv)
        .filter(|v| !is_safe[*v])
        .map(|v| dist1[v] + dist2[v])
        .collect_vec();
    print_vec(&ans);
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
        let n = rng.gen_range(1..=10);
        let xs = (0..n).map(|_| rng.gen_range(0..10)).collect_vec();

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
        // let mut rng = SmallRng::from_entropy();
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
    itertools::{chain, iproduct, izip, Itertools},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng},
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
use {mod_queue::*, std::usize};
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
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
