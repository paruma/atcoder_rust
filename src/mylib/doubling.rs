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
            if k >= 1 {
                dp[0] = f.to_vec();
            }
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

pub mod doubling_with_value {
    pub struct DoublingWithValue {
        n: usize,
        log: usize,
        dpf: Vec<Vec<usize>>,
        dpg: Vec<Vec<i64>>,
    }

    impl DoublingWithValue {
        /// doubling 前処理の構築をする
        /// k は 合成回数の最大値 (k>=1)
        /// f[x] は x の遷移先
        /// g[x] は x→f[x] の辺重み
        /// [計算量]
        /// n = f.len() としたとき、O(n log k)
        pub fn new(f: &[usize], g: &[i64], k: usize) -> DoublingWithValue {
            let n = f.len();
            // k を2進展開したときの桁数
            let log = (usize::BITS - k.leading_zeros()) as usize;
            // dp[i][x] = (f の 2^i 回合成)(x)
            let mut dpf = vec![vec![0; n]; log];
            let mut dpg = vec![vec![0; n]; log];
            if k >= 1 {
                dpf[0] = f.to_vec();
                dpg[0] = g.to_vec();
            }
            for i in 1..log {
                for x in 0..n {
                    let f = &dpf[i - 1];
                    let g = &dpg[i - 1];
                    // x → f[x] → f[f[x]]
                    dpg[i][x] = g[x] + g[f[x]];
                    dpf[i][x] = f[f[x]];
                }
            }

            DoublingWithValue { n, log, dpf, dpg }
        }

        /// fのk回合成を f^k とする。
        /// (f^k)(x) と x → f(x) → ... → (f^k)(x) のパス重みを求める
        ///
        /// 計算量: O(log k)
        pub fn eval(&self, k: usize, x: usize) -> (usize, i64) {
            assert!((0..self.n).contains(&x));
            assert!(k < (1 << self.log));

            if k == 0 {
                return (x, 0);
            }

            self.dpf
                .iter()
                .zip(self.dpg.iter())
                .enumerate()
                .filter(|(i, _)| (k >> i) & 1 == 1)
                .map(|(_, (f, g))| (f, g))
                .fold((x, 0), |(idx, val), (f, g)| (f[idx], val + g[idx]))
        }
    }
}
#[cfg(test)]
mod test {
    use super::doubling::Doubling;
    use super::doubling_with_value::DoublingWithValue;

    /// (f の k 回合成)(x) を愚直に計算する
    fn naive(f: &[usize], k: usize, x: usize) -> usize {
        (0..k).fold(x, |acc, _| f[acc])
    }

    /// (f の k 回合成)(x) とそのときの値の総和を愚直に計算する
    fn naive_with_value(f: &[usize], g: &[i64], k: usize, x: usize) -> (usize, i64) {
        (0..k).fold((x, 0), |(current_x, total_value), _| {
            (f[current_x], total_value + g[current_x])
        })
    }

    #[test]
    fn test_doubling() {
        // 0
        // ↓ ↖
        // 1 → 2
        let f = vec![1, 2, 0];

        // 網羅的にテスト
        for max_k in 0..10 {
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

    #[test]
    fn test_doubling_with_value() {
        // 0
        // ↓ ↖
        // 1 → 2
        let f = vec![1, 2, 0];
        let g = vec![10, 100, 1000];

        // 網羅的にテスト
        for max_k in 0..10 {
            let d = DoublingWithValue::new(&f, &g, max_k);
            for k in 0..=max_k {
                for x in 0..f.len() {
                    assert_eq!(d.eval(k, x), naive_with_value(&f, &g, k, x));
                }
            }
        }
    }

    #[test]
    fn test_doubling_with_value_example() {
        // 0
        // ↓ ↖
        // 1 → 2
        let f = vec![1, 2, 0];
        let g = vec![10, 100, 1000];
        let d = DoublingWithValue::new(&f, &g, 4);

        // 通常の例
        // k=2, x=0: 0 -> 1 -> 2, value = g[0] + g[1] = 10 + 100 = 110
        assert_eq!(d.eval(2, 0), (2, 110));
        // k=2, x=1: 1 -> 2 -> 0, value = g[1] + g[2] = 100 + 1000 = 1100
        assert_eq!(d.eval(2, 1), (0, 1100));
        // k=2, x=2: 2 -> 0 -> 1, value = g[2] + g[0] = 1000 + 10 = 1010
        assert_eq!(d.eval(2, 2), (1, 1010));

        // k=4, x=0: 0 -> 1 -> 2 -> 0 -> 1, value = g[0]+g[1]+g[2]+g[0] = 10+100+1000+10 = 1120
        assert_eq!(d.eval(4, 0), (1, 1120));
        // k=4, x=1: 1 -> 2 -> 0 -> 1 -> 2, value = g[1]+g[2]+g[0]+g[1] = 100+1000+10+100 = 1210
        assert_eq!(d.eval(4, 1), (2, 1210));
        // k=4, x=2: 2 -> 0 -> 1 -> 2 -> 0, value = g[2]+g[0]+g[1]+g[2] = 1000+10+100+1000 = 2110
        assert_eq!(d.eval(4, 2), (0, 2110));

        // 0回合成は恒等写像扱い
        assert_eq!(d.eval(0, 0), (0, 0));
        assert_eq!(d.eval(0, 1), (1, 0));
        assert_eq!(d.eval(0, 2), (2, 0));
    }
}
