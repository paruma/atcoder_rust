use cargo_snippet::snippet;

#[snippet(prefix = "use mod_stirling::*;")]
pub mod mod_stirling {
    use ac_library::modint::ModIntBase;

    /// 第二種スターリング数、全射、ベル数を計算する構造体
    #[derive(Clone, Debug)]
    pub struct Stirling<Mint: ModIntBase> {
        stirling: Vec<Vec<Mint>>,
        bell: Vec<Vec<Mint>>,
        factorial: Vec<Mint>,
    }

    impl<Mint: ModIntBase> Stirling<Mint> {
        /// 第二種スターリング数、ベル数、全射の数を計算するための前処理計算を最大値 `max_n` まで行う。
        ///
        /// 計算量: `O(max_n^2)`
        pub fn new(max_n: usize) -> Self {
            let mut stirling = vec![vec![Mint::new(0); max_n + 1]; max_n + 1];
            let mut bell = vec![vec![Mint::new(0); max_n + 1]; max_n + 1];

            // 第二種スターリング数: S(n, k) = k * S(n-1, k) + S(n-1, k-1)
            stirling[0][0] = Mint::new(1);
            for n in 1..=max_n {
                for k in 1..=n {
                    stirling[n][k] = stirling[n - 1][k] * Mint::new(k) + stirling[n - 1][k - 1];
                }
            }

            // ベル数（2変数）: B(n, k) = sum of S(n, i) for i = 0 to k
            for n in 0..=max_n {
                for k in 0..=max_n {
                    if k == 0 {
                        bell[n][k] = stirling[n][0];
                    } else {
                        bell[n][k] = bell[n][k - 1] + stirling[n][k];
                    }
                }
            }

            // 階乗の前計算
            let mut factorial = vec![Mint::new(0); max_n + 1];
            factorial[0] = Mint::new(1);
            for i in 1..=max_n {
                factorial[i] = factorial[i - 1] * Mint::new(i);
            }

            Self {
                stirling,
                bell,
                factorial,
            }
        }

        /// 第二種スターリング数 S(n, k)
        ///
        /// n個の要素をちょうどk個の非空グループに分割する数
        pub fn stirling_s2(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.stirling.len() && k < self.stirling[0].len(),
                "index out of range (n={}, k={}, max_n={})",
                n,
                k,
                self.stirling.len() - 1
            );
            self.stirling[n][k]
        }

        /// n個の要素からk個の要素への全射の数
        pub fn surjections(&self, n: usize, k: usize) -> Mint {
            assert!(
                k < self.factorial.len(),
                "k out of range (k={}, max_k={})",
                k,
                self.factorial.len() - 1
            );
            // surjections(n, k) = k! * S(n, k)
            self.stirling_s2(n, k) * self.factorial[k]
        }

        /// ベル数（2変数） B(n, k)
        ///
        /// n個の要素をk個以下のグループに分割する数
        pub fn bell(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.bell.len(),
                "n out of range (n={}, max_n={})",
                n,
                self.bell.len() - 1
            );
            // k >= n のとき B(n, k) = B(n) なので k を n にクランプ
            self.bell[n][k.min(n)]
        }

        /// ベル数（1変数） B(n)
        ///
        /// n個の要素をグループに分割する総数
        pub fn bell1(&self, n: usize) -> Mint {
            assert!(
                n < self.bell.len(),
                "n out of range (n={}, max_n={})",
                n,
                self.bell.len() - 1
            );
            self.bell[n][n]
        }
    }
}

#[snippet(prefix = "use mod_partition::*;")]
pub mod mod_partition {
    use ac_library::modint::ModIntBase;

    /// 分割数を計算する構造体
    #[derive(Clone, Debug)]
    pub struct Partition<Mint: ModIntBase> {
        table: Vec<Vec<Mint>>,
    }

