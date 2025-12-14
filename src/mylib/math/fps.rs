//! 形式的冪級数ライブラリ
use ac_library::{Modulus, StaticModInt, convolution};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 形式的冪級数を表す構造体。
/// 係数を`Vec<StaticModInt<M>>`で保持する。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormalPowerSeries<M: Modulus> {
    pub coeffs: Vec<StaticModInt<M>>,
}

impl<M: Modulus> FormalPowerSeries<M> {
    /// 新しいFPSを係数ベクトルから作成する。
    pub fn new(coeffs: Vec<StaticModInt<M>>) -> Self {
        Self { coeffs }
    }

    /// `len` - 1 次のゼロ多項式を返す。
    pub fn zero(len: usize) -> Self {
        Self::new(vec![StaticModInt::new(0); len])
    }

    /// 定数1の多項式を返す。
    pub fn one() -> Self {
        Self::new(vec![StaticModInt::new(1)])
    }

    /// 次数を返す (係数ベクトルの長さ)。
    pub fn degree(&self) -> usize {
        self.coeffs.len()
    }

    /// 末尾の0係数を削除して、表現を正規化する。
    pub fn trim(&mut self) {
        while self.coeffs.last().map_or(false, |&c| c.val() == 0) {
            self.coeffs.pop();
        }
    }

    /// `deg` 次までの逆元 `1/f(x)` をニュートン法で計算する。
    /// 計算量: O(deg log deg)
    pub fn inv(&self, deg: usize) -> Self {
        assert!(
            !self.coeffs.is_empty() && self.coeffs[0].val() != 0,
            "定数項が0です"
        );

        let mut g = Self::new(vec![self.coeffs[0].inv()]);
        let mut k = 1;
        while k < deg {
            k *= 2;
            let f_k = Self::new(self.coeffs.iter().take(k).cloned().collect());
            let mut fg = &f_k * &g;
            fg.coeffs.truncate(k);

            for c in &mut fg.coeffs {
                *c = -(*c);
            }
            fg.coeffs[0] += 2;
            g = &g * &fg;
            g.coeffs.truncate(k);
        }
        g.coeffs.truncate(deg);
        g
    }

    /// 指定された次数 `sz` でFPSを切り詰める（足りない場合は0で埋める）。
    pub fn pre(&self, sz: usize) -> Self {
        let mut coeffs = self.coeffs.clone();
        coeffs.truncate(sz);
        coeffs.resize(sz, StaticModInt::new(0));
        Self::new(coeffs)
    }

    /// 導関数 `f'(x)` を計算する。
    pub fn diff(&self) -> Self {
        let n = self.degree();
        if n <= 1 {
            return Self::new(vec![]);
        }
        let mut new_coeffs = vec![StaticModInt::new(0); n - 1];
        for i in 1..n {
            new_coeffs[i - 1] = self.coeffs[i] * i;
        }
        Self::new(new_coeffs)
    }

    /// 不定積分 `∫f(x)dx` を計算する（積分定数は0）。
    pub fn integral(&self) -> Self {
        let n = self.degree();
        let mut new_coeffs = vec![StaticModInt::new(0); n + 1];
        if n > 0 {
            let invs: Vec<StaticModInt<M>> =
                (1..=n).map(|i| StaticModInt::<M>::new(i).inv()).collect();
            for i in 0..n {
                new_coeffs[i + 1] = self.coeffs[i] * invs[i];
            }
        }
        Self::new(new_coeffs)
    }

    /// FPSの対数 `log(f(x))` を計算する。
    /// 前提条件: `f(0) == 1`
    /// 計算量: O(deg log deg)
    pub fn log(&self, deg: usize) -> Self {
        assert!(
            !self.coeffs.is_empty() && self.coeffs[0].val() == 1,
            "log(f(x)) を計算するには f(0) == 1 である必要があります。"
        );

        let df = self.diff(); // f'(x)
        let inv_f = self.inv(deg); // f(x)^-1
        let mut df_inv_f = &df * &inv_f; // f'(x) * f(x)^-1
        df_inv_f.coeffs.truncate(deg); // deg次まで切り詰める

        df_inv_f.integral().pre(deg) // 積分して、指定次数で切り詰める
    }

