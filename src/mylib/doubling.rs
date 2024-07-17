use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use doubling::*;")]
pub mod doubling {
    pub struct Doubling {
        n: usize,
        log: usize,
        dp: Vec<Vec<usize>>,
    }

    impl Doubling {
        /// doubling 前処理の構築をする
        /// k は 合成回数の最大値 (k>=1)
        /// [計算量]
        /// n = f.len() としたとき、O(n log k)
        pub fn new(f: &[usize], k: usize) -> Doubling {
            let n = f.len();
            // k を2進展開したときの桁数
            let log = (usize::BITS - k.leading_zeros()) as usize;
            // dp[i][x] = (f の 2^i 回合成)(x)
            let mut dp = vec![vec![0; n]; log];
            dp[0] = f.to_vec();
            for i in 1..log {
                for x in 0..n {
                    let f = &dp[i - 1];
                    dp[i][x] = f[f[x]];
                }
            }

            Doubling { n, log, dp }
        }

        /// (f の k回合成)(x) を求める。
        /// 計算量: O(log k)
        pub fn eval(&self, k: usize, x: usize) -> usize {
            assert!((0..self.n).contains(&x));
            assert!(k < (1 << self.log));

            if k == 0 {
                return x;
            }

            self.dp
                .iter()
                .enumerate()
                .filter(|(i, _)| (k >> i) & 1 == 1)
                .map(|(_, f)| f)
                .fold(x, |acc, f| f[acc])
        }
    }
}

#[cfg(test)]
mod test {
    use super::doubling::Doubling;

    /// (f の k 回合成)(x) を愚直に計算する
    fn naive(f: &[usize], k: usize, x: usize) -> usize {
        (0..k).fold(x, |acc, _| f[acc])
    }

    #[test]
    fn test_doubling() {
        // 0
        // ↓ ↖
        // 1 → 2
        let f = vec![1, 2, 0];

        // 網羅的にテスト
        for max_k in 1..10 {
            let d = Doubling::new(&f, max_k);
            for k in 0..=max_k {
                for x in 0..f.len() {
                    assert_eq!(d.eval(k, x), naive(&f, k, x));
                }
            }
        }
    }

    #[test]
    fn test_doubling_example() {
        // 0
        // ↓ ↖
        // 1 → 2
        let f = vec![1, 2, 0];
        let d = Doubling::new(&f, 4);

        // 通常の例
        assert_eq!(d.eval(2, 0), 2);
        assert_eq!(d.eval(2, 1), 0);
        assert_eq!(d.eval(2, 2), 1);

        assert_eq!(d.eval(4, 0), 1);
        assert_eq!(d.eval(4, 1), 2);
        assert_eq!(d.eval(4, 2), 0);

        // 0回合成は恒等写像扱い
        assert_eq!(d.eval(0, 0), 0);
        assert_eq!(d.eval(0, 1), 1);
        assert_eq!(d.eval(0, 2), 2);
    }
}