    impl<Mint: ModIntBase> Partition<Mint> {
        /// 分割数を計算するための前処理計算を最大値 `max_n` まで行う。
        ///
        /// 計算量: `O(max_n^2)`
        pub fn new(max_n: usize) -> Self {
            let mut table = vec![vec![Mint::new(0); max_n + 1]; max_n + 1];

            // 分割数: P(n, k) = P(n, k-1) + P(n-k, k)
            // ベースケース: P(0, k) = 1 (全ての k で空の和が 1 通り)
            for k in 0..=max_n {
                table[0][k] = Mint::new(1);
            }

            for n in 1..=max_n {
                for k in 1..=max_n {
                    table[n][k] = table[n][k - 1];
                    if n >= k {
                        table[n][k] = table[n][k] + table[n - k][k];
                    }
                }
            }

            Self { table }
        }

        /// 分割数 P(n, k)
        ///
        /// n を k 個以下の正整数の和として表す方法の数。
        /// n を k 以下の正整数の和として表す方法の数とも等しい。
        pub fn partition(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.table.len(),
                "n out of range (n={}, max_n={})",
                n,
                self.table.len() - 1
            );
            // k >= n のとき P(n, k) = P(n, n) なので k を n にクランプ
            self.table[n][k.min(n)]
        }

