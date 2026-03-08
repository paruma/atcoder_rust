/// `FnMut` クロージャーを再帰的に呼び出すためのユーティリティ。
/// # 使い方
/// クロージャーの第1引数を `self_` として受け取り、それを呼び出すことで再帰します。
/// 任意の型を引数として使用でき、戻り値にも対応しています。
/// タプルを使うことで複数引数の再帰も実現できます。
/// **注意**: クロージャーはクロージャーで使った外部変数の可変借用を保持するため、
/// クロージャーを使い終わった後に `drop(closure)` で明示的にドロップしてから、
/// その外部変数に再度アクセスする必要があります。
/// # 例
/// ## 1引数再帰（階乗）
/// ```ignore
/// let mut fact = unsafe { recurse_mut(|f, n: u64| {
///     if n == 0 { 1 } else { n * f(n - 1) }
/// }) };
/// assert_eq!(fact(5), 120);
/// ```
/// ## 戻り値なし（副作用ベースの再帰）
/// ```ignore
/// let mut visited = vec![false; 4];
/// let mut dfs = unsafe {
///     recurse_mut(|dfs, v: usize| {
///         visited[v] = true;
///         // 子頂点への再帰呼び出し
///         for &child in &children[v] {
///             if !visited[child] {
///                 dfs(child);
///             }
///         }
///     })
/// };
/// dfs(0);
/// drop(dfs);  // クロージャーで使った visited に再度アクセスするため、クロージャーを明示的にドロップ
/// println!("{:?}", visited);  // これで visited にアクセス可能
/// ```
/// # Safety
/// 再帰呼び出し中にキャプチャ変数を drop してはいけない。
/// 具体的には以下の操作が危険：
/// - 参照（`&T`）を保持したまま再帰呼び出しをして、再帰後にその参照を使う
/// - `Option::take()`, `drop()` など、キャプチャ変数内の値を無効化する操作を再帰中に行う
pub unsafe fn recurse_mut<Arg, R, F>(mut f: F) -> impl FnMut(Arg) -> R
where
    F: FnMut(&mut dyn FnMut(Arg) -> R, Arg) -> R,
{
    fn call_recursive<Arg, R, F: FnMut(&mut dyn FnMut(Arg) -> R, Arg) -> R>(
        f: *mut F,
        arg: Arg,
    ) -> R {
        let f_ptr = f;
        let mut self_: Box<dyn FnMut(Arg) -> R> = Box::new(move |a| call_recursive(f_ptr, a));
        unsafe { (*f)(&mut *self_, arg) }
    }
    move |arg| call_recursive(&mut f, arg)
}
#[fastout]
fn main() {
    input! {
        n: usize,
        xs: [i64; n],
        es: [(Usize1, Usize1); n-1],
    }

    let adj = es.iter().copied().fold(vec![vec![]; n], |mut acc, (u, v)| {
        acc[u].push(v);
        acc[v].push(u);
        acc
    });

    let children = make_tree_children(&adj, 0);
    let mut bag: HashBag<i64> = HashBag::new();
    let mut ans = vec![false; n];

    let mut dfs = unsafe {
        recurse_mut(|dfs, cur: usize| {
            bag.insert(xs[cur]);
            ans[cur] = bag.len() != bag.set_len();
            for &c in &children[cur] {
                dfs(c);
            }
            bag.remove(&xs[cur]);
        })
    };

    dfs(0);
    drop(dfs);

    for p in ans {
        println!("{}", if p { "Yes" } else { "No" });
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

use hashbag::HashBag;
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
