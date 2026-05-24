fn tree_cumsum(adj: &[Vec<usize>], xs: &[i64], root: usize) -> Vec<i64> {
    let children = make_tree_children(adj, root);
    let order = dfs_post_order(adj, root);

    let nv = adj.len();

    let mut dp = vec![0; nv];
    for cur in order {
        let children_sum = children[cur]
            .iter()
            .copied()
            .map(|child| dp[child])
            .sum::<i64>();
        dp[cur] = children_sum + xs[cur];
    }
    dp
}

use ac_library::ModInt998244353 as Mint;

fn nck(n: i64, k: i64) -> Mint {
    if n < k {
        Mint::new(0)
    } else {
        // 分子
        let f1 = (n - k + 1..=n).map(Mint::new).product::<Mint>();
        // 分母
        let f2 = (1..=k).map(Mint::new).product::<Mint>();

        f1 / f2
    }
}

// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut ps: [Usize1; n - 1],
        cs: [i64; n],
        ds: [i64; n],
    }
    ps.insert(0, 0);

    let adj = ps
        .iter()
        .copied()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, x)| {
            acc[i].push(x);
            acc[x].push(i);
            acc
        });

    // 葉から累積和
    let cum_cs = tree_cumsum(&adj, &cs, 0);
    let cum_ds = tree_cumsum(&adj, &ds, 0);

    // dbg!(&cs);
    // dbg!(&ds);
    // dbg!(&cum_cs);
    // dbg!(&cum_ds);

    let ans = (0..n)
        .map(|i| {
            let sum_cs = cum_cs[i];
            let sum_ds = cum_ds[i];

            let all = sum_cs - sum_ds + ds[i];
            let n_take = ds[i];
            nck(all, n_take)
        })
        .product::<Mint>();

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
/// 深さ優先探索 (DFS) を行い、帰りがけ順 (post-order) での頂点順序を返します。
/// # 計算量
/// O(V + E)
pub fn dfs_post_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    fn dfs(
        adj: &[Vec<usize>],
        current: usize,
        visited: &mut Vec<bool>,
        post_order: &mut Vec<usize>,
    ) {
        visited[current] = true;
        for &next in &adj[current] {
            if !visited[next] {
                dfs(adj, next, visited, post_order);
            }
        }
        post_order.push(current);
    }
    let nv = adj.len();
    let mut visited = vec![false; nv];
    let mut post_order = vec![];
    dfs(adj, init, &mut visited, &mut post_order);
    post_order
}
