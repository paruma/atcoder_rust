#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: Usize1,
    to: Usize1,
    w: u64,
}

const W: usize = 1024;

fn solve1(nv: usize, ne: usize, es: &[Edge]) -> Option<u64> {
    // SCC を使う
    // min{ w 上の辺重み xor sum | w: 頂点 0 から頂点 n-1 までの walk} を値域の空間で全探索する
    // f(v, x) = 頂点 0 から頂点 v までの walk で xor sum が x のものが存在する
    // とすると、f(v, x) = any i: B[i] = v s.t. f(A[i], x xor W[i])
    // という再帰関数が成り立つ。
    // このままだと f の呼び出しグラフにサイクルがある場合無限ループしてしまう。
    // SCC でサイクルを潰せばよい。

    // 頂点番号 * 1024 + xor の値 を新しいグラフの頂点番号にする

    fn id(v: usize, x: u64) -> usize {
        v * W + (x as usize)
    }

    let snv = nv * W;
    let ses = (0..(W as u64))
        .flat_map(|x| {
            es.iter().map(move |e| {
                let from_v = e.from;
                let from_x = x;
                let to_v = e.to;
                let to_x = x ^ e.w;
                (id(from_v, from_x), id(to_v, to_x))
            })
        })
        .collect_vec();

    let mut scc_graph = SccGraph::new(snv);

    for &(from, to) in &ses {
        scc_graph.add_edge(from, to);
    }

    let scc = scc_graph.scc();
    let vid_to_scc_id = {
        let mut vid_to_scc_id = vec![usize::MAX; nv * W];
        for (gid, g) in scc.iter().enumerate() {
            for &vid in g {
                vid_to_scc_id[vid] = gid;
            }
        }
        vid_to_scc_id
    };

    let scc_rev_adj = {
        let mut scc_rev_adj = vec![HashSet::<usize>::new(); scc.len()];
        for &(from, to) in &ses {
            let from_scc_idx = vid_to_scc_id[from];
            let to_scc_idx = vid_to_scc_id[to];
            if from_scc_idx != to_scc_idx {
                scc_rev_adj[to_scc_idx].insert(from_scc_idx);
            }
        }
        scc_rev_adj
            .iter()
            .map(|s| s.iter().copied().collect_vec())
            .collect_vec()
    };

    let mut dp = vec![None; scc.len()];
    dp[vid_to_scc_id[id(0, 0)]] = Some(true);

    fn f(sv: usize, dp: &mut [Option<bool>], rev_adj: &[Vec<usize>]) -> bool {
        if let Some(ans) = dp[sv] {
            return ans;
        }

        let ans = rev_adj[sv].iter().copied().any(|from| f(from, dp, rev_adj));

        dp[sv] = Some(ans);
        ans
    }

    // dbg!(scc);

    // dbg!(&dp);

    (0..(W as u64)).find(|x| f(vid_to_scc_id[id(nv - 1, *x)], &mut dp, &scc_rev_adj))
}
fn main() {
    input! {
        nv: usize,
        ne: usize,
        es: [Edge; ne],
    }

    let ans: Option<u64> = solve1(nv, ne, &es);
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
}

use ac_library::SccGraph;
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