    /// FPSの指数 `exp(f(x))` を計算する。
    /// 前提条件: `f(0) == 0`
    /// 計算量: O(deg log deg)
    pub fn exp(&self, deg: usize) -> Self {
        assert!(
            self.coeffs.is_empty() || self.coeffs[0].val() == 0,
            "exp(f(x)) を計算するには f(0) == 0 である必要があります。"
        );

        if deg == 0 {
            return Self::new(vec![]);
        }

        let mut g = Self::new(vec![StaticModInt::new(1)]); // g = 1 で初期化
        let mut k = 1;
        while k < deg {
            k *= 2;
            let f_k = self.pre(k); // f(x) を k 次まで切り詰める

            // g = g * (1 - log(g) + f)
            let log_g = g.log(k);
            let val = &f_k - &log_g; // f - log(g)
            let one_plus_val = &Self::one() + &val; // 1 + (f - log(g))
            g = &g * &one_plus_val;
            g.coeffs.truncate(k);
        }
        g.pre(deg) // 指定次数で切り詰める
    }

    /// FPSのべき乗 `f(x)^k` を計算する。
    /// 計算量: O(deg log deg)
    pub fn pow(&self, mut k: i64, deg: usize) -> Self {
        if deg == 0 {
            return Self::new(vec![]);
        }
        if k == 0 {
            let mut ret = Self::new(vec![StaticModInt::new(0); deg]);
            ret.coeffs[0] = StaticModInt::new(1);
            return ret;
        }
        if self.coeffs.is_empty() {
            return Self::new(vec![StaticModInt::new(0); deg]);
        }

        let mut first_nonzero_idx = 0;
        while first_nonzero_idx < self.coeffs.len() && self.coeffs[first_nonzero_idx].val() == 0 {
            first_nonzero_idx += 1;
        }

        if first_nonzero_idx == self.coeffs.len() {
            // 全ての係数が0
            return Self::new(vec![StaticModInt::new(0); deg]);
        }

        if first_nonzero_idx > 0 {
            // f(0) == 0 の場合
            if (first_nonzero_idx as i64) * k >= deg as i64 {
                return Self::new(vec![StaticModInt::new(0); deg]);
            }

            // f(x) = x^first_nonzero_idx * g(x) (g(0) != 0)
            // f(x)^k = x^(first_nonzero_idx * k) * g(x)^k
            let mut g_coeffs = self.coeffs[first_nonzero_idx..].to_vec();
            let mut g = Self::new(g_coeffs);

            // g(x)^k を計算
            let gk = g.pow(k, deg - first_nonzero_idx * k as usize);

            // x^(first_nonzero_idx * k) を掛ける (左シフト)
            let mut result_coeffs = vec![StaticModInt::new(0); first_nonzero_idx * k as usize];
            result_coeffs.extend_from_slice(&gk.coeffs);
            Self::new(result_coeffs).pre(deg)
        } else {
            // f(0) != 0 の場合
            let f0_inv = self.coeffs[0].inv();
            let mut normalized_f = self.coeffs.iter().map(|&c| c * f0_inv).collect::<Vec<_>>();
            let mut normalized_fps = Self::new(normalized_f);

            let log_normalized_f = normalized_fps.log(deg);
            let mut k_fps_coeffs = vec![StaticModInt::new(0); deg];
            if deg > 0 {
                k_fps_coeffs[1] = StaticModInt::new(k);
            }
            let k_fps = Self::new(k_fps_coeffs).pre(deg); // k*x のようなFPSではない。k倍するだけ

            let k_times_log_normalized_f_coeffs =
                log_normalized_f.coeffs.iter().map(|&c| c * k).collect();
            let k_times_log_normalized_f = Self::new(k_times_log_normalized_f_coeffs);

            let res = k_times_log_normalized_f.exp(deg);

            let f0 = self.coeffs[0];
            let f0_pow_k = if k >= 0 {
                f0.pow(k as u64)
            } else {
                f0.inv().pow((-k) as u64)
            };
            let final_coeffs = res.coeffs.iter().map(|&c| c * f0_pow_k).collect();
            Self::new(final_coeffs).pre(deg)
        }
    }
}
// --- 算術演算子 ---

// FPS + FPS
impl<M: Modulus> Add for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn add(self, rhs: Self) -> Self::Output {
        let n = self.degree();
        let m = rhs.degree();
        let max_len = n.max(m);
        let mut coeffs = vec![StaticModInt::new(0); max_len];
        for i in 0..n {
            coeffs[i] += self.coeffs[i];
        }
        for i in 0..m {
            coeffs[i] += rhs.coeffs[i];
        }
        FormalPowerSeries::new(coeffs)
    }
}

