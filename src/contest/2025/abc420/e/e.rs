// ACL のラッパーを使う
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut uf = UnionFindCore::new(n);
    let mut is_black = vec![false; n];
    let mut group_black_cnt = vec![Some(0); n];

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            }
            if let Some((root, merged)) = uf.unite(u, v) {
                *group_black_cnt[root].as_mut().unwrap() += group_black_cnt[merged].unwrap();
                group_black_cnt[merged] = None;
            }
        } else if t == 2 {
            input! {
                v: Usize1,
            }
            is_black[v] = !is_black[v];

            let root = uf.root(v);
            *group_black_cnt[root].as_mut().unwrap() += if is_black[v] { 1 } else { -1 };
        } else {
            input! {
                v: Usize1,
            }
            let root = uf.root(v);
            let ans = group_black_cnt[root].unwrap() > 0;
            print_yesno(ans);
        }
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

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{chain, iproduct, izip, Itertools},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng},
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
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
use union_find_core::*;
#[allow(clippy::module_inception)]
/// ac_library::Dsu のラッパー
pub mod union_find_core {
    use ac_library::Dsu;
    pub struct UnionFindCore {
        uf: Dsu,
    }
    impl UnionFindCore {
        pub fn new(n: usize) -> UnionFindCore {
            UnionFindCore { uf: Dsu::new(n) }
        }
        pub fn root(&mut self, v: usize) -> usize {
            self.uf.leader(v)
        }
        pub fn same_count(&mut self, v: usize) -> usize {
            self.uf.size(v)
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.uf.same(x, y)
        }
        /// 2 つの要素 `x` と `y` が属する集合を統合します。
        /// # 戻り値
        /// - `Some((root, merged))`:
        ///   - `root` は統合後の集合の代表元（リーダー）
        ///   - `merged` は統合されて消える側の旧代表元
        /// - `None`:
        ///   - `x` と `y` がすでに同じ集合に属していた場合
        /// ```
        pub fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
            let rx = self.uf.leader(x);
            let ry = self.uf.leader(y);
            if rx == ry {
                return None;
            }
            let root = self.uf.merge(rx, ry);
            let merged = root ^ rx ^ ry;
            Some((root, merged))
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            self.uf.groups()
        }
    }
}
