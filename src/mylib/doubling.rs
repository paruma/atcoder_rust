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
                    let fp = &dp[i - 1];
                    dp[i][x] = fp[fp[x]];
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
            let k_bits = (usize::BITS - k.leading_zeros()) as usize;

            self.dp
                .iter()
                .enumerate()
                .take(k_bits)
                .filter(|(i, _)| (k >> i) & 1 == 1)
                .map(|(_, fp)| fp)
                .fold(x, |acc, fp| fp[acc])
        }
    }
}

#[allow(clippy::module_inception)]
#[snippet(prefix = "use doubling_with_sum::*;")]
pub mod doubling_with_sum {
    pub struct DoublingWithSum {
        n: usize,
        log: usize,
        dp_f: Vec<Vec<usize>>,
        dp_g: Vec<Vec<i64>>,
    }

    impl DoublingWithSum {
        /// doubling 前処理の構築をする
        /// k は 合成回数の最大値 (k>=1)
        /// f[x] は x の遷移先
        /// g[x] は x→f[x] の辺重み
        /// [計算量]
        /// n = f.len() としたとき、O(n log k)
        pub fn new(f: &[usize], g: &[i64], k: usize) -> DoublingWithSum {
            let n = f.len();
            // k を2進展開したときの桁数
            let log = (usize::BITS - k.leading_zeros()) as usize;
            // dp[i][x] = (f の 2^i 回合成)(x)
            let mut dp_f = vec![vec![0; n]; log];
            let mut dp_g = vec![vec![0; n]; log];
            if k >= 1 {
                dp_f[0] = f.to_vec();
                dp_g[0] = g.to_vec();
            }
            for i in 1..log {
                for x in 0..n {
                    let fp = &dp_f[i - 1];
                    let gp = &dp_g[i - 1];
                    // x → fp[x] → fp[fp[x]]
                    dp_g[i][x] = gp[x] + gp[fp[x]];
                    dp_f[i][x] = fp[fp[x]];
                }
            }

            DoublingWithSum { n, log, dp_f, dp_g }
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
            let k_bits = (usize::BITS - k.leading_zeros()) as usize;

            self.dp_f
                .iter()
                .zip(self.dp_g.iter())
                .enumerate()
                .take(k_bits)
                .filter(|(i, _)| (k >> i) & 1 == 1)
                .map(|(_, (fp, gp))| (fp, gp))
                .fold((x, 0), |(idx, val), (fp, gp)| (fp[idx], val + gp[idx]))
        }
    }
}

#[allow(clippy::module_inception)]
#[snippet(prefix = "use doubling_with_monoid::*;")]
pub mod doubling_with_monoid {
    use ac_library::Monoid;


    pub struct DoublingWithMonoid<M: Monoid> {
        n: usize,
        log: usize,
        dp_f: Vec<Vec<usize>>,
        dp_g: Vec<Vec<M::S>>,
    }

    impl<M: Monoid> DoublingWithMonoid<M>
    where
        M::S: Clone,
    {
        /// doubling 前処理の構築をする
        /// k は 合成回数の最大値 (k>=1)
        /// f[x] は x の遷移先
        /// g[x] は x→f[x] の辺に対応するモノイドの値
        /// [計算量]
        /// n = f.len() としたとき、O(n log k)
        pub fn new(f: &[usize], g: &[M::S], k: usize) -> Self {
            let n = f.len();
            let log = (usize::BITS - k.leading_zeros()) as usize;
            let mut dp_f = vec![vec![0; n]; log];
            let mut dp_g = vec![vec![M::identity(); n]; log];

            if k >= 1 {
                dp_f[0] = f.to_vec();
                dp_g[0] = g.to_vec();
            }

            for i in 1..log {
                for x in 0..n {
                    let fp = &dp_f[i - 1];
                    let gp = &dp_g[i - 1];
                    dp_g[i][x] = M::binary_operation(&gp[x], &gp[fp[x]]);
                    dp_f[i][x] = fp[fp[x]];
                }
            }

            Self {
                n,
                log,
                dp_f,
                dp_g,
            }
        }

        /// fのk回合成を f^k とする。
        /// (f^k)(x) と x → f(x) → ... → (f^k)(x) のパス上の値の総積（モノイド演算）を求める
        ///
        /// 計算量: O(log k)
        pub fn eval(&self, k: usize, x: usize) -> (usize, M::S) {
            assert!((0..self.n).contains(&x));
            assert!(k < (1 << self.log));

            if k == 0 {
                return (x, M::identity());
            }
            let k_bits = (usize::BITS - k.leading_zeros()) as usize;

            self.dp_f
                .iter()
                .zip(self.dp_g.iter())
                .enumerate()
                .take(k_bits)
                .filter(|(i, _)| (k >> i) & 1 == 1)
                .map(|(_, (fp, gp))| (fp, gp))
                .fold((x, M::identity()), |(idx, val), (fp, gp)| {
                    (fp[idx], M::binary_operation(&val, &gp[idx]))
                })
        }
    }
}

#[cfg(test)]
mod test {
    use super::doubling::Doubling;
    use super::doubling_with_sum::DoublingWithSum;

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
    fn test_doubling_with_value_example() {
        // 0
        // ↓ ↖
        // 1 → 2
        let f = vec![1, 2, 0];
        let g = vec![10, 100, 1000];
        let d = DoublingWithSum::new(&f, &g, 4);

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

    #[test]
    #[ignore]
    fn test_doubling_random() {
        use rand::{rngs::SmallRng, Rng, SeedableRng};
        let mut rng = SmallRng::from_entropy();

        for _ in 0..500 {
            let n = rng.gen_range(1..11);
            let max_k = rng.gen_range(1..21);

            let f = (0..n).map(|_| rng.gen_range(0..n)).collect::<Vec<_>>();

            let d = Doubling::new(&f, max_k);

            for _ in 0..100 {
                let k = rng.gen_range(0..=max_k);
                let x = rng.gen_range(0..n);
                assert_eq!(d.eval(k, x), naive(&f, k, x));
            }
        }
    }

    #[test]
    #[ignore]
    fn test_doubling_with_value_random() {
        use rand::{rngs::SmallRng, Rng, SeedableRng};
        let mut rng = SmallRng::from_entropy();

        for _ in 0..500 {
            let n = rng.gen_range(1..11);
            let max_k = rng.gen_range(1..21);

            let f = (0..n).map(|_| rng.gen_range(0..n)).collect::<Vec<_>>();
            let g = (0..n)
                .map(|_| rng.gen_range(-1_000_000_000..1_000_000_000))
                .collect::<Vec<_>>();

            let d = DoublingWithSum::new(&f, &g, max_k);

            for _ in 0..100 {
                let k = rng.gen_range(0..=max_k);
                let x = rng.gen_range(0..n);
                assert_eq!(d.eval(k, x), naive_with_value(&f, &g, k, x));
            }
        }
    }

    use super::doubling_with_monoid::DoublingWithMonoid;
    use ac_library::Monoid;

    struct Sum;
    impl Monoid for Sum {
        type S = i64;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    #[test]
    fn test_doubling_with_monoid_sum() {
        let f = vec![1, 2, 0];
        let g = vec![10, 100, 1000];
        let max_k = 10;
        let d_sum = DoublingWithSum::new(&f, &g, max_k);
        let d_monoid = DoublingWithMonoid::<Sum>::new(&f, &g, max_k);

        for k in 0..=max_k {
            for x in 0..f.len() {
                assert_eq!(d_sum.eval(k, x), d_monoid.eval(k, x), "k={}, x={}", k, x);
            }
        }
    }
}