impl<M: Modulus> AddAssign for FormalPowerSeries<M> {
    fn add_assign(&mut self, rhs: Self) {
        if self.degree() < rhs.degree() {
            self.coeffs.resize(rhs.degree(), StaticModInt::new(0));
        }
        for i in 0..rhs.degree() {
            self.coeffs[i] += rhs.coeffs[i];
        }
    }
}

// FPS - FPS
impl<M: Modulus> Sub for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn sub(self, rhs: Self) -> Self::Output {
        let n = self.degree();
        let m = rhs.degree();
        let max_len = n.max(m);
        let mut coeffs = vec![StaticModInt::new(0); max_len];
        for i in 0..n {
            coeffs[i] += self.coeffs[i];
        }
        for i in 0..m {
            coeffs[i] -= rhs.coeffs[i];
        }
        FormalPowerSeries::new(coeffs)
    }
}

impl<M: Modulus> SubAssign for FormalPowerSeries<M> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.degree() < rhs.degree() {
            self.coeffs.resize(rhs.degree(), StaticModInt::new(0));
        }
        for i in 0..rhs.degree() {
            self.coeffs[i] -= rhs.coeffs[i];
        }
    }
}

// FPS * FPS
impl<M: Modulus> Mul for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.coeffs.is_empty() || rhs.coeffs.is_empty() {
            return FormalPowerSeries::new(vec![]);
        }
        let coeffs = convolution(&self.coeffs, &rhs.coeffs);
        FormalPowerSeries::new(coeffs)
    }
}

impl<M: Modulus> MulAssign for FormalPowerSeries<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = &*self * &rhs;
    }
}

// 単項マイナス
impl<M: Modulus> Neg for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn neg(self) -> Self::Output {
        let coeffs: Vec<_> = self.coeffs.iter().map(|&c| -c).collect();
        FormalPowerSeries::new(coeffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::Mod998244353;
    use rand::{Rng, rngs::ThreadRng};

    type Mint = StaticModInt<Mod998244353>;
    type Fps = FormalPowerSeries<Mod998244353>;

    #[test]
    fn test_add() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g = Fps::new(vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
        let h = &f + &g;
        assert_eq!(h.coeffs, vec![Mint::new(4), Mint::new(6), Mint::new(5)]);
    }

    #[test]
    fn test_sub() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g = Fps::new(vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
        let h = &f - &g;
        assert_eq!(h.coeffs, vec![Mint::new(-2), Mint::new(-2), Mint::new(-5)]);
    }

    #[test]
    fn test_mul() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g = Fps::new(vec![Mint::new(1), Mint::new(3)]);
        // (1+2x)(1+3x) = 1 + 5x + 6x^2
        let h = &f * &g;
        assert_eq!(h.coeffs, vec![Mint::new(1), Mint::new(5), Mint::new(6)]);
    }

    #[test]
    fn test_inv_geometric_series() {
        // 1 / (1-x) = 1 + x + x^2 + ...
        let f = Fps::new(vec![Mint::new(1), Mint::new(-1)]);
        let f_inv = f.inv(10);
        assert_eq!(f_inv.coeffs, vec![Mint::new(1); 10]);
    }

    #[test]
    #[ignore]
    fn test_inv_random() {
        let mut rng = ThreadRng::default();
        for _ in 0..10000 {
            let deg = rng.random_range(1..=20);
            let mut coeffs = vec![Mint::new(0); deg];
            for i in 0..deg {
                coeffs[i] = Mint::new(rng.random_range(0..Mint::modulus()));
            }
            // 定数項は0でない
            coeffs[0] = Mint::new(rng.random_range(1..Mint::modulus()));

            let f = Fps::new(coeffs);
            let f_inv = f.inv(deg);

            let mut one = &f * &f_inv;
            one.coeffs.truncate(deg);

            let mut expected = vec![Mint::new(0); deg];
            expected[0] = Mint::new(1);

            assert_eq!(one.coeffs, expected, "f * f.inv() should be 1");
        }
    }