        /// 分割数（1変数） P(n)
        ///
        /// n の整数分割の総数
        pub fn partition1(&self, n: usize) -> Mint {
            assert!(
                n < self.table.len(),
                "n out of range (n={}, max_n={})",
                n,
                self.table.len() - 1
            );
            self.table[n][n]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::ModInt998244353 as Mint;

    /// 全射の数 (n=k=0 のとき 1、n=0 かつ k>0 のとき 0)
    fn brute_surjections(n: usize, k: usize) -> u64 {
        if n == 0 && k == 0 {
            return 1;
        }
        if n == 0 || k == 0 {
            return 0;
        }
        // {0..n-1} -> {0..k-1} の全関数を列挙し、全射のものを数える
        (0..k.pow(n as u32))
            .filter(|&mask| {
                let mut m = mask;
                let mut used = vec![false; k];
                for _ in 0..n {
                    used[m % k] = true;
                    m /= k;
                }
                used.iter().all(|&u| u)
            })
            .count() as u64
    }

    /// 第二種スターリング数 S(n, k)
    ///
    /// surjections(n, k) はラベル付きグループへの全射の数なので、
    /// k! で割ってグループの順序を除去することで S(n, k) を得る。
    fn brute_stirling_s2(n: usize, k: usize) -> u64 {
        if n == 0 && k == 0 {
            return 1;
        }
        if n == 0 || k == 0 {
            return 0;
        }
        let factorial_k: u64 = (1..=k as u64).product();
        brute_surjections(n, k) / factorial_k
    }

    /// ベル数（2変数） B(n, k)
    ///
    /// B(n, k) = sum_{i=0}^{k} S(n, i) として計算する
    fn brute_bell(n: usize, k: usize) -> u64 {
        (0..=k).map(|i| brute_stirling_s2(n, i)).sum()
    }

    /// 分割数 P(n, k): n を k 個以下の正整数の和として表す方法の数
    fn brute_partition(n: usize, k: usize) -> u64 {
        fn count(remaining: usize, terms_left: usize, min_val: usize) -> u64 {
            if remaining == 0 {
                return 1;
            }
            if terms_left == 0 {
                return 0;
            }
            (min_val..=remaining)
                .map(|val| count(remaining - val, terms_left - 1, val))
                .sum()
        }
        count(n, k, 1)
    }

    #[test]
    fn test_stirling_s2_exhaustive() {
        let max_n = 7;
        let st = mod_stirling::Stirling::<Mint>::new(max_n);
        for n in 0..=max_n {
            for k in 0..=max_n {
                let expected = brute_stirling_s2(n, k);
                let actual = st.stirling_s2(n, k).val() as u64;
                assert_eq!(actual, expected, "stirling_s2({n}, {k})");
            }
        }
    }

    #[test]
    fn test_surjections_exhaustive() {
        let max_n = 7;
        let st = mod_stirling::Stirling::<Mint>::new(max_n);
        for n in 0..=max_n {
            for k in 0..=max_n {
                let expected = brute_surjections(n, k);
                let actual = st.surjections(n, k).val() as u64;
                assert_eq!(actual, expected, "surjections({n}, {k})");
            }
        }
    }

    #[test]
    fn test_bell_exhaustive() {
        let max_n = 7;
        let st = mod_stirling::Stirling::<Mint>::new(max_n);
        for n in 0..=max_n {
            for k in 0..=max_n {
                let expected = brute_bell(n, k);
                let actual = st.bell(n, k).val() as u64;
                assert_eq!(actual, expected, "bell({n}, {k})");
            }
        }
    }

    #[test]
    fn test_bell1_exhaustive() {
        let max_n = 7;
        let st = mod_stirling::Stirling::<Mint>::new(max_n);
        for n in 0..=max_n {
            let expected = brute_bell(n, n);
            let actual = st.bell1(n).val() as u64;
            assert_eq!(actual, expected, "bell1({n})");
        }
    }

    #[test]
    fn test_partition_exhaustive() {
        let max_n = 10;
        let p = mod_partition::Partition::<Mint>::new(max_n);
        for n in 0..=max_n {
            for k in 0..=max_n {
                let expected = brute_partition(n, k);
                let actual = p.partition(n, k).val() as u64;
                assert_eq!(actual, expected, "partition({n}, {k})");
            }
        }
    }

    #[test]
    fn test_partition1_exhaustive() {
        let max_n = 10;
        let p = mod_partition::Partition::<Mint>::new(max_n);
        for n in 0..=max_n {
            let expected = brute_partition(n, n);
            let actual = p.partition1(n).val() as u64;
            assert_eq!(actual, expected, "partition1({n})");
        }
    }

    #[test]
    fn test_stirling_s2() {
        let st = mod_stirling::Stirling::<Mint>::new(5);

        // S(0, 0) = 1
        assert_eq!(st.stirling_s2(0, 0), Mint::new(1));

        // S(3, 2) = 3
        assert_eq!(st.stirling_s2(3, 2), Mint::new(3));

        // S(4, 2) = 7
        assert_eq!(st.stirling_s2(4, 2), Mint::new(7));

        // S(n, 0) = 0 for n > 0
        assert_eq!(st.stirling_s2(3, 0), Mint::new(0));

        // S(n, k) = 0 for n < k
        assert_eq!(st.stirling_s2(2, 3), Mint::new(0));
    }

    #[test]
    #[should_panic(expected = "index out of range (n=10, k=5, max_n=3)")]
    fn test_stirling_s2_out_of_bounds() {
        let st = mod_stirling::Stirling::<Mint>::new(3);
        st.stirling_s2(10, 5);
    }

    #[test]
    fn test_surjections() {
        let st = mod_stirling::Stirling::<Mint>::new(5);

        // surjections(0, 0) = 0! * S(0, 0) = 1 * 1 = 1
        assert_eq!(st.surjections(0, 0), Mint::new(1));

        // surjections(3, 3) = 3! * S(3, 3) = 6 * 1 = 6 (全単射)
        assert_eq!(st.surjections(3, 3), Mint::new(6));

        // surjections(3, 2) = 2! * S(3, 2) = 2 * 3 = 6
        assert_eq!(st.surjections(3, 2), Mint::new(6));

        // surjections(n, 0) = 0 for n > 0
        assert_eq!(st.surjections(3, 0), Mint::new(0));
    }

    #[test]
    fn test_bell() {
        let st = mod_stirling::Stirling::<Mint>::new(5);

        // B(3, 2) = S(3, 0) + S(3, 1) + S(3, 2) = 0 + 1 + 3 = 4
        assert_eq!(st.bell(3, 2), Mint::new(4));

        // B(3, 3) = B(3) = 0 + 1 + 3 + 1 = 5
        assert_eq!(st.bell1(3), Mint::new(5));

        // B(4) = 15
        assert_eq!(st.bell1(4), Mint::new(15));

        // k > n のとき B(n, k) = B(n)
        assert_eq!(st.bell(3, 5), Mint::new(5));
    }

    #[test]
    fn test_stirling_max_n_0() {
        let st = mod_stirling::Stirling::<Mint>::new(0);

        // S(0, 0) = 1
        assert_eq!(st.stirling_s2(0, 0), Mint::new(1));

        // B(0, 0) = 1
        assert_eq!(st.bell(0, 0), Mint::new(1));

        // B(0) = 1
        assert_eq!(st.bell1(0), Mint::new(1));

        // surjections(0, 0) = 1
        assert_eq!(st.surjections(0, 0), Mint::new(1));
    }

    #[test]
    fn test_partition_max_n_0() {
        let p = mod_partition::Partition::<Mint>::new(0);

        // P(0, 0) = 1
        assert_eq!(p.partition(0, 0), Mint::new(1));

        // P(0) = 1
        assert_eq!(p.partition1(0), Mint::new(1));
    }

    #[test]
    fn test_partition() {
        let p = mod_partition::Partition::<Mint>::new(6);

        // P(5, 3) = 5: 5, 4+1, 3+2, 3+1+1, 2+2+1
        assert_eq!(p.partition(5, 3), Mint::new(5));

        // P(5, 5) = P(5) = 7: 5, 4+1, 3+2, 3+1+1, 2+2+1, 2+1+1+1, 1+1+1+1+1
        assert_eq!(p.partition1(5), Mint::new(7));

        // P(0, k) = 1 for all k
        assert_eq!(p.partition(0, 0), Mint::new(1));
        assert_eq!(p.partition(0, 5), Mint::new(1));

        // P(n, 0) = 0 for n > 0
        assert_eq!(p.partition(5, 0), Mint::new(0));
    }

    #[test]
    fn test_bell_k_clamped() {
        // k >= n のとき B(n, k) = B(n) となるクランプの確認
        let st = mod_stirling::Stirling::<Mint>::new(5);
        // B(3, 10) = B(3) = 5  (k=10 > max_n=5 でもクランプして返る)
        assert_eq!(st.bell(3, 10), Mint::new(5));
    }

    #[test]
    #[should_panic(expected = "n out of range (n=10, max_n=3)")]
    fn test_bell_out_of_bounds() {
        let st = mod_stirling::Stirling::<Mint>::new(3);
        st.bell(10, 5);
    }

    #[test]
    #[should_panic(expected = "n out of range (n=10, max_n=3)")]
    fn test_bell1_out_of_bounds() {
        let st = mod_stirling::Stirling::<Mint>::new(3);
        st.bell1(10);
    }

    #[test]
    #[should_panic(expected = "k out of range (k=10, max_k=3)")]
    fn test_surjections_out_of_bounds() {
        let st = mod_stirling::Stirling::<Mint>::new(3);
        st.surjections(2, 10);
    }

    #[test]
    fn test_partition_k_clamped() {
        // k >= n のとき P(n, k) = P(n) となるクランプの確認
        let p = mod_partition::Partition::<Mint>::new(5);
        // P(3, 10) = P(3) = 3  (k=10 > max_n=5 でもクランプして返る)
        assert_eq!(p.partition(3, 10), Mint::new(3));
    }

    #[test]
    #[should_panic(expected = "n out of range (n=10, max_n=3)")]
    fn test_partition_out_of_bounds() {
        let p = mod_partition::Partition::<Mint>::new(3);
        p.partition(10, 5);
    }

    #[test]
    #[should_panic(expected = "n out of range (n=10, max_n=3)")]
    fn test_partition1_out_of_bounds() {
        let p = mod_partition::Partition::<Mint>::new(3);
        p.partition1(10);
    }
}
