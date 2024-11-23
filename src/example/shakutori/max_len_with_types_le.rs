#![allow(dead_code)]

use hashbag::HashBag;

use crate::mylib::data_structure::queue0::mod_queue::Queue;

/// 数列 xs の部分列で種類数が k 以下になる最大の長さを求める
/// 参考: 【ゆっくり解説】尺取り法と二分探索の「本当の」違い - YouTube https://www.youtube.com/watch?v=omD-yyb730k
fn solve(xs: &[i64], k: usize) -> usize {
    let n = xs.len();
    let mut begin = 0; // [begin, end) の半開区間を考える
    let mut end = 0;
    let mut bag = HashBag::<i64>::new();
    let mut max_len = 0;

    while begin < n {
        // begin..end が条件を満たす範囲で end を繰り返し進める
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

fn solve_deque(xs: &[i64], k: usize) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let xs = [1, 2, 3, 4, 5];
        let k = 3;
        assert_eq!(solve(&xs, k), 3);
        assert_eq!(solve_deque(&xs, k), 3);

        let xs = [1, 2, 3, 3, 4, 4, 5];
        let k = 3;
        assert_eq!(solve(&xs, k), 5);
        assert_eq!(solve_deque(&xs, k), 5);
    }
}
