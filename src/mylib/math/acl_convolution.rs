use cargo_snippet::snippet;

#[snippet(prefix = "use acl_convolution::*;")]
#[allow(clippy::module_inception)]
pub mod acl_convolution {
    use ac_library::{Modulus, StaticModInt, convolution};

    /// 複数の数列を分割統治で畳み込む。
    ///
    /// 与えられた数列が0個のときは `[1]` を返す。
    ///
    /// 計算量: $O(N \log N \log K)$。$N$ は全数列の要素数の総和、$K$ は数列数である。
    pub fn convolution_all<M: Modulus>(xss: &[Vec<StaticModInt<M>>]) -> Vec<StaticModInt<M>> {
        match xss {
            [] => vec![StaticModInt::new(1)],
            [sequence] => sequence.clone(),
            _ => {
                let mid = xss.len() / 2;
                let left = convolution_all(&xss[..mid]);
                let right = convolution_all(&xss[mid..]);
                convolution(&left, &right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ac_library::{Mod998244353, StaticModInt};

    use super::acl_convolution::convolution_all;

    type Mint = StaticModInt<Mod998244353>;

    #[test]
    fn test_convolution_all() {
        assert_eq!(convolution_all::<Mod998244353>(&[]), vec![Mint::new(1)]);

        assert_eq!(
            convolution_all(&[vec![Mint::new(2), Mint::new(3), Mint::new(5)]]),
            vec![Mint::new(2), Mint::new(3), Mint::new(5)],
        );

        assert_eq!(
            convolution_all(&[
                vec![Mint::new(1), Mint::new(1)],
                vec![Mint::new(2), Mint::new(1)],
                vec![Mint::new(3), Mint::new(1)],
                vec![Mint::new(4), Mint::new(1)],
                vec![Mint::new(5), Mint::new(1)],
            ]),
            vec![
                Mint::new(120),
                Mint::new(274),
                Mint::new(225),
                Mint::new(85),
                Mint::new(15),
                Mint::new(1),
            ],
        );

        assert!(convolution_all(&[vec![Mint::new(1)], vec![]]).is_empty());
    }
}
