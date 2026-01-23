use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet("rollback_dsu")]
pub mod rollback_dsu {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// DSU 内の各要素の状態（親のインデックスまたは集合のサイズ）を保持する構造体。
    ///
    /// メモリ効率（32ビット整数 1 つ分）を維持したまま、以下の 2 つの状態を表現します。
    ///
    /// 1. **Root (根)**:
    ///    - 値が負の場合、その要素は集合の代表元（リーダー）です。
    ///    - 値の絶対値 `|v|` は、その集合に属する要素の数（サイズ）を表します。
    ///    - 例: `-1` はサイズ 1 の集合の根、`-5` はサイズ 5 の集合の根。
    ///
    /// 2. **Child (子)**:
    ///    - 値が 0 以上の場合、その要素は他の要素を親に持っています。
    ///    - 値 `v` は、親要素のインデックスを表します。
    struct Node(i32);

    impl Node {
        fn root(size: usize) -> Self {
            Self(-(size as i32))
        }

        fn child(parent: usize) -> Self {
            Self(parent as i32)
        }

        fn is_root(&self) -> bool {
            self.0 < 0
        }

        fn parent(&self) -> usize {
            self.0 as usize
        }

        fn size(&self) -> usize {
            (-self.0) as usize
        }
    }

    /// ロールバック可能なUnion-Find。
    ///
    /// 経路圧縮を行わないため、各操作の計算量は O(log N) となる。
    /// `merge`操作による変更は`rollback`で巻き戻すことができる。
    #[derive(Clone, Debug)]
    pub struct RollbackDsu {
        n: usize,
        nodes: Vec<Node>,
        history: Vec<(usize, Node)>,
    }

    impl RollbackDsu {
        /// `size`要素のUnion-Findを生成する。
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                nodes: vec![Node::root(1); size],
                history: Vec::new(),
            }
        }

        /// `a`が属するグループのリーダーを返す。経路圧縮は行わない。
        pub fn leader(&self, mut a: usize) -> usize {
            while !self.nodes[a].is_root() {
                a = self.nodes[a].parent();
            }
            a
        }

        /// `a`と`b`が同じグループに属するかどうかを返す。
        pub fn same(&self, a: usize, b: usize) -> bool {
            self.leader(a) == self.leader(b)
        }

        /// `a`が属するグループのサイズを返す。
        pub fn size(&self, a: usize) -> usize {
            let leader = self.leader(a);
            self.nodes[leader].size()
        }

        /// 2 つの要素 `a` と `b` が属する集合を統合する
        ///
        /// # 戻り値
        /// - `Some((leader, merged))`:
        ///   - `leader` は統合後の集合の代表元（リーダー）
        ///   - `merged` は統合されて消える側の旧代表元
        /// - `None`:
        ///   - `a` と `b` がすでに同じ集合に属していた場合
        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            let (mut leader_a, mut leader_b) = (self.leader(a), self.leader(b));
            if leader_a == leader_b {
                return None;
            }

            // Union by Size
            if self.nodes[leader_a].size() < self.nodes[leader_b].size() {
                std::mem::swap(&mut leader_a, &mut leader_b);
            }

            // 変更履歴を記録
            self.history.push((leader_a, self.nodes[leader_a]));
            self.history.push((leader_b, self.nodes[leader_b]));

            // マージ
            let size_a = self.nodes[leader_a].size();
            let size_b = self.nodes[leader_b].size();
            self.nodes[leader_a] = Node::root(size_a + size_b);
            self.nodes[leader_b] = Node::child(leader_a);

            Some((leader_a, leader_b))
        }

        /// 現在の状態を記録するスナップショットを作成する。
        ///
        /// このスナップショットは`rollback`メソッドに渡して使用する。
        pub fn snapshot(&self) -> usize {
            self.history.len()
        }

        /// `snapshot`で指定された時点まで状態を巻き戻す。
        pub fn rollback(&mut self, snapshot: usize) {
            while self.history.len() > snapshot {
                let (index, value) = self.history.pop().unwrap();
                self.nodes[index] = value;
            }
        }

        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rollback_dsu::*;
    use itertools::Itertools;

    #[test]
    fn test_rollback_dsu() {
        let mut dsu = RollbackDsu::new(5);

        dsu.merge(0, 1);
        dsu.merge(1, 2);

        assert!(dsu.same(0, 2));
        assert_eq!(dsu.size(0), 3);

        // スナップショットを作成
        let snap = dsu.snapshot();

        dsu.merge(3, 4);
        assert!(dsu.same(3, 4));
        assert_eq!(dsu.size(3), 2);
        assert!(!dsu.same(0, 3));

        // ロールバック
        dsu.rollback(snap);

        // 状態がスナップショット時点に戻っていることを確認
        assert!(!dsu.same(3, 4));
        assert_eq!(dsu.size(3), 1);
        assert_eq!(dsu.size(4), 1);

        // スナップショット以前の状態は維持されている
        assert!(dsu.same(0, 2));
        assert_eq!(dsu.size(0), 3);

        // 最初までロールバック
        dsu.rollback(0);
        assert!(!dsu.same(0, 1));
        assert!(!dsu.same(0, 2));
        assert_eq!(dsu.size(0), 1);
    }

    #[test]
    fn test_rollback_dsu_merge_edge_cases() {
        let mut dsu = RollbackDsu::new(5);

        // 同じ要素のマージ
        assert_eq!(dsu.merge(0, 0), None);

        // 同じグループのマージ
        dsu.merge(0, 1);
        assert_eq!(dsu.merge(0, 1), None);

        // swap ロジックの検証 (size(a) < size(b) の場合に a と b を入れ替える)
        // a: {0, 1} (size 2), leader 0
        // b: {2, 3, 4} (size 3), leader 2
        dsu.merge(2, 3);
        dsu.merge(3, 4);
        assert_eq!(dsu.size(0), 2);
        assert_eq!(dsu.size(2), 3);

        let res = dsu.merge(0, 2);
        assert!(res.is_some());
        let (leader, merged) = res.unwrap();
        // 2の方がサイズが大きいため、2がリーダーになり、0がマージされるはず
        assert_eq!(leader, 2);
        assert_eq!(merged, 0);
        assert_eq!(dsu.leader(0), 2);
        assert_eq!(dsu.size(2), 5);
    }

    #[test]
    fn test_rollback_dsu_groups() {
        let mut dsu = RollbackDsu::new(5);
        dsu.merge(0, 1);
        dsu.merge(2, 3);

        let sorted_groups = |groups: Vec<Vec<usize>>| {
            groups
                .into_iter()
                .map(|mut g| {
                    g.sort();
                    g
                })
                .sorted()
                .collect_vec()
        };

        assert_eq!(
            sorted_groups(dsu.groups()),
            vec![vec![0, 1], vec![2, 3], vec![4]]
        );

        dsu.merge(1, 3);
        assert_eq!(sorted_groups(dsu.groups()), vec![vec![0, 1, 2, 3], vec![4]]);
    }
}
