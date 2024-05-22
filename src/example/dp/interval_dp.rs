#![allow(dead_code)]

fn slimes(n: usize, xs: &[i64]) -> i64 {
    // Educational DP Contest N - Slimes https://atcoder.jp/contests/dp/tasks/dp_n
    // [問題]
    // スライムがn匹並んでいて、左からi番目のスライムの大きさはxs[i]で与えられる。
    // 隣り合うスライムを合体させて、一匹のスライムにする。
    // 大きさxのスライムと大きさyのスライムが合体したとき、x+yのスライムになる。このとき、コストx+yが発生する。
    // スライムを一匹にするまでに発生するコストを最小化する。
    //
    // [考え方]
    // 区間 [begin, end) を、[begin, end) の範囲にいたスライムを合体させたものだと考える。
    struct Rec {
        n: usize,
        xs: Vec<i64>,
        xs_cumsum: Vec<i64>,
    }

    impl Rec {
        fn new(n: usize, xs: &[i64]) -> Self {
            let xs = xs.to_vec();
            let mut xs_cumsum = vec![0; xs.len() + 1];
            for i in 0..xs.len() {
                xs_cumsum[i + 1] = xs[i] + xs_cumsum[i]
            }
            Self { n, xs, xs_cumsum }
        }

        // 区間[begin, end) におけるコスト最小値
        fn rec(&self, begin: usize, end: usize, dp: &mut [Vec<Option<i64>>]) -> i64 {
            if let Some(ans) = dp[begin][end] {
                return ans;
            }

            let ans = if end - begin <= 1 {
                // もう一匹になっているので合体は不要→コスト0
                0
            } else {
                /*
                [begin, end) でのコスト最小のスライムが
                2つの区間 [begin, mid), [mid, end) のスライムの合体であるとする。
                各割り方でコストを求めて、その最小値を求めれば良い。
                [begin, mid) と [mid, end) で合体する場合のコストの総和は以下のように求まる。

                [begin, mid) のスライムを作るまでのコスト
                + [mid, end) のスライムを作るまでのコスト
                + [begin, mid) と [mid, end) を合体するコスト

                */
                let cum_cost = (begin + 1..end)
                    .map(|mid| self.rec(begin, mid, dp) + self.rec(mid, end, dp))
                    .min()
                    .unwrap();
                let current_cost = self.xs_cumsum[end] - self.xs_cumsum[begin];
                cum_cost + current_cost
            };
            dp[begin][end] = Some(ans);
            ans
        }
    }
    let mut dp = vec![vec![None; n + 1]; n + 1];
    Rec::new(n, xs).rec(0, n, &mut dp)
}

fn daruma_otoshi(n: usize, xs: &[i64]) -> usize {
    // だるま落とし https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1611&lang=jp
    // [問題]
    // 重さの差が1以下である2つのブロックを同時に取り除くだるま落としを考える。
    // 取り除けるブロックの数の最大値は？
    struct Rec {
        n: usize,
        xs: Vec<i64>,
    }

    impl Rec {
        fn new(n: usize, xs: &[i64]) -> Self {
            let xs = xs.to_vec();
            Self { n, xs }
        }

        fn rec(&self, begin: usize, end: usize, dp: &mut [Vec<Option<usize>>]) -> usize {
            if let Some(ans) = dp[begin][end] {
                return ans;
            }

            let ans = if end - begin <= 1 {
                // ブロックが1つの場合は取り除けない
                0
            } else if self.rec(begin + 1, end - 1, dp) == end - begin - 2
                && i64::abs_diff(self.xs[begin], self.xs[end - 1]) <= 1
            {
                // 最適解が begin のブロックと end - 1 のブロックを同時に除く場合
                // begin, begin + 1, ... end - 2, end - 1
                //        |---------------------|
                //         この区間がすべて取り除ける
                // かつ、begin 番目のブロックと、end -1 番目のブロックの重さの差が1以下
                // という状況になっている
                end - begin
            } else {
                /*
                最適解が begin のブロックと end - 1 のブロックを同時に除かない場合
                ある mid in [begin + 1, end) が存在して、
                最適解で同時に取り除くすべての2ブロックは
                * 両方とも [begin, mid) に入っているか、
                * 両方とも [mid, end) に入っている
                ことが言える

                begin, begin + 1, ... mid - 1, mid, ... end - 2, end - 1
                |----------------------------| |-----------------------|
                [証明]
                * begin が除かれない場合は mid = begin + 1
                    begin, begin + 1, ... end - 2, end - 1
                    |----| |-----------------------------|
                * begin がブロック x と除かれる場合は、mid = x + 1 とすれば良い
                    x < end -1 なので、mid < end が言える。
                    begin, begin + 1, ... x, x + 1 ... end - 2, end - 1
                    |----------------------| |------------------------|
                    ↑ここの部分はすべて除かれる
                */
                (begin + 1..end)
                    .map(|mid| self.rec(begin, mid, dp) + self.rec(mid, end, dp))
                    .max()
                    .unwrap()
            };
            dp[begin][end] = Some(ans);
            ans
        }
    }
    Rec::new(n, xs).rec(0, n, &mut vec![vec![None; n + 1]; n + 1])
}

#[cfg(test)]
mod tests {

    use super::{daruma_otoshi, slimes};

    #[test]
    fn test_slimes() {
        assert_eq!(slimes(4, &[10, 20, 30, 40]), 190);
    }

    #[test]
    fn test_daruma_otoshi() {
        assert_eq!(daruma_otoshi(2, &[1, 2]), 2);
        assert_eq!(daruma_otoshi(2, &[2, 1]), 2);
        assert_eq!(daruma_otoshi(2, &[2, 2]), 2);
        assert_eq!(daruma_otoshi(2, &[2, 8]), 0);
        assert_eq!(daruma_otoshi(2, &[8, 2]), 0);
        assert_eq!(daruma_otoshi(3, &[4, 1, 2]), 2);
        assert_eq!(daruma_otoshi(3, &[1, 4, 2]), 0);
        assert_eq!(daruma_otoshi(4, &[1, 10, 11, 2]), 4);
        assert_eq!(daruma_otoshi(4, &[1, 2, 3, 4]), 4);
        assert_eq!(daruma_otoshi(4, &[1, 2, 3, 1]), 4);
        assert_eq!(daruma_otoshi(5, &[5, 1, 2, 3, 6]), 2);
        assert_eq!(
            daruma_otoshi(14, &[8, 7, 1, 4, 3, 5, 4, 1, 6, 8, 10, 4, 6, 5]),
            12
        );
        assert_eq!(daruma_otoshi(5, &[1, 3, 5, 1, 3]), 0);
    }
}
