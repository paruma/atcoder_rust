use cargo_snippet::snippet;

#[snippet(prefix = "use partition::*;")]
#[allow(clippy::module_inception)]
pub mod partition {
    use ac_library::modint::ModIntBase;
    /// 分割数を計算する構造体。
    #[derive(Clone, Debug)]
    pub struct Partition<Mint: ModIntBase> {
        table: Vec<Vec<Mint>>,
    }
    impl<Mint: ModIntBase> Partition<Mint> {
        /// 分割数を最大値 `max_n` まで前計算する。
        ///
        /// 計算量: $O(N^2)$
        pub fn new(max_n: usize) -> Self {
            let mut table = vec![vec![Mint::new(0); max_n + 1]; max_n + 1];
            for k in 0..=max_n {
                table[0][k] = Mint::new(1);
            }
            for n in 1..=max_n {
                for k in 1..=max_n {
                    let previous = table[n][k - 1];
                    let using_k = if n >= k {
                        table[n - k][k]
                    } else {
                        Mint::new(0)
                    };
                    table[n][k] = previous + using_k;
                }
            }
            Self { table }
        }
        /// $n$ を高々 $k$ 個の正整数の和として表す方法の数を返す。
        pub fn partition(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.table.len(),
                "n out of range (n={}, max_n={})",
                n,
                self.table.len() - 1
            );
            self.table[n][k.min(n)]
        }
        /// $n$ の整数分割の総数を返す。
        pub fn partition1(&self, n: usize) -> Mint {
            self.partition(n, n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::partition::Partition;
    use ac_library::ModInt998244353 as Mint;
    #[test]
    fn test_partition() {
        let p = Partition::<Mint>::new(5);
        assert_eq!(p.partition(0, 0), Mint::new(1));
        assert_eq!(p.partition(5, 3), Mint::new(5));
        assert_eq!(p.partition1(5), Mint::new(7));
        assert_eq!(p.partition(3, 10), Mint::new(3));
    }
    #[test]
    #[should_panic(expected = "n out of range (n=4, max_n=3)")]
    fn test_partition_out_of_range() {
        Partition::<Mint>::new(3).partition1(4);
    }
}
