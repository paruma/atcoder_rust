#[derive_readable]
#[derive(Debug, Clone)]
struct Edge {
    u: Usize1,
    v: Usize1,
    w: usize,
}

fn bit_get(bit: usize, i: usize) -> usize {
    (bit >> i) & 1
}
fn main() {
    input! {
        nv: usize,
        ne: usize,
        es: [Edge; ne],
    }

    let mut mask = BitSet::new((1 << 30) - 1);
    // dbg!(mask);

    for i in (0..30).rev() {
        let mut uf = UnionFind::new(nv);
        let next_mask = mask.removed(i);
        // dbg!(next_mask);

        for e in &es {
            if (e.w & next_mask.to_bit()) == e.w {
                uf.unite(e.u, e.v);
            }
        }

        // dbg!(uf.groups());

        if uf.same(0, nv - 1) {
            // dbg!("same");
            mask = next_mask;
        } else {
            //  dbg!("not same");

            mask = next_mask.inserted(i);
        }
        // dbg!(mask);
        // dbg!();
    }

    let ans: usize = mask.to_bit();
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
}

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
use simple_union_find::*;
pub mod simple_union_find {
    use itertools::Itertools;
    #[derive(Clone, Debug)]
    struct RootInfo {
        count: usize,
    }
    #[derive(Clone, Debug)]
    struct NonRootInfo {
        parent: usize,
    }
    #[derive(Clone, Debug)]
    enum Node {
        Root(RootInfo),
        NonRoot(NonRootInfo),
    }
    impl Node {
        fn root(count: usize) -> Node {
            Node::Root(RootInfo { count })
        }
        fn non_root(parent: usize) -> Node {
            Node::NonRoot(NonRootInfo { parent })
        }
        fn as_root(&self) -> &RootInfo {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }
    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            let nodes = (0..n).map(|_| Node::root(1)).collect_vec();
            UnionFind {
                nodes,
                cnt_groups: n,
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            match &self.nodes[index] {
                Node::Root(_) => index,
                Node::NonRoot(info) => {
                    let root = self.root(info.parent);
                    self.nodes[index] = Node::non_root(root);
                    root
                }
            }
        }
        pub fn same_count(&mut self, index: usize) -> usize {
            let root_index = self.root(index);
            self.nodes[root_index].as_root().count
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn num_groups(&self) -> usize {
            self.cnt_groups
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            if self.same(x, y) {
                return false;
            }
            self.cnt_groups -= 1;
            let (smaller_root, larger_root) = {
                let x_root = self.root(x);
                let y_root = self.root(y);
                let x_count = self.nodes[x_root].as_root().count;
                let y_count = self.nodes[y_root].as_root().count;
                if x_count < y_count {
                    (x_root, y_root)
                } else {
                    (y_root, x_root)
                }
            };
            let count_sum =
                self.nodes[smaller_root].as_root().count + self.nodes[larger_root].as_root().count;
            self.nodes[smaller_root] = Node::non_root(larger_root);
            self.nodes[larger_root] = Node::root(count_sum);
            true
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let n = self.nodes.len();
            let roots = (0..n).map(|i| self.root(i)).collect_vec();
            let group_size = (0..n).map(|i| roots[i]).fold(vec![0; n], |mut acc, x| {
                acc[x] += 1;
                acc
            });
            let result = {
                let mut result = vec![Vec::new(); n];
                for i in 0..n {
                    result[i].reserve(group_size[i]);
                }
                for i in 0..n {
                    result[roots[i]].push(i);
                }
                result
            };
            result.into_iter().filter(|x| !x.is_empty()).collect_vec()
        }
    }
}
use bitset::*;
#[allow(clippy::module_inception)]
pub mod bitset {
    use itertools::Itertools;
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor, Index, IndexMut},
    };
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BitSet {
        bit: usize,
    }
    impl BitSet {
        #[inline]
        pub fn new(bit: usize) -> BitSet {
            BitSet { bit }
        }
        pub fn to_bit(self) -> usize {
            self.bit
        }
        /// 持っている要素を Vec<usize> で返す
        pub fn to_vec(self, len: usize) -> Vec<usize> {
            (0..len).filter(|i| (self.bit >> i) & 1 == 1).collect_vec()
        }
        /// 持っている要素を Iterator で返す
        pub fn to_iter(self, len: usize) -> impl Iterator<Item = usize> {
            (0..len).filter(move |i| (self.bit >> i) & 1 == 1)
        }
        pub fn contains(self, x: usize) -> bool {
            (self.bit >> x) & 1 == 1
        }
        pub fn len(self) -> usize {
            self.bit.count_ones() as usize
        }
        pub fn inserted(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }
        pub fn removed(self, x: usize) -> BitSet {
            BitSet::new(self.bit & !(1 << x))
        }
        pub fn empty() -> BitSet {
            BitSet::new(0)
        }
        pub fn universal_set(size: usize) -> BitSet {
            BitSet::new((1 << size) - 1)
        }
        pub fn complement(self, size: usize) -> BitSet {
            BitSet::new(self.bit ^ ((1 << size) - 1))
        }
        pub fn set_minus(self, other: BitSet) -> BitSet {
            BitSet::new(self.bit & !other.bit)
        }
        pub fn is_empty(self) -> bool {
            self.bit == 0
        }
        pub fn is_subset(self, other: BitSet) -> bool {
            self | other == other
        }
        pub fn all_subset(size: usize) -> impl Iterator<Item = BitSet> {
            (0..(1 << size)).map(BitSet::new)
        }
        pub fn subsets(self) -> impl Iterator<Item = BitSet> {
            std::iter::successors(Some(self.bit), move |x| {
                if *x == 0 {
                    None
                } else {
                    Some((x - 1) & self.bit)
                }
            })
            .map(BitSet::new)
        }
    }
    impl BitAnd for BitSet {
        type Output = BitSet;
        fn bitand(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit & rhs.bit)
        }
    }
    impl BitOr for BitSet {
        type Output = BitSet;
        fn bitor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit | rhs.bit)
        }
    }
    impl BitXor for BitSet {
        type Output = BitSet;
        fn bitxor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit ^ rhs.bit)
        }
    }
    use std::fmt::Debug;
    impl Debug for BitSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("{:#b}", self.bit))?;
            Ok(())
        }
    }
    impl<T> Index<BitSet> for [T] {
        type Output = T;
        fn index(&self, s: BitSet) -> &Self::Output {
            &self[s.to_bit()]
        }
    }
    impl<T> IndexMut<BitSet> for [T] {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[s.to_bit()]
        }
    }
    impl<T> Index<BitSet> for Vec<T> {
        type Output = T;
        fn index(&self, s: BitSet) -> &Self::Output {
            &self[..][s]
        }
    }
    impl<T> IndexMut<BitSet> for Vec<T> {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[..][s]
        }
    }
}
