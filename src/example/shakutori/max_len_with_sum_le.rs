#![allow(dead_code)]
use itertools::Itertools;

/// 数列 xs の部分列で総和が k 以下になる最大の長さを求める
fn solve(xs: &[i64], k: i64) -> usize {
    // end を 1 ずつ動かす尺取法
    let n = xs.len();
    let mut begin = 0;
    let mut sum = 0;
    let mut max_len = 0;
    for end in 1..=n {
        sum += xs[end - 1];

        while sum > k {
            sum -= xs[begin];
            begin += 1;
        }

        max_len = max_len.max(end - begin);
    }

    max_len
}

/// 数列 xs の部分列で総和が k 以下になる最大の長さを求める
fn solve2(xs: &[i64], k: i64) -> usize {
    // begin を 1 ずつ動かす尺取法
    let n = xs.len();
    let mut begin = 0;
    let mut end = 0;
    let mut sum = 0;
    let mut max_len = 0;

    while begin < n {
        // begin..end が条件を満たす範囲で end を繰り返し進める
        while end < n {
            // end を1進めたときに条件を満たさなくなる場合は break
            if sum + xs[end] > k {
                break;
            }

            // end を進める
            sum += xs[end];
            end += 1;
        }

        max_len = max_len.max(end - begin);

        if begin == end {
            end += 1; // begin が end を追い抜かなさいように end も進める。
            begin += 1;
        } else {
            // begin を進める
            sum -= xs[begin];
            begin += 1;
        }
    }
    max_len
}

fn solve_naive(xs: &[i64], k: i64) -> usize {
    let n = xs.len();
    (0..=n)
        .tuple_combinations()
        .filter(|(begin, end)| xs[*begin..*end].iter().sum::<i64>() <= k)
        .map(|(begin, end)| end - begin)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        let xs = [2, 2, 1, 1, 3];
        let k = 5;
        assert_eq!(solve_naive(&xs, k), 3);
        assert_eq!(solve(&xs, k), 3);
        assert_eq!(solve2(&xs, k), 3);
    }
    #[test]
    fn random_test() {
        use rand::{rngs::SmallRng, *};
        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..100 {
            let n = rng.random_range(1..6);
            let xs = (0..n).map(|_| rng.random_range(1..6)).collect_vec();
            let k = rng.random_range(0..30);
            let naive_ans = solve_naive(&xs, k);
            assert_eq!(solve(&xs, k), naive_ans);
            assert_eq!(solve2(&xs, k), naive_ans);
        }
    }
}
