// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        es: [(Usize1, Usize1); q],
    }

    let mut count_geq_n = vec![0; 2 * n];
    for i in n..2 * n {
        count_geq_n[i] = 1;
    }
    let mut dsu: MonoidDsu<Additive<i64>> = MonoidDsu::new(&count_geq_n);

    let mut black_cnt = 0; // 二部グラフの小さい方のグループの頂点数

    let mut is_bipartite = true;

    let mut ans = vec![];

    for &(u, v) in &es {
        if !dsu.same(u, v + n) {
            let before =
                { i64::min(dsu.prod(u), dsu.prod(u + n)) + i64::min(dsu.prod(v), dsu.prod(v + n)) };
            dsu.merge(u, v + n);
            dsu.merge(u + n, v);
            let after = i64::min(dsu.prod(u), dsu.prod(u + n));
            // dbg!(before);
            // dbg!(after);
            black_cnt += after - before;
        }

        // black_cnt
        if !is_bipartite || dsu.same(u, u + n) || dsu.same(v, v + n) {
            is_bipartite = false;
            ans.push(None);
        } else {
            ans.push(Some(black_cnt));
        }
    }

    // 出力
    let ans = ans
        .iter()
        .copied()
        .map(|x| x.unwrap_or(-1_i64))
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
use monoid_dsu::*;
use {ac_library::Additive, dsu_core::*};
#[allow(clippy::module_inception)]
/// ac_library::Dsu の merge のみ実装を変えたもの
pub mod dsu_core {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// DSU 内の各要素の状態（親のインデックスまたは集合のサイズ）を保持する構造体。
    /// メモリ効率（32ビット整数 1 つ分）を維持したまま、以下の 2 つの状態を表現します。
    /// 1. **Root (根)**:
    ///    - 値が負の場合、その要素は集合の代表元（リーダー）です。
    ///    - 値の絶対値 `|v|` は、その集合に属する要素の数（サイズ）を表します。
    ///    - 例: `-1` はサイズ 1 の集合の根、`-5` はサイズ 5 の集合の根。
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
    #[derive(Clone, Debug)]
    pub struct DsuCore {
        n: usize,
        nodes: Vec<Node>,
        cnt_groups: usize,
    }
    impl DsuCore {
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                nodes: vec![Node::root(1); size],
                cnt_groups: size,
            }
        }
        /// 2 つの要素 `a` と `b` が属する集合を統合する
        /// # 戻り値
        /// - `Some((leader, merged))`:
        ///   - `leader` は統合後の集合の代表元（リーダー）
        ///   - `merged` は統合されて消える側の旧代表元
        /// - `None`:
        ///   - `a` と `b` がすでに同じ集合に属していた場合
        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return None;
            }
            if self.nodes[x].size() < self.nodes[y].size() {
                std::mem::swap(&mut x, &mut y);
            }
            let size_x = self.nodes[x].size();
            let size_y = self.nodes[y].size();
            self.nodes[x] = Node::root(size_x + size_y);
            self.nodes[y] = Node::child(x);
            self.cnt_groups -= 1;
            Some((x, y))
        }
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.nodes[a].is_root() {
                return a;
            }
            let parent = self.nodes[a].parent();
            let new_parent = self.leader(parent);
            self.nodes[a] = Node::child(new_parent);
            new_parent
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            self.nodes[x].size()
        }
        pub fn count_group(&self) -> usize {
            self.cnt_groups
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
#[allow(clippy::module_inception)]
pub mod monoid_dsu {
    use super::DsuCore;
    use ac_library::Monoid;
    #[derive(Clone, Debug)]
    pub struct MonoidDsu<M: Monoid> {
        dsu: DsuCore,
        prods: Vec<M::S>,
    }
    impl<M: Monoid> MonoidDsu<M> {
        pub fn new(data: &[M::S]) -> MonoidDsu<M> {
            let dsu = DsuCore::new(data.len());
            MonoidDsu {
                dsu,
                prods: data.to_vec(),
            }
        }
        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            let merge_result = self.dsu.merge(a, b);
            if let Some((leader, merged)) = merge_result {
                self.prods[leader] = M::binary_operation(&self.prods[leader], &self.prods[merged]);
                self.prods[merged] = M::identity();
            }
            merge_result
        }
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            self.dsu.same(a, b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            self.dsu.leader(a)
        }
        pub fn size(&mut self, a: usize) -> usize {
            self.dsu.size(a)
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            self.dsu.groups()
        }
        pub fn count_group(&self) -> usize {
            self.dsu.count_group()
        }
        pub fn prod(&mut self, a: usize) -> M::S {
            let leader = self.leader(a);
            self.prods[leader].clone()
        }
        /// 指定された要素 `a` が属するグループの現在の積に対して、新しい値 `d` を二項演算で適用する。
        /// この操作は、グループの積 `p` を `M::binary_operation(p, d)` で更新する。
        /// # 例
        /// `Additive` Monoid を使用している場合、グループの合計値に `d` を加算する。
        /// `Max` Monoid を使用している場合、グループの最大値を `max(current_max, d)` で更新する。
        pub fn apply(&mut self, a: usize, d: &M::S) {
            let leader = self.leader(a);
            self.prods[leader] = M::binary_operation(&self.prods[leader], d);
        }
        pub fn groups_with_prod(&mut self) -> Vec<(Vec<usize>, M::S)> {
            self.groups()
                .into_iter()
                .map(|x| {
                    let prod = self.prod(x[0]);
                    (x, prod)
                })
                .collect()
        }
    }
}
