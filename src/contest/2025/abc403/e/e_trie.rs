// trie による解法
// 文字列の多重集合 X, Y を trie で管理する。
// #{y ∈ Y | y は どの X の要素も接頭辞として持たない} をクエリごとに差分更新していく
// X に x を追加するクエリでは、x を接頭辞として持つ Y の要素の個数（今までに他の X の要素を接頭辞として持っていたものは除く）を数えて差分更新する。
// Y に y を追加するクエリでは、y が X のある要素の接頭辞として持っている場合は Y への追加をスキップする
#[derive_readable]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Query {
    t: usize,
    s: Chars,
}
fn main() {
    input! {
        nq: usize,
        qs: [Query; nq],
    }
    let mut trie_x: TrieCore<char> = TrieCore::new();
    let mut trie_x_contains_count = vec![0];
    let mut trie_y: TrieCore<char> = TrieCore::new();
    let mut trie_y_prefix_count = vec![0];

    let mut cnt = 0; // #{y ∈ Y | y は どの X の要素も接頭辞として持たない}
    let mut ans: Vec<i64> = vec![];
    for q in qs {
        if q.t == 1 {
            // X に追加
            let contains = trie_x
                .find_node(&q.s)
                .is_some_and(|last_node| trie_x_contains_count[last_node] >= 1);
            if !contains {
                trie_x.insert(&q.s);
                trie_x_contains_count.resize(trie_x.num_nodes(), 0);
                if let Some(last_node) = trie_x.find_node(&q.s) {
                    trie_x_contains_count[last_node] += 1;
                }
                if let Some(last_node) = trie_y.find_node(&q.s) {
                    cnt -= trie_y_prefix_count[last_node];
                }
            }
        } else {
            // Y に追加
            let has_x_elem_as_prefix = trie_x
                .trace_nodes(&q.s)
                .iter()
                .any(|&node| trie_x_contains_count[node] >= 1);
            if !has_x_elem_as_prefix {
                trie_y.insert(&q.s);
                trie_y_prefix_count.resize(trie_y.num_nodes(), 0);
                for node in trie_y.trace_nodes(&q.s) {
                    trie_y_prefix_count[node] += 1;
                }

                cnt += 1;
            }
        }
        ans.push(cnt);
    }

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
use trie::*;
#[allow(clippy::module_inception)]
/// トライ木（接頭辞木）の実装。
/// 数列（文字列）の集合を管理し、共通接頭辞の検索やノードのパス取得を効率的に行う。
pub mod trie {
    use std::collections::BTreeMap;
    /// トライ木の実装。
    /// 各ノードは `BTreeMap` を用いて、次の文字に対する遷移先ノード ID を保持する。
    #[derive(Clone, Debug)]
    pub struct TrieCore<T> {
        children_list: Vec<BTreeMap<T, usize>>,
    }
    impl<T: Ord + Copy> TrieCore<T> {
        /// 空のトライ木を構築する。
        /// # 計算量
        /// O(1)
        pub fn new() -> Self {
            Self {
                children_list: vec![BTreeMap::new()],
            }
        }
        /// 指定したノード `node` が持つ子ノードへの遷移情報を取得する。
        /// # 計算量
        /// O(1)
        pub fn children(&self, node: usize) -> &BTreeMap<T, usize> {
            &self.children_list[node]
        }
        /// 数列 `xs` をトライ木に挿入する。
        /// # 計算量
        /// O(|xs| log Σ) （Σ はアルファベットサイズ、ここでは文字の種類数）
        pub fn insert(&mut self, xs: &[T]) {
            let mut cur_node = 0;
            for &x in xs {
                if !self.children_list[cur_node].contains_key(&x) {
                    let new_node = self.children_list.len();
                    self.children_list[cur_node].insert(x, new_node);
                    self.children_list.push(BTreeMap::new());
                }
                cur_node = self.children_list[cur_node][&x];
            }
        }
        /// 指定したノード `cur` から、文字 `x` による遷移先のノード ID を取得する。
        /// 遷移先が存在しない場合は `None` を返す。
        /// # 計算量
        /// O(log Σ)
        pub fn next(&self, cur: usize, x: T) -> Option<usize> {
            self.children_list[cur].get(&x).copied()
        }
        /// 数列 `xs` に対応する終端ノード ID を取得する。
        /// `xs` がトライ木に含まれない場合は `None` を返す。
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn find_node(&self, xs: &[T]) -> Option<usize> {
            let mut cur_node = 0;
            for &x in xs {
                let next_node = self.next(cur_node, x)?;
                cur_node = next_node;
            }
            Some(cur_node)
        }
        /// 数列 `xs` を辿る際に通過するノード ID のリストを返す。
        /// 途中で遷移できなくなった場合は、そこまでのノード ID リストを返す。
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn trace_nodes(&self, xs: &[T]) -> Vec<usize> {
            let mut cur_node = 0;
            let mut path = vec![cur_node];
            for &x in xs {
                let Some(next_node) = self.next(cur_node, x) else {
                    break;
                };
                cur_node = next_node;
                path.push(cur_node);
            }
            path
        }
        /// トライ木に含まれるノードの総数を返す。
        /// # 計算量
        /// O(1)
        pub fn num_nodes(&self) -> usize {
            self.children_list.len()
        }
    }
    impl<T: Ord + Copy> Default for TrieCore<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
