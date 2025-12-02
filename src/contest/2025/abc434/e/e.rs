#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive_readable]
struct Rabbit {
    x: i64,
    r: i64,
}
// #[fastout]
fn main() {
    input! {
        n: usize,
        rabbits: [Rabbit; n],
    }

    let cand = rabbits
        .iter()
        .copied()
        .flat_map(|rabbit| [rabbit.x + rabbit.r, rabbit.x - rabbit.r])
        .collect_vec();
    let cc = CoordinateCompression::new(&cand);

    let cc_size = cc.space_size();

    let mut mf: MfGraph<i64> = MfGraph::new(n + cc_size + 2);
    let source = n + cc_size;
    let terminal = n + cc_size + 1;
    let id_cc = |cc_x: usize| cc_x + n;

    for i in 0..n {
        mf.add_edge(source, i, 1);
    }

    for (i, r) in rabbits.iter().copied().enumerate() {
        mf.add_edge(i, id_cc(cc.compress(r.x + r.r)), 1);
        mf.add_edge(i, id_cc(cc.compress(r.x - r.r)), 1);
    }

    for cc_x in 0..cc_size {
        mf.add_edge(id_cc(cc_x), terminal, 1);
    }

    let ans: i64 = mf.flow(source, terminal);
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
use {ac_library::MfGraph, coordinate_compression::*};
#[allow(clippy::module_inception)]
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;
    pub struct CoordinateCompression {
        space: Vec<i64>,
    }
    impl CoordinateCompression {
        /// # 計算量
        /// O(|space|log(|space|))
        pub fn new(space: &[i64]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }
        /// # 計算量
        /// O(log(|space|))
        pub fn compress(&self, x: i64) -> usize {
            self.space.binary_search(&x).unwrap()
        }
        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log(|space|))
        pub fn compress_floor(&self, x: i64) -> usize {
            self.space.upper_bound(&x) - 1
        }
        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log(|space|))
        pub fn compress_ceil(&self, x: i64) -> usize {
            self.space.lower_bound(&x)
        }
        /// # 計算量
        /// O(|xs|log(|space|))
        pub fn compress_vec(&self, xs: &[i64]) -> Vec<usize> {
            xs.iter().copied().map(|x| self.compress(x)).collect_vec()
        }
        /// # 計算量
        /// O(1)
        pub fn decompress(&self, i: usize) -> i64 {
            self.space[i]
        }
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
    }
}
