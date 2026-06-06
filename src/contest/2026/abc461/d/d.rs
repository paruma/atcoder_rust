// 問題文と制約は読みましたか？
// #[fastout]

// k 以下
fn solve(h: usize, w: usize, cumsum: &CumSum2D, k: i64) -> i64 {
    if k < 0 {
        return 0;
    }
    (0..=w)
        .tuple_combinations()
        .map(|(l, r)| {
            let mut u = 0;
            let mut cnt = 0;
            for d in 1..=h {
                while cumsum.rect_sum(u..d, l..r) > k {
                    u += 1;
                }
                cnt += (d - u) as i64;
            }
            cnt
        })
        .sum::<i64>()
}

fn main() {
    input! {
        h: usize, w: usize,
        k: i64,
        grid: [Chars; h],
    }

    let grid = grid
        .iter()
        .map(|row| {
            row.iter()
                .copied()
                .map(|ch| (ch == '1') as i64)
                .collect_vec()
        })
        .collect_vec();
    let cumsum = {
        let mut cumsum = vec![vec![0; w + 1]; h + 1];
        for y in 1..h + 1 {
            for x in 1..w + 1 {
                cumsum[y][x] =
                    cumsum[y - 1][x] + cumsum[y][x - 1] - cumsum[y - 1][x - 1] + grid[y - 1][x - 1];
            }
        }
        cumsum
    };

    let cumsum = CumSum2D::new(&grid);

    let ans: i64 = solve(h, w, &cumsum, k) - solve(h, w, &cumsum, k - 1);
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
use cumsum_2d::*;
#[allow(clippy::module_inception)]
pub mod cumsum_2d {
    use std::ops::{Bound, Range, RangeBounds};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum2D {
        pub cumsum: Vec<Vec<i64>>,
    }
    impl CumSum2D {
        pub fn new(xss: &[Vec<i64>]) -> CumSum2D {
            if xss.is_empty() {
                return CumSum2D {
                    cumsum: vec![vec![0]],
                };
            }
            let height = xss.len();
            let width = xss[0].len();
            let mut cumsum = vec![vec![0; width + 1]; height + 1];
            for y in 1..height + 1 {
                for x in 1..width + 1 {
                    cumsum[y][x] = cumsum[y - 1][x] + cumsum[y][x - 1] - cumsum[y - 1][x - 1]
                        + xss[y - 1][x - 1];
                }
            }
            CumSum2D { cumsum }
        }
        pub fn rect_sum(
            &self,
            y_range: impl RangeBounds<usize>,
            x_range: impl RangeBounds<usize>,
        ) -> i64 {
            let y_len = self.cumsum.len() - 1;
            let x_len = self.cumsum[0].len() - 1;
            let y_range = open(y_range, y_len);
            let x_range = open(x_range, x_len);
            let y1 = y_range.start;
            let y2 = y_range.end;
            let x1 = x_range.start;
            let x2 = x_range.end;
            self.cumsum[y2][x2] - self.cumsum[y2][x1] - self.cumsum[y1][x2] + self.cumsum[y1][x1]
        }
    }
    fn open(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
        let begin = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let end = match range.end_bound() {
            Bound::Excluded(&x) => x,
            Bound::Included(&x) => x + 1,
            Bound::Unbounded => len,
        };
        begin..end
    }
}