    #[test]
    fn test_utils() {
        // zero
        let z = Fps::zero(5);
        assert_eq!(z.coeffs, vec![Mint::new(0); 5]);

        // one
        let o = Fps::one();
        assert_eq!(o.coeffs, vec![Mint::new(1)]);

        // trim
        let mut f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(0), Mint::new(0)]);
        f.trim();
        assert_eq!(f.coeffs, vec![Mint::new(1), Mint::new(2)]);
        let mut g = Fps::new(vec![Mint::new(0), Mint::new(0)]);
        g.trim();
        assert_eq!(g.coeffs, Vec::<Mint>::new());
    }

    #[test]
    fn test_assign_ops() {
        // AddAssign
        let mut f1 = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g1 = Fps::new(vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
        f1.add_assign(g1);
        assert_eq!(f1.coeffs, vec![Mint::new(4), Mint::new(6), Mint::new(5)]);

        // SubAssign
        let mut f2 = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g2 = Fps::new(vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
        f2.sub_assign(g2);
        assert_eq!(f2.coeffs, vec![Mint::new(-2), Mint::new(-2), Mint::new(-5)]);

        // MulAssign
        let mut f3 = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g3 = Fps::new(vec![Mint::new(1), Mint::new(3)]);
        f3.mul_assign(g3);
        assert_eq!(f3.coeffs, vec![Mint::new(1), Mint::new(5), Mint::new(6)]);
    }

    #[test]
    fn test_edge_cases() {
        // 空のFPSとの演算
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let empty = Fps::new(vec![]);

        let add_res = &f + &empty;
        assert_eq!(add_res.coeffs, f.coeffs);

        let mul_res = &f * &empty;
        assert_eq!(mul_res.coeffs, empty.coeffs);

        // inv の deg=0, 1
        let inv_deg0 = f.inv(0);
        assert_eq!(inv_deg0.coeffs, Vec::<Mint>::new());
        let inv_deg1 = f.inv(1);
        assert_eq!(inv_deg1.coeffs, vec![Mint::new(1)]);
    }

    #[test]
    fn test_neg() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3)]);
        let neg_f = -&f;
        assert_eq!(
            neg_f.coeffs,
            vec![Mint::new(-1), Mint::new(-2), Mint::new(-3)]
        );
        // 元のFPSが変更されていないことを確認
        assert_eq!(f.coeffs, vec![Mint::new(1), Mint::new(2), Mint::new(3)]);
    }

    #[test]
    fn test_pre() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3), Mint::new(4)]);

        // 短く切り詰める
        let f_pre2 = f.pre(2);
        assert_eq!(f_pre2.coeffs, vec![Mint::new(1), Mint::new(2)]);

        // 同じ長さ
        let f_pre4 = f.pre(4);
        assert_eq!(
            f_pre4.coeffs,
            vec![Mint::new(1), Mint::new(2), Mint::new(3), Mint::new(4)]
        );

        // 長くする (0で埋める)
        let f_pre6 = f.pre(6);
        assert_eq!(
            f_pre6.coeffs,
            vec![
                Mint::new(1),
                Mint::new(2),
                Mint::new(3),
                Mint::new(4),
                Mint::new(0),
                Mint::new(0)
            ]
        );

        // 空のFPS
        let empty = Fps::new(vec![]);
        let empty_pre3 = empty.pre(3);
        assert_eq!(empty_pre3.coeffs, vec![Mint::new(0); 3]);
    }

    #[test]
    fn test_diff() {
        // f(x) = 1 + 2x + 3x^2 + 4x^3
        // f'(x) = 2 + 6x + 12x^2
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3), Mint::new(4)]);
        let df = f.diff();
        assert_eq!(df.coeffs, vec![Mint::new(2), Mint::new(6), Mint::new(12)]);

        // 定数
        let c = Fps::new(vec![Mint::new(10)]);
        let dc = c.diff();
        assert!(dc.coeffs.is_empty());

        // 空
        let empty = Fps::new(vec![]);
        let de = empty.diff();
        assert!(de.coeffs.is_empty());
    }

    #[test]
    fn test_integral() {
        // f(x) = 1 + 2x + 3x^2
        // ∫f(x)dx = x + x^2 + x^3
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3)]);
        let int_f = f.integral();
        let inv2 = Mint::new(2).inv();
        let inv3 = Mint::new(3).inv();
        assert_eq!(
            int_f.coeffs,
            vec![
                Mint::new(0),
                Mint::new(1),
                Mint::new(2) * inv2,
                Mint::new(3) * inv3
            ]
        );
        assert_eq!(
            int_f.coeffs,
            vec![Mint::new(0), Mint::new(1), Mint::new(1), Mint::new(1)]
        );

        // 空
        let empty = Fps::new(vec![]);
        let int_e = empty.integral();
        assert_eq!(int_e.coeffs, vec![Mint::new(0)]);
    }

    #[test]
    fn test_log() {
        // log(1+x) = x - x^2/2 + x^3/3 - x^4/4 + ...
        let f = Fps::new(vec![Mint::new(1), Mint::new(1)]); // 1+x
        let log_f = f.log(5); // 5次まで計算

        let inv2 = Mint::new(2).inv();
        let inv3 = Mint::new(3).inv();
        let inv4 = Mint::new(4).inv();
        let expected = vec![Mint::new(0), Mint::new(1), -inv2, inv3, -inv4];
        assert_eq!(log_f.coeffs, expected);

        // f(0) != 1 のケースはパニックする
        // let f_bad = Fps::new(vec![Mint::new(2), Mint::new(1)]); // 2+x
        // f_bad.log(5); // assert! でパニックする
    }

    #[test]
    fn test_exp() {
        // exp(x) = 1 + x + x^2/2 + x^3/6 + x^4/24 + ...
        let f = Fps::new(vec![Mint::new(0), Mint::new(1)]); // x
        let exp_f = f.exp(5); // 5次まで計算

        let inv2 = Mint::new(2).inv();
        let inv6 = Mint::new(6).inv(); // 1/3!
        let inv24 = Mint::new(24).inv(); // 1/4!

        let expected = vec![Mint::new(1), Mint::new(1), inv2, inv6, inv24];
        assert_eq!(exp_f.coeffs, expected);

        // f(0) != 0 のケースはパニックする
        // let f_bad = Fps::new(vec![Mint::new(1), Mint::new(1)]); // 1+x
        // f_bad.exp(5); // assert! でパニックする
    }

    #[test]
    fn test_pow() {
        // (1+x)^2 = 1 + 2x + x^2
        let f = Fps::new(vec![Mint::new(1), Mint::new(1)]); // 1+x
        let pow_f = f.pow(2, 5); // k=2, 5次まで計算

        let expected = vec![
            Mint::new(1),
            Mint::new(2),
            Mint::new(1),
            Mint::new(0),
            Mint::new(0),
        ];
        assert_eq!(pow_f.coeffs, expected);

        // (1-x)^(-1) = 1 + x + x^2 + ...
        let f_inv = Fps::new(vec![Mint::new(1), Mint::new(-1)]); // 1-x
        let pow_f_inv = f_inv.pow(-1, 5); // k=-1, 5次まで計算
        assert_eq!(pow_f_inv.coeffs, vec![Mint::new(1); 5]);

        // f(0) != 1 かつ f_0 != 0 のケース
        // f(x) = (2+x)^2 = 4 + 4x + x^2
        let f2 = Fps::new(vec![Mint::new(2), Mint::new(1)]); // 2+x
        let pow_f2 = f2.pow(2, 5); // k=2, 5次まで計算
        let expected2 = vec![
            Mint::new(4),
            Mint::new(4),
            Mint::new(1),
            Mint::new(0),
            Mint::new(0),
        ];
        assert_eq!(pow_f2.coeffs, expected2);

        // f(0) == 0 のケースは注意が必要
        // (x)^2 = x^2 ( deg は f(0) != 0 の場合と同じ)
        let f_zero_coeff = Fps::new(vec![Mint::new(0), Mint::new(1)]); // x
        let pow_f_zero_coeff = f_zero_coeff.pow(2, 5); // k=2, 5次まで計算
        let expected_zero_coeff = vec![
            Mint::new(0),
            Mint::new(0),
            Mint::new(1),
            Mint::new(0),
            Mint::new(0),
        ];
        assert_eq!(pow_f_zero_coeff.coeffs, expected_zero_coeff);

        // f(0) == 0 で k=0
        let pow_zero_k0 = f_zero_coeff.pow(0, 5);
        assert_eq!(pow_zero_k0.coeffs, Fps::one().pre(5).coeffs);

        // fが空の場合
        let empty = Fps::new(vec![]);
        let pow_empty = empty.pow(2, 5);
        assert_eq!(pow_empty.coeffs, Fps::new(vec![Mint::new(0); 5]).coeffs);

        // fが0の冪乗 (k=0)
        let f_one = Fps::one();
        let pow_k0 = f_one.pow(0, 5);
        assert_eq!(pow_k0.coeffs, Fps::one().pre(5).coeffs);
    }
}
