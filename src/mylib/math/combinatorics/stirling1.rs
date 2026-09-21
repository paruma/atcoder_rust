use cargo_snippet::snippet;

#[snippet(prefix = "use stirling1::*;")]
#[allow(clippy::module_inception)]
pub mod stirling1 {
    use ac_library::{Modulus, StaticModInt, convolution, modint::ModIntBase};

    // $f(x+c)$ は階乗で重み付けした係数列の畳み込みとして計算できる。
    fn taylor_shift<M: Modulus>(f: &[StaticModInt<M>], c: StaticModInt<M>) -> Vec<StaticModInt<M>> {
        let n = f.len();
        let mut fac = vec![StaticModInt::new(1); n];
        for i in 1..n {
            fac[i] = fac[i - 1] * i;
        }
        let mut invfac = vec![StaticModInt::new(0); n];
        invfac[n - 1] = fac[n - 1].inv();
        for i in (1..n).rev() {
            invfac[i - 1] = invfac[i] * i;
        }
        let left = f
            .iter()
            .zip(&fac)
            .map(|(&x, &factorial)| x * factorial)
            .rev()
            .collect::<Vec<_>>();
        let mut c_pow = StaticModInt::new(1);
        let right = invfac
            .iter()
            .map(|&x| {
                let result = c_pow * x;
                c_pow *= c;
                result
            })
            .collect::<Vec<_>>();
        let product = convolution(&left, &right);
        (0..n).map(|i| product[n - 1 - i] * invfac[i]).collect()
    }

    // 一次式との積は畳み込みより線形時間で更新できる。
    fn multiply_by_x_plus<M: Modulus>(
        mut f: Vec<StaticModInt<M>>,
        c: StaticModInt<M>,
    ) -> Vec<StaticModInt<M>> {
        f.push(StaticModInt::new(0));
        for i in (0..f.len() - 1).rev() {
            let coefficient = f[i];
            f[i + 1] += coefficient;
            f[i] = coefficient * c;
        }
        f
    }

    /// 固定した $n$ に対する符号なし第一種スターリング数を計算する。
    ///
    /// 計算量: $O(N \log N)$
    pub fn stirling1_row<M: Modulus>(n: usize) -> Vec<StaticModInt<M>> {
        if n == 0 {
            return vec![StaticModInt::new(1)];
        }
        let m = n / 2;
        let left = stirling1_row::<M>(m);
        let right = taylor_shift(&left, StaticModInt::new(m));
        let result = convolution(&left, &right);
        if n.is_multiple_of(2) {
            result
        } else {
            // x^(overline(2m+1)) = x^(overline(m)) (x+m)^(overline(m)) (x+2m)
            multiply_by_x_plus(result, StaticModInt::new(2 * m))
        }
    }

    /// $(S_1(n,k))_{0\le n\le n_max, 0\le k\le k_max}$ を計算する。
    ///
    /// 計算量: $O(NK)$
    pub fn stirling1_table<Mint: ModIntBase>(n_max: usize, k_max: usize) -> Vec<Vec<Mint>> {
        let mut table = vec![vec![Mint::new(0); k_max + 1]; n_max + 1];
        table[0][0] = Mint::new(1);
        for n in 1..=n_max {
            for k in 1..=k_max.min(n) {
                table[n][k] = table[n - 1][k - 1] + Mint::new(n - 1) * table[n - 1][k];
            }
        }
        table
    }
}

#[cfg(test)]
mod tests {
    use super::stirling1::{stirling1_row, stirling1_table};
    use ac_library::{Mod998244353, StaticModInt};
    type Mint = StaticModInt<Mod998244353>;

    #[test]
    fn test_stirling1_row_and_table() {
        let expected = vec![
            vec![1],
            vec![0, 1],
            vec![0, 1, 1],
            vec![0, 2, 3, 1],
            vec![0, 6, 11, 6, 1],
        ];
        let table = stirling1_table::<Mint>(4, 4);
        for (n, row) in expected.into_iter().enumerate() {
            let expected = row.into_iter().map(Mint::new).collect::<Vec<_>>();
            assert_eq!(stirling1_row::<Mod998244353>(n), expected);
            assert_eq!(&table[n][..=n], expected);
        }
        assert_eq!(table[2][3], Mint::new(0));
    }
}
