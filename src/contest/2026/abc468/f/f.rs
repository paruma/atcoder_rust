// 問題文と制約は読みましたか？
// #[fastout]

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaxUsizeUsize(Infallible);
impl Monoid for MaxUsizeUsize {
    type S = (usize, usize);
    fn identity() -> Self::S {
        (usize::MIN, usize::MIN)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::max(*a, *b)
    }
}

fn lis(xs: &[usize]) -> Vec<usize> {
    if xs.len() == 0 {
        return vec![];
    }
    let n = xs.len();
    let mut dp: Vec<usize> = vec![0; n];
    let mut prev = vec![usize::MAX; n];
    let mut seg = Segtree::<MaxUsizeUsize>::new(n);
    for x in 0..n {
        seg.set(x, (0, usize::MAX));
    }
    for (i, x) in xs.iter().copied().enumerate() {
        dp[i] = seg.prod(..x).0.wrapping_add(1);
        prev[i] = if x == 0 { usize::MAX } else { seg.prod(..x).1 };
        if seg.get(x).0 < dp[i] {
            seg.set(x, (dp[i], i));
        }
    }

    let lis_len = dp.iter().copied().max().unwrap();
    let pos = dp.iter().position(|x| *x == lis_len).unwrap();
    //    dbg!(prev);
    std::iter::successors(Some(pos), |cur| {
        let prev = prev[*cur];
        if prev < n { Some(prev) } else { None }
    })
    .collect_vec()
}
fn main() {
    input! {
        n: usize,
        xs: [Usize1; n],
    }

    let lis1 = lis(&xs);

    let lis_set = lis1.iter().copied().collect::<HashSet<usize>>();
    // dbg!(&lis1);

    let ys = (0..n)
        .filter(|i| !lis_set.contains(&i))
        .map(|i| xs[i])
        .collect_vec();
    let ys_cc = CoordinateCompression::new(&ys).compress_vec(&ys);
    // dbg!(&ys);
    let lis2 = lis(&ys_cc);

    let ans: usize = lis1.len() + lis2.len();
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
use {
    ac_library::{Monoid, Segtree},
    num_traits::WrappingAdd,
    std::convert::Infallible,
};
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
use coordinate_compression::*;
#[allow(clippy::module_inception)]
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;
    #[derive(Debug, Clone)]
    pub struct CoordinateCompression<T> {
        space: Vec<T>,
    }
    impl<T: Ord + Copy> CoordinateCompression<T> {
        /// 与えられた要素から座標圧縮空間を構築する。
        /// # 計算量
        /// O(N log N) (N = |space|)
        pub fn new(space: &[T]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }
        /// 与えられた値を座標圧縮したインデックスを返す。
        /// 値が空間に存在しない場合はパニックする。
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress(&self, x: T) -> usize {
            self.space.binary_search(&x).unwrap()
        }
        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_floor(&self, x: T) -> usize {
            self.space.upper_bound(&x) - 1
        }
        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_ceil(&self, x: T) -> usize {
            self.space.lower_bound(&x)
        }
        /// 与えられた各要素を座標圧縮した結果を返す。
        /// # 計算量
        /// O(M log N) (M = |xs|, N = space_size)
        pub fn compress_vec(&self, xs: &[T]) -> Vec<usize> {
            xs.iter().map(|&x| self.compress(x)).collect_vec()
        }
        /// 指定された範囲内の値に対応する座標圧縮後のインデックス範囲を [begin, end) で返す。
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_range(
            &self,
            range: impl std::ops::RangeBounds<T>,
        ) -> std::ops::Range<usize> {
            use std::ops::Bound::*;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => self.space.lower_bound(&x),
                Excluded(&x) => self.space.upper_bound(&x),
            };
            let end = match range.end_bound() {
                Unbounded => self.space.len(),
                Included(&x) => self.space.upper_bound(&x),
                Excluded(&x) => self.space.lower_bound(&x),
            };
            begin..end
        }
        /// 座標圧縮されたインデックスから元の値を復元する。
        /// # 計算量
        /// O(1)
        pub fn decompress(&self, i: usize) -> T {
            self.space[i]
        }
        /// 座標圧縮後の空間の大きさ（要素数）を返す。
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
    }
}
