// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: Chars,
        abs: [(Usize1, Usize1); m]
    }

    let xs = xs
        .iter()
        .copied()
        .map(|ch| ch as i64 - 'a' as i64)
        .collect_vec();
    use ac_library::ModInt998244353 as Mint;

    let groups = {
        let mut dsu = DsuCore::new(n);

        for &(a, b) in &abs {
            dsu.merge(a, b);
        }
        dsu.groups()
    };

    // 同じ連結成分に同じ文字がいるか？
    let mut has_same = false;
    let mut set_by_group = vec![HashBag::<i64>::new(); groups.len()];
    let mut vec_by_group = vec![Vec::<i64>::new(); groups.len()];

    for gi in 0..groups.len() {
        for &i in &groups[gi] {
            if set_by_group[gi].contains(&xs[i]) != 0 {
                has_same = true;
            }
            set_by_group[gi].insert(xs[i]);
            vec_by_group[gi].push(xs[i]);
        }
    }

    let counts_by_group = vec_by_group
        .iter()
        .map(|vec| vec.iter().copied().counts().values().copied().collect_vec())
        .collect_vec();

    // dbg!(&counts_by_group);

    let comb: Comb<Mint> = Comb::new(n);

    // 連結成分が1の場合は注意

    let factor1 = counts_by_group
        .iter()
        .map(|cnts| {
            let sum = cnts.iter().copied().sum::<usize>();

            let bunshi = comb.factorial(sum);
            let bunbo = cnts
                .iter()
                .copied()
                .map(|cnt| comb.factorial(cnt))
                .product::<Mint>();
            bunshi / bunbo
        })
        .product::<Mint>();

    let ans: Mint = if has_same { factor1 } else { factor1 / 2 };
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
        for ch in 'a'..='z' {
            println!("{} {}", ch as i64 - 'a' as i64, ch);
        }
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
use {hashbag::HashBag, mod_combinatorics::*};
pub mod mod_combinatorics {
    use ac_library::modint::ModIntBase;
    #[derive(Clone, Debug)]
    pub struct Comb<Mint: ModIntBase> {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }
    impl<Mint: ModIntBase> Comb<Mint> {
        /// 階乗とその逆元を `max_val` まで前計算する。
        /// 計算量: O(max_val)
        pub fn new(max_val: usize) -> Self {
            let mut inv = vec![Mint::new(0); max_val + 1];
            let mut fac = vec![Mint::new(0); max_val + 1];
            let mut invfac = vec![Mint::new(0); max_val + 1];
            fac[0] = 1.into();
            fac[1] = 1.into();
            invfac[0] = 1.into();
            invfac[1] = 1.into();
            inv[1] = 1.into();
            let modulus = Mint::modulus() as usize;
            for i in 2..=max_val {
                inv[i] = -inv[modulus % i] * Mint::new(modulus / i);
                fac[i] = fac[i - 1] * Mint::new(i);
                invfac[i] = invfac[i - 1] * inv[i];
            }
            Self { fac, invfac }
        }
        pub fn comb(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[k] * self.invfac[n - k]
            }
        }
        pub fn perm(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[n - k]
            }
        }
        pub fn factorial(&self, n: usize) -> Mint {
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            self.fac[n]
        }
        pub fn inv_factorial(&self, n: usize) -> Mint {
            assert!(
                n < self.invfac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.invfac.len() - 1
            );
            self.invfac[n]
        }
    }
}
use dsu_core::*;
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
