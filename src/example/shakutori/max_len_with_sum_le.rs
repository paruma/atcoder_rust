#![allow(dead_code)]
use itertools::Itertools;

/// 数列 xs の部分列で総和が k 以下になる最大の長さを求める
fn solve(xs: &[i64], k: i64) -> usize {
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
    }
}
