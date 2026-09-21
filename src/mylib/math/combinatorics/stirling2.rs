use cargo_snippet::snippet;

#[snippet(prefix = "use stirling2::*;")]
#[allow(clippy::module_inception)]
pub mod stirling2 {
    use ac_library::{Modulus, StaticModInt, convolution, modint::ModIntBase};

    fn factorials<Mint: ModIntBase>(n: usize) -> Vec<Mint> {
        let mut fac = vec![Mint::new(1); n + 1];
        for i in 1..=n {
            fac[i] = fac[i - 1] * Mint::new(i);
        }
        fac
    }

    /// 第二種スターリング数 $S_2(n, k)$ を計算する。
    ///
    /// 計算量: $O(K \log N)$
    pub fn stirling2<Mint: ModIntBase>(n: usize, k: usize) -> Mint {
        if k > n {
            return Mint::new(0);
        }
        let fac = factorials::<Mint>(k);
        let invfac_k = fac[k].inv();
        (0..=k)
            .map(|i| {
                let term = Mint::new(i).pow(n as u64) * fac[k] * fac[i].inv() * fac[k - i].inv();
                if (k - i).is_multiple_of(2) {
                    term
                } else {
                    -term
                }
            })
            .fold(Mint::new(0), |acc, x| acc + x)
            * invfac_k
    }

    /// 固定した $n$ に対する $(S_2(n,k))_{k=0}^{n}$ を計算する。
    ///
    /// 計算量: $O(N \log N)$
    pub fn stirling2_row<M: Modulus>(n: usize) -> Vec<StaticModInt<M>> {
        let fac = factorials::<StaticModInt<M>>(n);
        let invfac = fac.iter().map(|&x| x.inv()).collect::<Vec<_>>();
        let a = (0..=n)
            .map(|i| StaticModInt::new(i).pow(n as u64) * invfac[i])
            .collect::<Vec<_>>();
        let b = (0..=n)
            .map(|i| {
                if i.is_multiple_of(2) {
                    invfac[i]
                } else {
                    -invfac[i]
                }
            })
            .collect::<Vec<_>>();
        let mut result = convolution(&a, &b);
        result.truncate(n + 1);
        result
    }

    /// $(S_2(n,k))_{0\le n\le n_max, 0\le k\le k_max}$ を計算する。
    ///
    /// 計算量: $O(NK)$
    pub fn stirling2_table<Mint: ModIntBase>(n_max: usize, k_max: usize) -> Vec<Vec<Mint>> {
        let mut table = vec![vec![Mint::new(0); k_max + 1]; n_max + 1];
        table[0][0] = Mint::new(1);
        for n in 1..=n_max {
            for k in 1..=k_max.min(n) {
                table[n][k] = table[n - 1][k - 1] + Mint::new(k) * table[n - 1][k];
            }
        }
        table
    }

    /// $n$ 個の要素から $k$ 個の要素への全射の数を計算する。
    ///
    /// 計算量: $O(K \log N)$
    pub fn surj<Mint: ModIntBase>(n: usize, k: usize) -> Mint {
        stirling2::<Mint>(n, k) * factorials::<Mint>(k)[k]
    }

    /// 固定した $n$ に対する全射数を計算する。
    ///
    /// 計算量: $O(N \log N)$
    pub fn surj_row<M: Modulus>(n: usize) -> Vec<StaticModInt<M>> {
        let fac = factorials::<StaticModInt<M>>(n);
        stirling2_row(n)
            .into_iter()
            .enumerate()
            .map(|(k, x)| x * fac[k])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::stirling2::{stirling2, stirling2_row, stirling2_table, surj, surj_row};
    use ac_library::{Mod998244353, ModInt998244353 as Mint};

    #[test]
    fn test_stirling2() {
        let expected = [
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 3, 1, 0],
            [0, 1, 7, 6, 1],
        ];
        let table = stirling2_table::<Mint>(4, 4);
        for (n, row) in expected.into_iter().enumerate() {
            let row = row.into_iter().map(Mint::new).collect::<Vec<_>>();
            assert_eq!(stirling2_row::<Mod998244353>(n), row[..=n]);
            for (k, &value) in row.iter().enumerate() {
                assert_eq!(stirling2::<Mint>(n, k), value);
                assert_eq!(table[n][k], value);
            }
        }
        assert_eq!(stirling2::<Mint>(2, 3), Mint::new(0));
    }

    #[test]
    fn test_surj() {
        assert_eq!(surj::<Mint>(0, 0), Mint::new(1));
        assert_eq!(surj::<Mint>(3, 2), Mint::new(6));
        assert_eq!(surj::<Mint>(2, 3), Mint::new(0));
        assert_eq!(
            surj_row::<Mod998244353>(3),
            vec![Mint::new(0), Mint::new(1), Mint::new(6), Mint::new(6)]
        );
    }
}
