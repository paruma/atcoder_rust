use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet("rollback_dsu")]
pub mod rollback_dsu {
    /// ロールバック可能なUnion-Find。
    ///
    /// 経路圧縮を行わないため、各操作の計算量は O(log N) となる。
    /// `merge`操作による変更は`rollback`で巻き戻すことができる。
    #[derive(Clone, Debug)]
    pub struct RollbackDsu {
        n: usize,
        parent_or_size: Vec<i32>,
        history: Vec<(usize, i32)>,
    }

    impl RollbackDsu {
        /// `size`要素のUnion-Findを生成する。
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
                history: Vec::new(),
            }
        }

        /// `a`が属するグループのリーダーを返す。経路圧縮は行わない。
        pub fn leader(&self, mut a: usize) -> usize {
            while self.parent_or_size[a] >= 0 {
                a = self.parent_or_size[a] as usize;
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
            -self.parent_or_size[leader] as usize
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
            if -self.parent_or_size[leader_a] < -self.parent_or_size[leader_b] {
                std::mem::swap(&mut leader_a, &mut leader_b);
            }

            // 変更履歴を記録
            self.history.push((leader_a, self.parent_or_size[leader_a]));
            self.history.push((leader_b, self.parent_or_size[leader_b]));

            // マージ
            self.parent_or_size[leader_a] += self.parent_or_size[leader_b];
            self.parent_or_size[leader_b] = leader_a as i32;

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
                self.parent_or_size[index] = value;
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
}
