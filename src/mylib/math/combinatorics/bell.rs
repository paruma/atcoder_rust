use cargo_snippet::snippet;

#[snippet(prefix = "use bell::*;")]
#[allow(clippy::module_inception)]
pub mod bell {
    use ac_library::modint::ModIntBase;

    /// ベル数を前計算する構造体。
    #[derive(Clone, Debug)]
    pub struct Bell<Mint: ModIntBase> {
        table: Vec<Vec<Mint>>,
    }
    impl<Mint: ModIntBase> Bell<Mint> {
        /// ベル数を最大値 `max_n` まで前計算する。
        ///
        /// 計算量: $O(N^2)$
        pub fn new(max_n: usize) -> Self {
            let mut stirling = vec![vec![Mint::new(0); max_n + 1]; max_n + 1];
            let mut table = vec![vec![Mint::new(0); max_n + 1]; max_n + 1];
            stirling[0][0] = Mint::new(1);
            for n in 1..=max_n {
                for k in 1..=n {
                    stirling[n][k] = stirling[n - 1][k - 1] + Mint::new(k) * stirling[n - 1][k];
                }
            }
            for n in 0..=max_n {
                for k in 0..=max_n {
                    table[n][k] = if k == 0 {
                        stirling[n][0]
                    } else {
                        table[n][k - 1] + stirling[n][k]
                    };
                }
            }
            Self { table }
        }
        /// $n$ 個の要素を高々 $k$ グループへ分割する方法の数を返す。
        pub fn bell(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.table.len(),
                "n out of range (n={}, max_n={})",
                n,
                self.table.len() - 1
            );
            self.table[n][k.min(n)]
        }
        /// $n$ 個の要素をグループへ分割する方法の総数を返す。
        pub fn bell1(&self, n: usize) -> Mint {
            self.bell(n, n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bell::Bell;
    use ac_library::ModInt998244353 as Mint;
    #[test]
    fn test_bell() {
        let bell = Bell::<Mint>::new(4);
        assert_eq!(bell.bell(0, 0), Mint::new(1));
        assert_eq!(bell.bell(3, 2), Mint::new(4));
        assert_eq!(bell.bell1(4), Mint::new(15));
        assert_eq!(bell.bell(3, 10), Mint::new(5));
    }
    #[test]
    #[should_panic(expected = "n out of range (n=5, max_n=4)")]
    fn test_bell_out_of_range() {
        Bell::<Mint>::new(4).bell(5, 0);
    }
}
