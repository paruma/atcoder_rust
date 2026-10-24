// 問題文と制約は読みましたか？
// #[fastout]

fn main() {
    input! {
        nv: usize,
        ne: usize,
        es: [(Usize1, Usize1, i64); ne],
    }

    let adj1 = make_adj_from_directed(
        nv,
        &es.iter().copied().map(|(u, v, _)| (u, v)).collect_vec(),
    );
    let mut adj2 = make_adj_from_weighted_directed(nv, &es);

    for nexts in &mut adj2 {
        nexts.sort_by_key(|(_, c)| *c);
    }

    let mut trie: Vec<BTreeMap<i64, usize>> = vec![BTreeMap::new()];
    let mut buf: Vec<usize> = vec![0];
    let mut visited = vec![false; nv];
    let mut trie_ids = vec![usize::MAX; nv];
    visited[0] = true;
    trie_ids[0] = 0;
    use ac_library::ModInt998244353 as Mint;
    let mut ans = vec![Mint::new(0); nv];

    while !buf.is_empty() {
        let mut buf2: Vec<usize> = vec![];
        // buf 辞書順でソート

        // trie_id → buf に入ってる値
        let mut trie_id_to_buf: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for &i in &buf {
            trie_id_to_buf.entry(trie_ids[i]).or_default().push(i);
        }

        for (_trie_id, group) in trie_id_to_buf {
            let mut nexts = vec![];
            for cur in group {
                for &(next, cost) in &adj2[cur] {
                    nexts.push((cur, next, cost));
                }
            }
            nexts.sort_by_key(|(_, _, cost)| *cost);

            for &(cur, next, cost) in &nexts {
                let cur_trie_id = trie_ids[cur];

                if !visited[next] {
                    visited[next] = true;
                    buf2.push(next);
                    ans[next] = ans[cur] * 10 + Mint::new(cost);

                    if !trie[cur_trie_id].contains_key(&cost) {
                        let new_node = trie.len();
                        trie[cur_trie_id].insert(cost, new_node);
                        trie.push(BTreeMap::new());
                    }

                    let next_trie_id = trie[cur_trie_id][&cost];
                    trie_ids[next] = next_trie_id;
                }
            }
        }

        // let trie_id_min = buf.iter().copied().map(|i| trie_ids[i]).min().unwrap();

        // for &cur in &buf {
        //     let cur_trie_id = trie_ids[cur];
        //     for &(next, cost) in &adj2[cur] {
        //         if !visited[next] {
        //             visited[next] = true;
        //             buf2.push(next);
        //             ans[next] = ans[cur] * 10 + Mint::new(cost);

        //             if !trie[cur_trie_id].contains_key(&cost) {
        //                 let new_node = trie.len();
        //                 trie[cur_trie_id].insert(cost, new_node);
        //                 trie.push(BTreeMap::new());
        //             }

        //             let next_trie_id = trie[cur_trie_id][&cost];
        //             trie_ids[next] = next_trie_id;
        //         }
        //     }
        // }
        buf = buf2;
    }

    print_vec(&ans[1..]);
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
/// 有向グラフの辺集合から隣接リストを作成します。
///
/// # 計算量
/// O(V + E)
pub fn make_adj_from_directed(n_vertex: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n_vertex];

    for &(from, to) in edges {
        adj[from].push(to);
    }

    adj
}

/// 重み付き有向グラフの辺集合から隣接リストを作成します。
///
/// # 計算量
/// O(V + E)
pub fn make_adj_from_weighted_directed<T>(
    n_vertex: usize,
    edges: &[(usize, usize, T)],
) -> Vec<Vec<(usize, T)>>
where
    T: Clone,
{
    let mut adj = vec![vec![]; n_vertex];

    for (from, to, weight) in edges {
        adj[*from].push((*to, weight.clone()));
    }

    adj
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
