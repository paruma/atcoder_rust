fn solve(sts: Vec<(i64, i64)>) -> Option<Vec<usize>> {
    let space = sts.iter().copied().flat_map(|(s, t)| [s, t]).collect_vec();
    let cc = CoordinateCompression::new(&space);
    let sts = sts
        .iter()
        .copied()
        .map(|(s, t)| (cc.compress(s), cc.compress(t)))
        .collect_vec();

    let mut cnts = FenwickTree::new(cc.space_size() + 1, 0);

    for (s, _) in &sts {
        cnts.add(*s, 1);
    }

    let mut ans = vec![];

    for (i, (s, t)) in sts
        .iter()
        .copied()
        .enumerate()
        .sorted_by_key(|(_, (s, t))| {
            let is_right = t > s;
            let key = if is_right { -(*s as i64) } else { *s as i64 };
            (is_right, key)
        })
    {
        let min = usize::min(s, t);
        let max = usize::max(s, t);
        if cnts.sum(min..=max) > 1 {
            return None;
        }

        cnts.add(s, -1);
        cnts.add(t, 1);
        ans.push(i);
    }

    Some(ans)
}

fn solve_naive(sts: Vec<(i64, i64)>) -> Option<Vec<usize>> {
    let max = sts.iter().copied().flat_map(|(s, t)| [s, t]).max().unwrap();

    let n = sts.len();

    (0..n).permutations(n).find(|ps| {
        let mut cnts = FenwickTree::new((max as usize) + 1, 0_i64);
        for (s, _) in &sts {
            cnts.add(*s as usize, 1);
        }

        for &i in ps {
            let (s, t) = sts[i];
            let min = i64::min(s, t) as usize;
            let max = i64::max(s, t) as usize;

            if cnts.sum(min..=max) > 1 {
                return false;
            }

            cnts.add(s as usize, -1);
            cnts.add(t as usize, 1);
        }

        true
    })
}
fn main() {
    input! {
        n: usize,
        sts: [(i64, i64); n],
    }

    // dbg!("naive");
    let ans = solve(sts);

    if let Some(ans) = ans {
        println!("Yes");
        let ans = ans.iter().copied().map(|i| i + 1).collect_vec();
        print_vec_1line(&ans);
    } else {
        println!("No");
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
        let n = rng.random_range(1..=6);
        let sts = (0..n)
            .map(|_| (rng.random_range(1..100), rng.random_range(1..10)))
            .collect_vec();

        // ==== 解く ====
        let main_ans = solve(sts.clone());
        let naive_ans = solve_naive(sts.clone());

        // ==== 間違っていたら報告をする ====
        if main_ans.is_some() != naive_ans.is_some() {
            // 問題を出力
            println!("{:?}", sts);
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
use {ac_library::FenwickTree, coordinate_compression::*};
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;
    pub struct CoordinateCompression {
        space: Vec<i64>,
    }
    impl CoordinateCompression {
        /// 計算量: O(|space|log(|space|))
        pub fn new(space: &[i64]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }
        /// 計算量: O(log(|space|))
        pub fn compress(&self, x: i64) -> usize {
            self.space.binary_search(&x).unwrap()
        }
        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        /// 計算量: O(log(|space|))
        pub fn compress_floor(&self, x: i64) -> usize {
            self.space.upper_bound(&x) - 1
        }
        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        /// 計算量: O(log(|space|))
        pub fn compress_ceil(&self, x: i64) -> usize {
            self.space.lower_bound(&x)
        }
        /// 計算量: O(|xs|log(|space|))
        pub fn compress_vec(&self, xs: &[i64]) -> Vec<usize> {
            xs.iter().copied().map(|x| self.compress(x)).collect_vec()
        }
        /// 計算量: O(1)
        pub fn decompress(&self, i: usize) -> i64 {
            self.space[i]
        }
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
    }
}
pub fn fenwick_tree_to_vec<T>(fenwick_tree: &ac_library::FenwickTree<T>, len: usize) -> Vec<T>
where
    T: Clone + std::ops::AddAssign<T> + std::ops::Sub<Output = T>,
{
    (0..len).map(|i| fenwick_tree.sum(i..=i)).collect()
}
