#[derive_readable]
struct Query {
    a: Usize1,
    b: Usize1,
    dist: i64,
}
struct Problem {
    len: usize,
    ns: usize,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            len: usize,
            ns: usize,
            qs: [Query; ns],
        }
        Problem { len, ns, qs }
    }

    fn solve2(&self) -> Answer {
        let Problem { len, ns, qs } = self;
        let mut uf = PotentializedDsu::<AdditiveAbGroup<i64>>::new(*len);
        let mut ans = vec![];
        for (i, &Query { a, b, dist }) in qs.iter().enumerate() {
            let merge_result = uf.merge(b, a, dist);
            if matches!(
                merge_result,
                MergeResult::Unchanged | MergeResult::Merged { .. }
            ) {
                ans.push(i);
            }
        }
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        print_vec_1line(&self.ans.iter().map(|x| x + 1).collect_vec());
    }
}

fn main() {
    Problem::read().solve2().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
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
use dsu_core::*;
use potentialized_dsu::*;
#[allow(clippy::module_inception)]
/// ac_library::Dsu の merge のみ実装を変えたもの
pub mod dsu_core {
    pub struct DsuCore {
        n: usize,
        parent_or_size: Vec<i32>,
        cnt_groups: usize,
    }
    impl DsuCore {
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
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
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
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
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
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
pub mod potentialized_dsu {
    use std::{
        convert::Infallible,
        iter::Sum,
        marker::PhantomData,
        ops::{Add, Neg},
    };
    /// 可換群 (Abelian Group)
    pub trait AbGroup {
        type S: Clone;
        fn zero() -> Self::S;
        fn add(a: &Self::S, b: &Self::S) -> Self::S;
        fn neg(a: &Self::S) -> Self::S;
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            Self::add(a, &Self::neg(b))
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct AdditiveAbGroup<T>(Infallible, PhantomData<fn() -> T>);
    impl<T: Sum + Add<Output = T> + Neg<Output = T> + Copy> AbGroup for AdditiveAbGroup<T> {
        type S = T;
        fn zero() -> Self::S {
            std::iter::empty().sum()
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
        fn neg(a: &Self::S) -> Self::S {
            -(*a)
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct XorAbGroup(Infallible);
    impl AbGroup for XorAbGroup {
        type S = u64;
        fn zero() -> Self::S {
            0
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
        fn neg(a: &Self::S) -> Self::S {
            *a
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MergeResult {
        /// 新しくマージされた場合
        Merged { leader: usize, merged: usize },
        /// すでに同じ集合だった場合（変化なし）
        Unchanged,
        /// 矛盾があった場合
        Contradiction,
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PotentializedDsu<G: AbGroup>
    where
        G::S: PartialEq,
    {
        n: usize,
        parent_or_size: Vec<i32>,
        p_diff: Vec<G::S>,
        cnt_groups: usize,
    }
    impl<G: AbGroup> PotentializedDsu<G>
    where
        G::S: PartialEq,
    {
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
                p_diff: vec![G::zero(); size],
                cnt_groups: size,
            }
        }
        /// 2 つの要素 `src` と `dst` が属する集合を統合する。
        /// diff = dst のポテンシャル - src のポテンシャル となるように統合する
        pub fn merge(&mut self, src: usize, dst: usize, mut diff: G::S) -> MergeResult {
            assert!(src < self.n);
            assert!(dst < self.n);
            let (mut lsrc, mut psrc) = self.leader_potential(src);
            let (mut ldst, mut pdst) = self.leader_potential(dst);
            if lsrc == ldst {
                let result = if self.diff(src, dst).unwrap() == diff {
                    MergeResult::Unchanged
                } else {
                    MergeResult::Contradiction
                };
                return result;
            }
            if -self.parent_or_size[ldst] < -self.parent_or_size[lsrc] {
                std::mem::swap(&mut lsrc, &mut ldst);
                std::mem::swap(&mut psrc, &mut pdst);
                diff = G::neg(&diff);
            }
            self.parent_or_size[ldst] += self.parent_or_size[lsrc];
            self.parent_or_size[lsrc] = ldst as i32;
            self.cnt_groups -= 1;
            let ldiff = G::add(&G::neg(&psrc), &G::add(&diff, &pdst));
            self.p_diff[lsrc] = ldiff;
            MergeResult::Merged {
                leader: ldst,
                merged: lsrc,
            }
        }
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        /// dst のポテンシャル - src のポテンシャル を求める
        pub fn diff(&mut self, src: usize, dst: usize) -> Option<G::S> {
            if self.same(src, dst) {
                let (_, psrc) = self.leader_potential(src);
                let (_, pdst) = self.leader_potential(dst);
                let diff = G::sub(&psrc, &pdst);
                Some(diff)
            } else {
                None
            }
        }
        fn leader_potential(&mut self, a: usize) -> (usize, G::S) {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return (a, G::zero());
            }
            let parent = self.parent_or_size[a] as usize;
            let (leader, parent_potential) = self.leader_potential(parent);
            self.parent_or_size[a] = leader as i32;
            let potential = G::add(&self.p_diff[a], &parent_potential);
            self.p_diff[a] = potential.clone();
            (leader, potential)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            self.leader_potential(a).0
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
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
