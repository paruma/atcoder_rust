use cargo_snippet::snippet;

/// `FnMut` クロージャーを再帰的に呼び出すためのユーティリティ。
///
/// # 使い方
///
/// クロージャーの第1引数を `self_` として受け取り、それを呼び出すことで再帰します。
/// 任意の型を引数として使用でき、戻り値にも対応しています。
/// タプルを使うことで複数引数の再帰も実現できます。
///
/// **注意**: クロージャーはクロージャーで使った外部変数の可変借用を保持するため、
/// クロージャーを使い終わった後に `drop(closure)` で明示的にドロップしてから、
/// その外部変数に再度アクセスする必要があります。
///
/// # 例
///
/// ## 1引数再帰（階乗）
/// ```ignore
/// let mut fact = recurse_mut(|f, n: u64| {
///     if n == 0 { 1 } else { n * f(n - 1) }
/// });
/// assert_eq!(fact(5), 120);
/// ```
///
/// ## 戻り値なし（副作用ベースの再帰）
/// ```ignore
/// let mut visited = vec![false; 4];
/// let mut dfs = recurse_mut(|dfs, v: usize| {
///     visited[v] = true;
///     // 子頂点への再帰呼び出し
///     for &child in &children[v] {
///         if !visited[child] {
///             dfs(child);
///         }
///     }
/// });
/// dfs(0);
/// drop(dfs);  // クロージャーで使った visited に再度アクセスするため、クロージャーを明示的にドロップ
/// println!("{:?}", visited);  // これで visited にアクセス可能
/// ```
#[snippet]
pub fn recurse_mut<Arg, R, F>(mut f: F) -> impl FnMut(Arg) -> R
where
    F: FnMut(&mut dyn FnMut(Arg) -> R, Arg) -> R,
{
    fn call_recursive<Arg, R, F: FnMut(&mut dyn FnMut(Arg) -> R, Arg) -> R>(
        f: *mut F,
        arg: Arg,
    ) -> R {
        let f_ptr = f; // *mut F は Copy なのでポインタ値をコピー
        let mut self_: Box<dyn FnMut(Arg) -> R> = Box::new(move |a| call_recursive(f_ptr, a));
        // Safety: `self_` だけが `f_ptr` のコピーを保持する。
        // `(*f)` は 1 回の `call_recursive` 呼び出しにつき 1 回だけ呼ばれ、
        // `self_` はその呼び出しの内側からのみ使われる（シングルスレッドの逐次呼び出し）。
        // よって `*f` への同時可変アクセスは発生しない。
        unsafe { (*f)(&mut *self_, arg) }
    }
    move |arg| call_recursive(&mut f, arg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        let mut fact = recurse_mut(|f, n: u64| if n == 0 { 1 } else { n * f(n - 1) });
        assert_eq!(fact(0), 1);
        assert_eq!(fact(5), 120);
        assert_eq!(fact(10), 3628800);
    }

    #[test]
    fn test_tree_dfs_depth() {
        // 木の各頂点の深さを計算する DFS
        let adj = [vec![1, 2], vec![0, 3], vec![0], vec![1]]; // 根=0
        let mut depth = vec![0usize; 4];
        let mut dfs = recurse_mut(|dfs, (v, d): (usize, usize)| {
            depth[v] = d;
            for &u in &adj[v] {
                if depth[u] == 0 && u != 0 {
                    dfs((u, d + 1));
                }
            }
        });
        dfs((0, 0));
        drop(dfs);
        assert_eq!(depth, [0, 1, 1, 2]);
    }

    #[test]
    fn test_tree_subtree_size() {
        // 木の各頂点の部分木サイズを計算する DFS（木DP）
        let adj = [vec![1, 2], vec![0, 3], vec![0], vec![1]]; // 根=0
        let mut visited = [false; 4];
        let mut subtree_size = [0usize; 4];

        let mut dfs = recurse_mut(|dfs, v: usize| -> usize {
            visited[v] = true;
            subtree_size[v] = 1;
            for &u in &adj[v] {
                if !visited[u] {
                    subtree_size[v] += dfs(u);
                }
            }
            subtree_size[v]
        });

        dfs(0);
        drop(dfs);
        assert_eq!(subtree_size, [4, 2, 1, 1]);
    }

    #[test]
    fn test_abc448_d_duplicate_detection() {
        use hashbag::HashBag;

        // ABC448 D 相当：根付き木の各頂点に値が割り当てられている。
        // 頂点 v の答えは、根から v へのパス上に v と同じ値が存在するかどうか。
        // HashBag で多重集合を管理し、bag.len() != bag.set_len() で重複判定。

        let children = [vec![1usize, 2], vec![3], vec![], vec![]];
        let xs = [1i64, 2, 1, 2];
        let mut bag: HashBag<i64> = HashBag::new();
        let mut has_dup = vec![false; 4];

        let mut dfs = recurse_mut(|dfs, v: usize| {
            bag.insert(xs[v]);
            has_dup[v] = bag.len() != bag.set_len();
            for &c in &children[v] {
                dfs(c);
            }
            bag.remove(&xs[v]);
        });
        dfs(0);
        drop(dfs);
        assert_eq!(has_dup, [false, false, true, true]);
    }

    #[test]
    fn test_two_argument_recursion() {
        // タプルを使った2引数の再帰：二項係数 C(n, k) の計算
        // C(n, k) = C(n-1, k-1) + C(n-1, k)
        let mut memo = vec![vec![None; 7]; 7];

        let mut binomial = recurse_mut(|binom, (n, k): (usize, usize)| -> u64 {
            if let Some(result) = memo[n][k] {
                return result;
            }
            let result = match (n, k) {
                (_, 0) => 1,
                (n, k) if k > n => 0,
                (n, k) if k == n => 1,
                _ => binom((n - 1, k - 1)) + binom((n - 1, k)),
            };
            memo[n][k] = Some(result);
            result
        });

        assert_eq!(binomial((4, 2)), 6); // C(4, 2) = 6
        assert_eq!(binomial((5, 2)), 10); // C(5, 2) = 10
        assert_eq!(binomial((6, 3)), 20); // C(6, 3) = 20
    }
}
