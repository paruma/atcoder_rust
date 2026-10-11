// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        nv: usize,
        es: [(Usize1, Usize1); nv - 1],
    }

    let adj = es
        .iter()
        .copied()
        .fold(vec![vec![]; nv], |mut acc, (u, v)| {
            acc[u].push(v);
            acc[v].push(u);
            acc
        });

    let mut cost_map = HashMap::new();

    for (i, (u, v)) in es.iter().copied().enumerate() {
        cost_map.insert((u, v), i + 1);
        cost_map.insert((v, u), i + 1);
    }

    let centroid = {
        // 頂点0を根とした根付き木を考える。
        // sub_weights[v] は vを根とする部分木の重みの総和とする
        let sub_weights = {
            let mut dp = vec![-1; nv];
            let mut visited = vec![false; nv];

            fn rec(v: usize, adj: &[Vec<usize>], visited: &mut [bool], dp: &mut [i64]) -> i64 {
                let mut sum = 0;
                for &next in &adj[v] {
                    if !visited[next] {
                        visited[next] = true;
                        sum += rec(next, adj, visited, dp);
                    }
                }
                sum += 1;
                dp[v] = sum;
                sum
            }

            visited[0] = true; //行きがけ
            rec(0, &adj, &mut visited, &mut dp);

            dp
        };
        let weight_sum = sub_weights[0];
        sub_weights
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_i, w)| w * 2 >= weight_sum)
            .min_by_key(|&(_, w)| w)
            .unwrap()
            .0
    };

    let all_sum = (nv * (nv - 1)) / 2;
    let children = make_tree_children(&adj, centroid);
    let bfs_ord = bfs_order(&adj, centroid);

    let mut dp = vec![0; nv];

    for cur in bfs_ord {
        for &next in &children[cur] {
            dp[next] = dp[cur] + cost_map[&(cur, next)];
        }
    }

    let ans = dp.iter().copied().map(|w| all_sum - w).collect_vec();
    print_vec_1line(&ans);
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
/// 根付き木の隣接リスト `adj` と根 `root` から、各頂点の子頂点リストを求めます。
///
/// # 計算量
/// O(V + E)
pub fn make_tree_children(adj: &[Vec<usize>], root: usize) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut children = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[root] = true;
    queue.push_back(root);
    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                children[v].push(u);
                queue.push_back(u);
            }
        }
    }
    children
}
use mod_queue::*;
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
/// 幅優先探索 (BFS) を行い、頂点の訪問順序を返します。
///
/// # 計算量
/// O(V + E)
pub fn bfs_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    let nv = adj.len();
    let mut order = vec![];
    let mut visited = vec![false; nv];
    let mut open = Queue::new();
    open.push(init);
    order.push(init);
    visited[init] = true;
    while let Some(current) = open.pop() {
        for &next in &adj[current] {
            if !visited[next] {
                order.push(next);
                visited[next] = true;
                open.push(next);
            }
        }
    }
    order
}
