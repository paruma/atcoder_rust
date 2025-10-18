#![allow(dead_code)]

use hashbag::HashBag;
use itertools::Itertools;

/// 数列 xs の連続部分列で種類数が k 以上になる最小の長さを求める
/// そういう連続部分列が存在しない場合は None を返す
fn solve(xs: &[i64], k: usize) -> Option<usize> {
    // begin を 1 ずつ動かす尺取法
    let n = xs.len();
    let mut end = 0;
    let mut bag = HashBag::<i64>::new();

    let mut lens = vec![];

    'begin_for: for begin in 0..n {
        // 条件を満たすまで end を動かす
        while bag.set_len() < k {
            if end == n {
                // [begin, n) ですら種類数が k 未満
                // →種類数が k 以上になる begin から連続部分列は存在しない
                break 'begin_for;
            }
            bag.insert(xs[end]);
            end += 1;
        }

        lens.push(end - begin);
        bag.remove(&xs[begin]);
    }

    lens.iter().copied().min()
}

fn solve_naive(xs: &[i64], k: usize) -> Option<usize> {
    let n = xs.len();
    (0..n)
        .flat_map(|begin| (begin..=n).map(move |end| (begin, end)))
        .filter(|&(begin, end)| xs[begin..end].iter().unique().count() >= k)
        .map(|(begin, end)| end - begin)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        {
            let xs = [1, 2, 3, 4, 5];
            let k = 3;
            assert_eq!(solve(&xs, k), Some(3));
            // assert_eq!(solve2(&xs, k), 3);
            assert_eq!(solve_naive(&xs, k), Some(3));
        }
        {
            // [3, 4, 4, 5] が種類数3以上の最短連続部分列
            let xs = [2, 3, 3, 3, 4, 4, 5, 5, 5, 5, 5, 6];
            let k = 3;
            assert_eq!(solve(&xs, k), Some(4));
            assert_eq!(solve_naive(&xs, k), Some(4));
        }
        {
            let xs = [1, 1, 1, 1, 1];
            let k = 3;
            assert_eq!(solve(&xs, k), None);
            assert_eq!(solve_naive(&xs, k), None);
        }
    }

    #[test]
    fn random_test() {
        use rand::{rngs::SmallRng, *};
        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..100 {
            let n = rng.random_range(1..6);
            let xs = (0..n).map(|_| rng.random_range(1..6)).collect_vec();
            let k = rng.random_range(1..6); // k = 0 だと動かない
            let naive_ans = solve_naive(&xs, k);
            assert_eq!(solve(&xs, k), naive_ans);
        }
    }
}
