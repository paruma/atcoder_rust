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
    fn solve(&self) -> Answer {
        let Problem { len, ns, qs } = self;
        let mut uf = union_find::UnionFind::new(*len);
        let mut ans = vec![];
        for (i, Query { a, b, dist }) in qs.iter().enumerate() {
            if uf.unite(*a, *b, *dist) {
                ans.push(i)
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
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_uf() {
        use super::union_find::*;
        let mut uf = UnionFind::new(8);
        uf.unite(0, 1, 0);
        uf.unite(3, 4, 0);
        uf.unite(4, 5, 0);
        uf.unite(4, 6, 0);
        uf.unite(1, 4, 0);

        // {0,1,3,4,5,6}, {2}, {7}

        assert_eq!(uf.num_groups(), 3);
        assert!(uf.same(0, 4));
        assert!(!uf.same(2, 4));
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

pub mod union_find {

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Root {
        count: i32,
        value: i64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Node {
        Root { root: Root },
        NonRoot { parent_index: usize, diff: i64 }, //diff は親との値の差分 (自分 - 親)
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RootAndIndex {
        root: Root,
        index: usize,
    }

    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<Node>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            UnionFind { nodes: vec![Node::Root { root: Root { count: 1, value: 0 } }; n] }
        }

        fn root_node(&mut self, index: usize) -> RootAndIndex {
            match self.nodes[index] {
                Node::Root { root } => RootAndIndex { root, index },
                Node::NonRoot { parent_index, diff } => {
                    let root_and_index = self.root_node(parent_index);
                    // TODO: 書き換える
                    // self.nodes[index] = Node::NonRoot { parent_index: root_and_index.index, diff };
                    root_and_index
                }
            }
        }

        pub fn root(&mut self, index: usize) -> usize {
            self.root_node(index).index
        }

        pub fn value(&mut self, index: usize) -> i64 {
            match self.nodes[index] {
                Node::Root { root } => root.value,
                Node::NonRoot { parent_index, diff } => diff + self.value(parent_index),
            }
        }

        pub fn same_count(&mut self, index: usize) -> i32 {
            self.root_node(index).root.count
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn num_groups(&self) -> usize {
            self.nodes.iter().filter(|&node| matches!(node, Node::Root { .. })).count()
        }

        // xの値 - yの値 = diff になるようにする
        pub fn unite(&mut self, x: usize, y: usize, diff: i64) -> bool {
            if self.same(x, y) {
                // 矛盾の判定はここでやる
                return self.value(x) - self.value(y) == diff;
            }

            let x_root_node = self.root_node(x);
            let y_root_node = self.root_node(y);

            // 自分と同じグループのノードの数
            let x_count = x_root_node.root.count;
            let y_count = y_root_node.root.count;

            let x_root_index = x_root_node.index;
            let y_root_index = y_root_node.index;

            let x_value = self.value(x);
            let y_value = self.value(y);
            let x_root_value = self.value(x_root_index);
            let y_root_value = self.value(y_root_index);

            if x_count < y_count {
                // yのrootにxのrootをくっつける
                // xのルートの子供をどうするか...
                // x_root_value は多分捨てる
                // x の value と x_root の value の差分を求める

                let new_x_value = diff + y_value;
                let new_x_root_value = x_root_value + new_x_value - x_value;
                let diff = new_x_root_value - y_root_value;

                self.nodes[x_root_index] = Node::NonRoot { parent_index: y_root_index, diff };

                self.nodes[y_root_index] =
                    Node::Root { root: Root { count: x_count + y_count, value: y_root_value } };
            } else {
                let new_y_value = -diff + x_value;
                let new_y_root_value = y_root_value + new_y_value - y_value;
                let diff = new_y_root_value - x_root_value;

                self.nodes[y_root_index] = Node::NonRoot { parent_index: x_root_index, diff };

                self.nodes[x_root_index] =
                    Node::Root { root: Root { count: y_count + x_count, value: x_root_value } };
            }
            true
        }
    }
}
