#![allow(dead_code)]

use hashbag::HashBag;
use itertools::Itertools;

use crate::mylib::data_structure::queue0::mod_queue::Queue;

/// 数列 xs の連続部分列で種類数が k 以下になる最大の長さを求める
/// 参考: 【ゆっくり解説】尺取り法と二分探索の「本当の」違い - YouTube https://www.youtube.com/watch?v=omD-yyb730k
fn solve(xs: &[i64], k: usize) -> usize {
    // begin を1ずつ動かす尺取法
    let n = xs.len();
    let mut begin = 0; // [begin, end) の半開区間を考える
    let mut end = 0;
    let mut bag = HashBag::<i64>::new();
    let mut max_len = 0;

    while begin < n {
        // [begin, end) が条件を満たす範囲で end を繰り返し進める
        while end < n {
            // xs[end] を含めることで種類数 が k + 1 になりそうなときは break
            if bag.set_len() == k && bag.contains(&xs[end]) == 0 {
                break;
            }

            // end を進める
            bag.insert(xs[end]);
            end += 1;
        }

        max_len = max_len.max(end - begin);

        if begin == end {
            end += 1; // begin が end を追い抜かなさいように end も進める。
            begin += 1;
        } else {
            // begin を進める
            bag.remove(&xs[begin]);
            begin += 1;
        }
    }
    max_len
}

fn solve2(xs: &[i64], k: usize) -> usize {
    // 条件を満たさないギリギリまで伸ばす尺取法
    let n = xs.len();
    let mut end = 0;
    let mut bag = HashBag::<i64>::new();
    let mut max_len = 0;
    for begin in 0..n {
        while bag.set_len() <= k && end <= n {
            if end != n {
                bag.insert(xs[end]);
            }
            end += 1;
        }
        // end = min {end | xs[begin..end] の種類数が k より大きい} になっている（min ∅ は n + 1扱い)
        // ↓
        // end - 1 = max {end | xs[begin..end] の種類数が k 以下} になっている。
        // begin を固定したとき、[begin, end - 1) が種類数 k 以下になる最長区間
        max_len = max_len.max((end - 1) - begin);
        bag.remove(&xs[begin]);
    }

    max_len
}

fn solve3(xs: &[i64], k: usize) -> usize {
    // end を 1 ずつ動かす尺取法
    let n = xs.len();
    let mut begin = 0;
    let mut bag = HashBag::<i64>::new();
    let mut max_len = 0;
    for end in 1..=n {
        bag.insert(xs[end - 1]);
        // 区間 [begin, end) が条件を満たすようになるまで begin を進める
        while bag.set_len() > k {
            bag.remove(&xs[begin]);
            begin += 1;
        }
        max_len = max_len.max(end - begin);
    }

    max_len
}

fn solve_queue(xs: &[i64], k: usize) -> usize {
    // Queue を使った尺取法
    let mut bag = HashBag::<i64>::new();
    let mut max_len = 0;

    let mut queue = Queue::new();

    for &x in xs {
        queue.push(x);
        bag.insert(x);

        // 条件を満たすようになるまで pop する
        while bag.set_len() > k {
            let removed = queue.pop().unwrap();
            bag.remove(&removed);
        }

        max_len = max_len.max(queue.len());
    }
    max_len
}

fn solve_naive(xs: &[i64], k: usize) -> usize {
    let n = xs.len();
    (0..n)
        .flat_map(|begin| (begin..=n).map(move |end| (begin, end)))
        .filter(|&(begin, end)| xs[begin..end].iter().unique().count() <= k)
        .map(|(begin, end)| end - begin)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let xs = [1, 2, 3, 4, 5];
        let k = 3;
        assert_eq!(solve(&xs, k), 3);
        assert_eq!(solve2(&xs, k), 3);
        assert_eq!(solve3(&xs, k), 3);
        assert_eq!(solve_queue(&xs, k), 3);
        assert_eq!(solve_naive(&xs, k), 3);

        let xs = [1, 2, 3, 3, 4, 4, 5];
        let k = 3;
        assert_eq!(solve(&xs, k), 5);
        assert_eq!(solve2(&xs, k), 5);
        assert_eq!(solve3(&xs, k), 5);
        assert_eq!(solve_queue(&xs, k), 5);
        assert_eq!(solve_naive(&xs, k), 5);
    }

    #[test]
    fn random_test() {
        use rand::{rngs::SmallRng, *};
        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..100 {
            let n = rng.gen_range(1..6);
            let xs = (0..n).map(|_| rng.gen_range(1..6)).collect_vec();
            let k = rng.gen_range(0..6);
            let naive_ans = solve_naive(&xs, k);
            assert_eq!(solve(&xs, k), naive_ans);
            assert_eq!(solve2(&xs, k), naive_ans);
            assert_eq!(solve3(&xs, k), naive_ans, "n={}, xs={:?}, k={}", n, &xs, k);
            assert_eq!(solve_queue(&xs, k), naive_ans);
        }
    }
}
