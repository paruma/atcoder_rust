// https://github.com/NyaanNyaan/library/blob/master/fps/formal-power-series.hpp をもとに作成した。

//! 形式的冪級数ライブラリ
// 設計方針
// - 末尾の 0 係数を常に除去し、同じ多項式をメモリ上で一意に表現する。
//   これにより `Eq` と `Hash` を derive で実装できる。
// - 範囲外の係数は 0 として扱う。
// - `String + &str` と同様に通常の演算は左辺を消費し右辺を借用する。左辺の係数ベクトルを再利用するためである。
// - `/` は FPS の逆元を用いる除算ではなく、商多項式を返す多項式除算を表す。
// - NTT を使う演算では、法が NTT-friendly であり、畳み込み時の NTT 長が法の許容範囲内であることを仮定する。
//   たとえば `998244353` では `2^23` 以下であり、この条件は実行時に検査しない。
use ac_library::{Modulus, StaticModInt, convolution};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Rem, Shl, Shr, Sub, SubAssign};

/// 形式的冪級数を表す構造体。
///
/// 係数を`Vec<StaticModInt<M>>`で保持する。
///
/// `Mod998244353` を法とする場合は、次の型エイリアスを用いる。
///
/// ```ignore
/// type Fps = FormalPowerSeries<Mod998244353>;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct FormalPowerSeries<M: Modulus> {
    // coeffs[k] は x^k の係数。末尾の0係数は保持しない。
    coeffs: Vec<StaticModInt<M>>,
}

impl<M: Modulus> FormalPowerSeries<M> {
    /// 新しいFPSを係数ベクトルから作成する。
    ///
    /// `coeffs[k]` は `x^k` の係数である。
    ///
    /// 計算量: $O(N)$ (N = coeffs.len())
    pub fn new(mut coeffs: Vec<StaticModInt<M>>) -> Self {
        Self::normalize_coeffs(&mut coeffs);
        Self { coeffs }
    }

    /// ゼロ多項式を返す。
    ///
    /// 計算量: O(1)
    pub fn zero() -> Self {
        Self::new(vec![])
    }

    /// 定数1の多項式を返す。
    ///
    /// 計算量: O(1)
    pub fn one() -> Self {
        Self::new(vec![StaticModInt::new(1)])
    }

    /// 多項式 `x` を返す。
    ///
    /// 計算量: O(1)
    pub fn x() -> Self {
        Self::new(vec![StaticModInt::new(0), StaticModInt::new(1)])
    }

    /// 多項式 `x^k` を返す。
    ///
    /// 計算量: O(k)
    pub fn x_pow(k: usize) -> Self {
        let mut coeffs = vec![StaticModInt::new(0); k];
        coeffs.push(StaticModInt::new(1));
        Self::new(coeffs)
    }

    /// 複数の FPS の積を分割統治で計算する。
    ///
    /// 空のスライスの積は `1` とする。
    ///
    /// 計算量: $O(N \log^2 N)$。$N$ は全 FPS の係数数の総和である。
    pub fn product_all(fpss: &[Self]) -> Self {
        match fpss {
            [] => Self::one(),
            [fps] => fps.clone(),
            _ => {
                let mid = fpss.len() / 2;
                Self::product_all(&fpss[..mid]) * &Self::product_all(&fpss[mid..])
            }
        }
    }

    /// 末尾の0を含めない係数ベクトルの長さを返す。
    ///
    /// 計算量: O(1)
    pub fn coeff_len(&self) -> usize {
        self.coeffs.len()
    }

    /// 係数ベクトルを返す。
    ///
    /// 末尾の0係数は含まれない。
    ///
    /// 計算量: $O(1)$
    pub fn coeffs(&self) -> &[StaticModInt<M>] {
        &self.coeffs
    }

    /// 係数ベクトルを取り出す。
    ///
    /// 末尾の0係数は含まれない。
    ///
    /// 計算量: $O(1)$
    pub fn into_coeffs(self) -> Vec<StaticModInt<M>> {
        self.coeffs
    }

    fn normalize_coeffs(coeffs: &mut Vec<StaticModInt<M>>) {
        while coeffs.last().is_some_and(|&c| c.val() == 0) {
            coeffs.pop();
        }
    }

    fn normalize(&mut self) {
        Self::normalize_coeffs(&mut self.coeffs);
    }

    fn reversed_coeffs(coeffs: &[StaticModInt<M>], len: usize) -> Vec<StaticModInt<M>> {
        let mut reversed_coeffs = coeffs.iter().take(len).copied().collect::<Vec<_>>();
        reversed_coeffs.resize(len, StaticModInt::new(0));
        reversed_coeffs.reverse();
        reversed_coeffs
    }

    /// `1/f(x)` の `x^deg` 未満の項をニュートン法で計算する。
    ///
    /// 計算量: O(N log N) (N = deg)
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
            let mut fg = f_k * &g;
            fg.coeffs.truncate(k);

            for c in &mut fg.coeffs {
                *c = -(*c);
            }
            fg.coeffs[0] += 2;
            g *= &fg;
            g.coeffs.truncate(k);
            g.normalize();
        }
        g.coeffs.truncate(deg);
        g.normalize();
        g
    }

    /// `x^len` 以上の項を切り捨て、`x^(len - 1)` 以下の係数のみを残す。
    ///
    /// 計算量: O(len)
    pub fn prefix(mut self, len: usize) -> Self {
        self.coeffs.truncate(len);
        self.normalize();
        self
    }

    /// 導関数 `f'(x)` を計算する。
    ///
    /// 計算量: O(N) (N = self.coeff_len())
    pub fn diff(&self) -> Self {
        Self::new(
            self.coeffs
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, &coeff)| coeff * i)
                .collect(),
        )
    }

    /// 不定積分 `∫f(x)dx` を計算する（積分定数は0）。
    ///
    /// 計算量: O(N) (N = self.coeff_len())
    pub fn integral(&self) -> Self {
        let n = self.coeff_len();
        let mut new_coeffs = vec![StaticModInt::new(0); n + 1];
        if n > 0 {
            // O(N) で逆元を計算する
            let mut invs = vec![StaticModInt::new(0); n + 1];
            invs[1] = StaticModInt::new(1);
            let modulus = StaticModInt::<M>::modulus() as usize;
            for i in 2..=n {
                invs[i] = -invs[modulus % i] * StaticModInt::new(modulus / i);
            }

            for i in 0..n {
                new_coeffs[i + 1] = self.coeffs[i] * invs[i + 1];
            }
        }
        Self::new(new_coeffs)
    }

    /// `log(f(x))` の `x^deg` 未満の項を計算する。
    ///
    /// 前提条件: `f(0) == 1`
    ///
    /// 計算量: O(N log N) (N = deg)
    pub fn log(&self, deg: usize) -> Self {
        assert!(
            !self.coeffs.is_empty() && self.coeffs[0].val() == 1,
            "log(f(x)) を計算するには f(0) == 1 である必要があります。"
        );

        let df = self.diff(); // f'(x)
        let inv_f = self.inv(deg); // f(x)^-1
        let mut df_inv_f = df * &inv_f; // f'(x) * f(x)^-1
        df_inv_f.coeffs.truncate(deg); // deg次まで切り詰める
        df_inv_f.normalize();

        df_inv_f.integral().clone().prefix(deg) // 積分して、指定次数で切り詰める
    }

    /// `exp(f(x))` の `x^deg` 未満の項を計算する。
    ///
    /// 前提条件: `f(0) == 0`
    ///
    /// 計算量: O(N log N) (N = deg)
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
            let f_k = self.clone().prefix(k); // f(x) を k 次まで切り詰める

            // g = g * (1 - log(g) + f)
            let log_g = g.log(k);
            let val = f_k - &log_g; // f - log(g)
            let one_plus_val = Self::one() + &val; // 1 + (f - log(g))
            g *= &one_plus_val;
            g.coeffs.truncate(k);
            g.normalize();
        }
        g.prefix(deg) // 指定次数で切り詰める
    }

    /// `f(x)^k` の `x^deg` 未満の項を計算する。
    ///
    /// 計算量: O(N log N + log k) (N = deg)
    pub fn pow(&self, k: i64, deg: usize) -> Self {
        if deg == 0 {
            return Self::new(vec![]);
        }
        if k == 0 {
            return Self::one();
        }
        assert!(
            k >= 0 || self.coeffs.first().is_some_and(|c| c.val() != 0),
            "負の指数には定数項が0でないFPSが必要です"
        );
        if self.coeffs.is_empty() {
            return Self::zero();
        }

        let Some(first_nonzero_idx) = self.coeffs.iter().position(|c| c.val() != 0) else {
            // 全ての係数が0
            return Self::zero();
        };

        if first_nonzero_idx > 0 {
            // f(0) == 0 の場合
            if (first_nonzero_idx as i64) * k >= deg as i64 {
                return Self::zero();
            }

            // f(x) = x^first_nonzero_idx * g(x) (g(0) != 0)
            // f(x)^k = x^(first_nonzero_idx * k) * g(x)^k
            let g_coeffs = self.coeffs[first_nonzero_idx..].to_vec();
            let g = Self::new(g_coeffs);

            // g(x)^k を計算
            let gk = g.pow(k, deg - first_nonzero_idx * k as usize);

            // x^(first_nonzero_idx * k) を掛ける (左シフト)
            let mut result_coeffs = vec![StaticModInt::new(0); first_nonzero_idx * k as usize];
            result_coeffs.extend_from_slice(&gk.coeffs);
            Self::new(result_coeffs).prefix(deg)
        } else {
            // f(0) != 0 の場合
            let f0 = self.coeffs[0]; // f0をここで定義
            let f0_pow_k = if k >= 0 {
                // f0_pow_k をf0定義直後に移動
                f0.pow(k as u64)
            } else {
                f0.inv().pow(k.unsigned_abs())
            };

            let f0_inv = f0.inv();
            let normalized_f: Vec<_> = self.coeffs.iter().map(|&c| c * f0_inv).collect();
            let normalized_fps = Self::new(normalized_f);

            let log_normalized_f = normalized_fps.log(deg);

            let k_val = if k >= 0 {
                StaticModInt::new(k as u64)
            } else {
                StaticModInt::new(k)
            };
            let k_times_log_normalized_f_coeffs: Vec<_> =
                log_normalized_f.coeffs.iter().map(|&c| c * k_val).collect();
            let k_times_log_normalized_f = Self::new(k_times_log_normalized_f_coeffs);

            let res = k_times_log_normalized_f.exp(deg);

            let final_coeffs: Vec<_> = res.coeffs.iter().map(|&c| c * f0_pow_k).collect();
            Self::new(final_coeffs).prefix(deg)
        }
    }

    /// 多項式をスカラ値 x で評価する (Horner法)。
    ///
    /// 計算量: O(N) (N = self.coeff_len())
    pub fn eval(&self, x: StaticModInt<M>) -> StaticModInt<M> {
        self.coeffs
            .iter()
            .rev()
            .fold(StaticModInt::new(0), |res, &coeff| res * x + coeff)
    }

    /// Polynomial Taylor Shift `f(x + c)` を計算する。
    ///
    /// 計算量: $O(N \log N)$ (N = self.coeff_len())
    pub fn taylor_shift(&self, c: StaticModInt<M>) -> Self {
        let n = self.coeff_len();
        if n == 0 {
            return Self::zero();
        }

        let mut factorials = vec![StaticModInt::new(1); n];
        for i in 1..n {
            factorials[i] = factorials[i - 1] * i;
        }

        let mut inv_factorials = vec![StaticModInt::new(0); n];
        inv_factorials[n - 1] = factorials[n - 1].inv();
        for i in (1..n).rev() {
            inv_factorials[i - 1] = inv_factorials[i] * i;
        }

        let reversed = self
            .coeffs
            .iter()
            .zip(factorials.iter())
            .map(|(&a, &factorial)| a * factorial)
            .rev()
            .collect::<Vec<_>>();
        let mut c_pow = StaticModInt::new(1);
        let g = inv_factorials
            .iter()
            .map(|&inv_factorial| {
                let value = c_pow * inv_factorial;
                c_pow *= c;
                value
            })
            .collect::<Vec<_>>();
        let product = convolution(&reversed, &g);

        Self::new(
            (0..n)
                .map(|l| product[n - 1 - l] * inv_factorials[l])
                .collect(),
        )
    }

    /// `x^k` の係数を取得する。`k` が `coeffs` の配列範囲外の場合は0を返す。
    ///
    /// 計算量: O(1)
    pub fn get(&self, k: usize) -> StaticModInt<M> {
        self.coeffs.get(k).copied().unwrap_or_default()
    }

    /// 多項式の割り算 `self / rhs` を計算する。
    /// N = self.coeff_len(), M = rhs.coeff_len() とする。
    ///
    /// 計算量: M <= 64 の場合 O(N * M)、M > 64 の場合 O((N - M) log (N - M))
    pub fn div_polynomial(&self, rhs: &Self) -> Self {
        let mut a_coeffs = self.coeffs.clone();
        let b_coeffs = rhs.coeffs.clone();

        if b_coeffs.is_empty() {
            panic!("Division by zero polynomial");
        }

        let n = a_coeffs.len();
        let m = b_coeffs.len();

        // 割られる多項式の次数が割る多項式の次数より小さい場合、商は0
        if n < m {
            return Self::zero();
        }

        let quotient_deg = n - m + 1;
        let long_division_threshold = 64; // C++の実装に合わせる

        if m <= long_division_threshold {
            // 長除算 (Long Division)
            // 商の係数を格納するベクタ。次数は n - m。
            let mut q_coeffs = vec![StaticModInt::new(0); quotient_deg];

            // 割る多項式の最高次係数の逆元
            let b_leading_inv = b_coeffs[m - 1].inv();

            // 多項式の長除法
            // 商の最高次係数から順に計算していく
            for i in (0..=(n - m)).rev() {
                // 現在のステップで商に立つ係数
                // a_coeffs[i + m - 1] は、現在の割られる多項式における最高次係数
                let current_a_leading_coeff = a_coeffs[i + m - 1];

                // この係数が0の場合、商のこの項も0
                if current_a_leading_coeff.val() == 0 {
                    continue;
                }

                let q_coeff = current_a_leading_coeff * b_leading_inv;
                q_coeffs[i] = q_coeff;

                // 割られる多項式から (q_coeff * x^i * B(x)) を引く
                for j in 0..m {
                    a_coeffs[i + j] -= q_coeff * b_coeffs[j];
                }
            }

            let mut quotient = Self::new(q_coeffs);
            quotient.normalize();
            quotient
        } else {
            // NTTベースの高速除算 (Fast Polynomial Division)
            // (A(x)を N-M 次で反転) * (B(x)を N-M 次で反転の逆元) ) を N-M 次で反転
            let a_rev = Self::reversed_coeffs(&a_coeffs, n);
            let b_rev = Self::new(Self::reversed_coeffs(&b_coeffs, m));

            let b_rev_inv = b_rev.inv(quotient_deg);

            let mut product = convolution(&a_rev[..quotient_deg], b_rev_inv.coeffs());
            product.truncate(quotient_deg);
            Self::new(Self::reversed_coeffs(&product, quotient_deg))
        }
    }

    /// 多項式の剰余 `self % rhs` を計算する。
    /// N = self.coeff_len(), M = rhs.coeff_len() とする。
    ///
    /// 計算量: div_polynomial の計算量に依存するため、M <= 64 の場合 O(N * M)、M > 64 の場合 O(N log N)
    pub fn rem_polynomial(&self, rhs: &Self) -> Self {
        let q = self.div_polynomial(rhs);
        let mut r = self.clone() - &(q * rhs);
        r.normalize();
        r
    }
}
// --- 算術演算子 ---

macro_rules! impl_integer_scalar_ops {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<M: Modulus> Add<$ty> for FormalPowerSeries<M> {
                type Output = Self;

                fn add(self, rhs: $ty) -> Self::Output {
                    self + StaticModInt::new(rhs)
                }
            }

            impl<M: Modulus> AddAssign<$ty> for FormalPowerSeries<M> {
                fn add_assign(&mut self, rhs: $ty) {
                    *self += StaticModInt::new(rhs);
                }
            }

            impl<M: Modulus> Sub<$ty> for FormalPowerSeries<M> {
                type Output = Self;

                fn sub(self, rhs: $ty) -> Self::Output {
                    self - StaticModInt::new(rhs)
                }
            }

            impl<M: Modulus> SubAssign<$ty> for FormalPowerSeries<M> {
                fn sub_assign(&mut self, rhs: $ty) {
                    *self -= StaticModInt::new(rhs);
                }
            }

            impl<M: Modulus> Mul<$ty> for FormalPowerSeries<M> {
                type Output = Self;

                fn mul(self, rhs: $ty) -> Self::Output {
                    self * StaticModInt::new(rhs)
                }
            }

            impl<M: Modulus> MulAssign<$ty> for FormalPowerSeries<M> {
                fn mul_assign(&mut self, rhs: $ty) {
                    *self *= StaticModInt::new(rhs);
                }
            }
        )*
    };
}

impl_integer_scalar_ops!(usize, i64);

// FPS + FPS
/// N = self.coeff_len(), M = rhs.coeff_len() とする。
///
/// 計算量: O(N + M)
impl<M: Modulus> Add<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn add(mut self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        self += rhs;
        self
    }
}

// FPS + Mint
/// N = self.coeff_len() とする。
///
/// 計算量: O(N)
impl<M: Modulus> Add<StaticModInt<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn add(mut self, rhs: StaticModInt<M>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<M: Modulus> AddAssign<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    /// N = self.coeff_len(), M = rhs.coeff_len() とする。
    ///
    /// 計算量: O(N + M)
    fn add_assign(&mut self, rhs: &FormalPowerSeries<M>) {
        if self.coeff_len() < rhs.coeff_len() {
            self.coeffs.resize(rhs.coeff_len(), StaticModInt::new(0));
        }
        for i in 0..rhs.coeff_len() {
            self.coeffs[i] += rhs.coeffs[i];
        }
        self.normalize();
    }
}

// FPS += Mint
/// 計算量: O(1)
impl<M: Modulus> AddAssign<StaticModInt<M>> for FormalPowerSeries<M> {
    fn add_assign(&mut self, rhs: StaticModInt<M>) {
        if self.coeffs.is_empty() {
            self.coeffs.push(rhs);
        } else {
            self.coeffs[0] += rhs;
        }
        self.normalize();
    }
}

// FPS -= Mint
/// 計算量: O(1)
impl<M: Modulus> SubAssign<StaticModInt<M>> for FormalPowerSeries<M> {
    fn sub_assign(&mut self, rhs: StaticModInt<M>) {
        if self.coeffs.is_empty() {
            self.coeffs.push(-rhs);
        } else {
            self.coeffs[0] -= rhs;
        }
        self.normalize();
    }
}

// FPS *= Mint
/// N = self.coeff_len() とする。
///
/// 計算量: O(N)
impl<M: Modulus> MulAssign<StaticModInt<M>> for FormalPowerSeries<M> {
    fn mul_assign(&mut self, rhs: StaticModInt<M>) {
        for c in &mut self.coeffs {
            *c *= rhs;
        }
        self.normalize();
    }
}

// FPS - FPS
/// N = self.coeff_len(), M = rhs.coeff_len() とする。
///
/// 計算量: O(N + M)
impl<M: Modulus> Sub<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn sub(mut self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        self -= rhs;
        self
    }
}

// FPS - Mint
/// N = self.coeff_len() とする。
///
/// 計算量: O(N)
impl<M: Modulus> Sub<StaticModInt<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn sub(mut self, rhs: StaticModInt<M>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<M: Modulus> SubAssign<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    /// N = self.coeff_len(), M = rhs.coeff_len() とする。
    ///
    /// 計算量: O(N + M)
    fn sub_assign(&mut self, rhs: &FormalPowerSeries<M>) {
        if self.coeff_len() < rhs.coeff_len() {
            self.coeffs.resize(rhs.coeff_len(), StaticModInt::new(0));
        }
        for i in 0..rhs.coeff_len() {
            self.coeffs[i] -= rhs.coeffs[i];
        }
        self.normalize();
    }
}

// FPS * FPS
/// N = self.coeff_len(), M = rhs.coeff_len() とする。
///
/// 計算量: O((N+M) log (N+M)) (convolutionの計算量に依存)
impl<M: Modulus> Mul<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn mul(self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        if self.coeffs.is_empty() || rhs.coeffs.is_empty() {
            return Self::new(vec![]);
        }
        let coeffs = convolution(&self.coeffs, &rhs.coeffs);
        Self::new(coeffs)
    }
}

// FPS * Mint
/// N = self.coeff_len() とする。
///
/// 計算量: O(N)
impl<M: Modulus> Mul<StaticModInt<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn mul(mut self, rhs: StaticModInt<M>) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<M: Modulus> MulAssign<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    /// N = self.coeff_len(), M = rhs.coeff_len() とする。
    ///
    /// 計算量: O((N+M) log (N+M))
    fn mul_assign(&mut self, rhs: &FormalPowerSeries<M>) {
        let coeffs = convolution(&self.coeffs, &rhs.coeffs);
        *self = Self::new(coeffs);
    }
}

// 単項マイナス
/// N = self.coeff_len() とする。
///
/// 計算量: O(N)
impl<M: Modulus> Neg for FormalPowerSeries<M> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for coeff in &mut self.coeffs {
            *coeff = -*coeff;
        }
        self
    }
}

// FPS << usize
/// N = self.coeff_len() とする。rhs はシフト量。
///
/// `x^rhs` を掛ける。
///
/// 計算量: O(N + rhs)
impl<M: Modulus> Shl<usize> for FormalPowerSeries<M> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        if self.coeffs.is_empty() {
            return self;
        }
        let mut new_coeffs = vec![StaticModInt::new(0); rhs];
        new_coeffs.extend_from_slice(&self.coeffs);
        Self::new(new_coeffs)
    }
}

// FPS >> usize
/// N = self.coeff_len() とする。rhs はシフト量。
///
/// 低次の `rhs` 項を捨てる。
///
/// 計算量: O(N) (最悪ケースで drain が N 要素を処理するため)
impl<M: Modulus> Shr<usize> for FormalPowerSeries<M> {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        if self.coeffs.len() <= rhs {
            self.coeffs.clear();
        } else {
            self.coeffs.drain(0..rhs);
        }
        self
    }
}

// FPS / FPS
/// N = self.coeff_len(), M = rhs.coeff_len() とする。
///
/// 計算量: div_polynomial の計算量に依存。M <= 64 の場合 O(N * M)、M > 64 の場合 O((N - M) log (N - M))
impl<M: Modulus> Div<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn div(self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        self.div_polynomial(rhs)
    }
}

// FPS % FPS
/// N = self.coeff_len(), M = rhs.coeff_len() とする。
///
/// 計算量: rem_polynomial の計算量に依存。M <= 64 の場合 O(N * M)、M > 64 の場合 O(N log N)
impl<M: Modulus> Rem<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = Self;

    fn rem(self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        self.rem_polynomial(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::Mod998244353;
    use rand::Rng;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    type Mint = StaticModInt<Mod998244353>;
    type Fps = FormalPowerSeries<Mod998244353>;

    #[test]
    fn test_add() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g = Fps::new(vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
        let h = f + &g;
        assert_eq!(h.coeffs, vec![Mint::new(4), Mint::new(6), Mint::new(5)]);
        assert_eq!(g.coeffs, vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
    }

    #[test]
    fn test_sub() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g = Fps::new(vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
        let h = f - &g;
        assert_eq!(h.coeffs, vec![Mint::new(-2), Mint::new(-2), Mint::new(-5)]);
        assert_eq!(g.coeffs, vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
    }

    #[test]
    fn test_mul() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g = Fps::new(vec![Mint::new(1), Mint::new(3)]);
        // (1+2x)(1+3x) = 1 + 5x + 6x^2
        let h = f * &g;
        assert_eq!(h.coeffs, vec![Mint::new(1), Mint::new(5), Mint::new(6)]);
        assert_eq!(g.coeffs, vec![Mint::new(1), Mint::new(3)]);
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
        let mut rng = StdRng::from_os_rng();
        for _ in 0..1000 {
            let deg = rng.random_range(1..=50);
            let mut coeffs = vec![Mint::new(0); deg];
            for i in 0..deg {
                coeffs[i] = Mint::new(rng.random_range(-3..=3));
            }
            // 定数項は0でない
            while coeffs[0].val() == 0 {
                coeffs[0] = Mint::new(rng.random_range(-3..=3));
            }

            let f = Fps::new(coeffs);
            let f_inv = f.inv(deg);

            let mut actual = f.clone() * &f_inv;
            actual.coeffs.truncate(deg);
            actual.normalize();

            let expected = vec![Mint::new(1)];

            assert_eq!(actual.coeffs, expected, "f * f.inv() should be 1");
        }
    }

    #[test]
    fn test_utils() {
        // one
        let o = Fps::one();
        assert_eq!(o.coeffs, vec![Mint::new(1)]);

        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(0), Mint::new(0)]);
        assert_eq!(f.coeffs, vec![Mint::new(1), Mint::new(2)]);
        let g = Fps::new(vec![Mint::new(0), Mint::new(0)]);
        assert_eq!(g.coeffs, Vec::<Mint>::new());

        assert_eq!(Fps::x().coeffs, vec![Mint::new(0), Mint::new(1)]);
        assert_eq!(Fps::x_pow(0), Fps::one());
        assert_eq!(
            Fps::x_pow(3).coeffs,
            vec![Mint::new(0), Mint::new(0), Mint::new(0), Mint::new(1)]
        );
    }

    #[test]
    fn test_product_all() {
        assert_eq!(Fps::product_all(&[]), Fps::one());

        let f = Fps::new(vec![Mint::new(2), Mint::new(3)]);
        assert_eq!(Fps::product_all(std::slice::from_ref(&f)), f);

        let fpss = [
            Fps::new(vec![Mint::new(1), Mint::new(1)]),
            Fps::new(vec![Mint::new(2), Mint::new(1)]),
            Fps::new(vec![Mint::new(3), Mint::new(1)]),
            Fps::new(vec![Mint::new(4), Mint::new(1)]),
            Fps::new(vec![Mint::new(5), Mint::new(1)]),
        ];
        assert_eq!(
            Fps::product_all(&fpss).coeffs,
            vec![
                Mint::new(120),
                Mint::new(274),
                Mint::new(225),
                Mint::new(85),
                Mint::new(15),
                Mint::new(1),
            ]
        );
    }

    #[test]
    fn test_taylor_shift() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3)]);
        assert_eq!(
            f.taylor_shift(Mint::new(5)).coeffs,
            vec![Mint::new(86), Mint::new(32), Mint::new(3)]
        );
        assert_eq!(f.taylor_shift(Mint::new(0)), f);
        assert_eq!(Fps::zero().taylor_shift(Mint::new(5)), Fps::zero());

        let mut rng = StdRng::seed_from_u64(0);
        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let coeffs = (0..n)
                .map(|_| Mint::new(rng.random_range(-10..=10)))
                .collect::<Vec<_>>();
            let c = Mint::new(rng.random_range(-10..=10));
            let f = Fps::new(coeffs.clone());

            let mut expected = vec![Mint::new(0); n];
            for (k, &a) in coeffs.iter().enumerate() {
                let mut binom = Mint::new(1);
                for l in 0..=k {
                    expected[l] += a * binom * c.pow((k - l) as u64);
                    if l < k {
                        binom *= k - l;
                        binom /= l + 1;
                    }
                }
            }

            assert_eq!(f.taylor_shift(c), Fps::new(expected));
        }
    }

    #[test]
    fn test_normalized_representation() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(0)]);
        assert_eq!(f.coeffs(), &[Mint::new(1)]);
        assert_eq!(f, Fps::new(vec![Mint::new(1)]));
        assert_eq!(
            Fps::new(vec![Mint::new(1), Mint::new(0)]).into_coeffs(),
            vec![Mint::new(1)]
        );

        let g = Fps::new(vec![Mint::new(1)]);
        assert_eq!(f - &g, Fps::zero());

        let mut h = Fps::new(vec![Mint::new(1)]);
        h *= Mint::new(0);
        assert_eq!(h, Fps::zero());
    }

    #[test]
    fn test_assign_ops() {
        // AddAssign
        let mut f1 = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g1 = Fps::new(vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
        f1 += &g1;
        assert_eq!(f1.coeffs, vec![Mint::new(4), Mint::new(6), Mint::new(5)]);
        assert_eq!(g1.coeffs, vec![Mint::new(3), Mint::new(4), Mint::new(5)]);

        // SubAssign
        let mut f2 = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g2 = Fps::new(vec![Mint::new(3), Mint::new(4), Mint::new(5)]);
        f2 -= &g2;
        assert_eq!(f2.coeffs, vec![Mint::new(-2), Mint::new(-2), Mint::new(-5)]);
        assert_eq!(g2.coeffs, vec![Mint::new(3), Mint::new(4), Mint::new(5)]);

        // MulAssign
        let mut f3 = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let g3 = Fps::new(vec![Mint::new(1), Mint::new(3)]);
        f3 *= &g3;
        assert_eq!(f3.coeffs, vec![Mint::new(1), Mint::new(5), Mint::new(6)]);
        assert_eq!(g3.coeffs, vec![Mint::new(1), Mint::new(3)]);
    }

    #[test]
    fn test_edge_cases() {
        // 空のFPSとの演算
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let empty = Fps::new(vec![]);

        let add_res = f.clone() + &empty;
        assert_eq!(add_res.coeffs, f.coeffs);

        let mul_res = f.clone() * &empty;
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
        let neg_f = -f.clone();
        assert_eq!(
            neg_f.coeffs,
            vec![Mint::new(-1), Mint::new(-2), Mint::new(-3)]
        );
        // 元のFPSが変更されていないことを確認
        assert_eq!(f.coeffs, vec![Mint::new(1), Mint::new(2), Mint::new(3)]);
    }

    #[test]
    fn test_prefix() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3), Mint::new(4)]);

        // 短く切り詰める
        let f_pre2 = f.clone().prefix(2);
        assert_eq!(f_pre2.coeffs, vec![Mint::new(1), Mint::new(2)]);

        // 同じ長さ
        let f_pre4 = f.clone().prefix(4);
        assert_eq!(
            f_pre4.coeffs,
            vec![Mint::new(1), Mint::new(2), Mint::new(3), Mint::new(4)]
        );

        // 長くしても末尾の0は保持しない
        let f_pre6 = f.clone().prefix(6);
        assert_eq!(
            f_pre6.coeffs,
            vec![Mint::new(1), Mint::new(2), Mint::new(3), Mint::new(4)]
        );

        // 空のFPS
        let empty = Fps::new(vec![]);
        let empty_pre3 = empty.prefix(3);
        assert!(empty_pre3.coeffs.is_empty());
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
        assert!(int_e.coeffs.is_empty());
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
    fn test_exp_zero() {
        let f = Fps::zero();
        let exp_f = f.exp(5);
        let expected = vec![Mint::new(1)];
        assert_eq!(exp_f.coeffs, expected);
    }

    /// テイラー展開 exp(f) = Σ_{i=0}^∞ (f^i / i!) を用いて愚直に計算する。
    fn naive_exp<M: Modulus>(f: &FormalPowerSeries<M>, deg: usize) -> FormalPowerSeries<M> {
        let mut res = vec![StaticModInt::new(0); deg];
        let mut term = FormalPowerSeries::one();
        let mut fact_inv = StaticModInt::new(1);
        for i in 0..deg {
            if i > 0 {
                fact_inv *= StaticModInt::new(i as u64).inv();
            }
            for j in 0..deg {
                res[j] += term.get(j) * fact_inv;
            }
            if i + 1 < deg {
                term *= f;
                term.coeffs.truncate(deg);
                term.normalize();
            }
        }
        FormalPowerSeries::new(res)
    }

    /// テイラー展開 ln(1+g) = Σ_{i=1}^∞ (-1)^{i-1} * (g^i / i) (ただし g = f - 1) を用いて愚直に計算する。
    fn naive_log<M: Modulus>(f: &FormalPowerSeries<M>, deg: usize) -> FormalPowerSeries<M> {
        let mut g = f.clone();
        g.coeffs[0] -= 1; // g = f - 1
        let mut res = vec![StaticModInt::new(0); deg];
        let mut term = g.clone();
        for i in 1..deg {
            let mut val = StaticModInt::new(1) / i;
            if i % 2 == 0 {
                val = -val;
            }
            for j in 0..deg {
                res[j] += term.get(j) * val;
            }
            if i + 1 < deg {
                term *= &g;
                term.coeffs.truncate(deg);
                term.normalize();
            }
        }
        FormalPowerSeries::new(res)
    }

    fn naive_pow<M: Modulus>(
        f: &FormalPowerSeries<M>,
        k: usize,
        deg: usize,
    ) -> FormalPowerSeries<M> {
        let mut res = FormalPowerSeries::one().prefix(deg);
        for _ in 0..k {
            res *= f;
            res.coeffs.truncate(deg);
            res.normalize();
        }
        res.prefix(deg)
    }

    #[test]
    #[ignore]
    fn test_exp_random() {
        // exp(f) の結果が、定義に基づくテイラー展開（naive_exp）の結果と一致するかをテストすることで、
        // exp の実装の正当性を確認しています。
        let mut rng = StdRng::from_os_rng();
        for _ in 0..100 {
            let deg = rng.random_range(1..=50);
            let mut coeffs = vec![Mint::new(0); deg];
            for i in 1..deg {
                coeffs[i] = Mint::new(rng.random_range(-3..=3));
            }
            coeffs[0] = Mint::new(0);

            let f = Fps::new(coeffs);
            let exp_f = f.exp(deg);
            let exp_naive = naive_exp(&f, deg);

            assert_eq!(
                exp_f.coeffs, exp_naive.coeffs,
                "exp(f) should match naive Taylor expansion"
            );
        }
    }

    #[test]
    #[ignore]
    fn test_exp_log_inverse_property() {
        // log(exp(f)) == f (ただし f(0) = 0) という性質を用いて、
        // exp と log の実装が互いに整合しているか（逆写像になっているか）をテストしています。
        let mut rng = StdRng::from_os_rng();
        for _ in 0..100 {
            let deg = rng.random_range(1..=50);
            let mut coeffs = vec![Mint::new(0); deg];
            for i in 1..deg {
                coeffs[i] = Mint::new(rng.random_range(0..Mint::modulus()));
            }
            coeffs[0] = Mint::new(0);

            let f = Fps::new(coeffs);
            let exp_f = f.exp(deg);
            let log_exp_f = exp_f.log(deg);

            assert_eq!(
                f.coeffs, log_exp_f.coeffs,
                "log(exp(f)) should be f for f(0)=0"
            );
        }
    }

    #[test]
    #[ignore]
    fn test_log_random() {
        // log(f) の結果が、定義に基づくテイラー展開（naive_log）の結果と一致するかをテストすることで、
        // log の実装の正当性を確認しています。
        let mut rng = StdRng::from_os_rng();
        for _ in 0..100 {
            let deg = rng.random_range(1..=50);
            let mut coeffs = vec![Mint::new(0); deg];
            for i in 1..deg {
                coeffs[i] = Mint::new(rng.random_range(-3..=3));
            }
            coeffs[0] = Mint::new(1);

            let f = Fps::new(coeffs);
            let log_f = f.log(deg);
            let log_naive = naive_log(&f, deg);

            assert_eq!(
                log_f.coeffs, log_naive.coeffs,
                "log(f) should match naive Taylor expansion"
            );
        }
    }

    #[test]
    fn test_pow() {
        // (1+x)^2 = 1 + 2x + x^2
        let f = Fps::new(vec![Mint::new(1), Mint::new(1)]); // 1+x
        let pow_f = f.pow(2, 5); // k=2, 5次まで計算

        let expected = vec![Mint::new(1), Mint::new(2), Mint::new(1)];
        assert_eq!(pow_f.coeffs, expected);

        // (1-x)^(-1) = 1 + x + x^2 + ...
        let f_inv = Fps::new(vec![Mint::new(1), Mint::new(-1)]); // 1-x
        let pow_f_inv = f_inv.pow(-1, 5); // k=-1, 5次まで計算
        assert_eq!(pow_f_inv.coeffs, vec![Mint::new(1); 5]);

        // f(0) != 1 かつ f_0 != 0 のケース
        // f(x) = (2+x)^2 = 4 + 4x + x^2
        let f2 = Fps::new(vec![Mint::new(2), Mint::new(1)]); // 2+x
        let pow_f2 = f2.pow(2, 5); // k=2, 5次まで計算
        let expected2 = vec![Mint::new(4), Mint::new(4), Mint::new(1)];
        assert_eq!(pow_f2.coeffs, expected2);

        // f(0) == 0 のケースは注意が必要
        // (x)^2 = x^2 ( deg は f(0) != 0 の場合と同じ)
        let f_zero_coeff = Fps::new(vec![Mint::new(0), Mint::new(1)]); // x
        let pow_f_zero_coeff = f_zero_coeff.pow(2, 5); // k=2, 5次まで計算
        let expected_zero_coeff = vec![Mint::new(0), Mint::new(0), Mint::new(1)];
        assert_eq!(pow_f_zero_coeff.coeffs, expected_zero_coeff);

        // f(0) == 0 で k=0
        let pow_zero_k0 = f_zero_coeff.pow(0, 5);
        assert_eq!(pow_zero_k0.coeffs, Fps::one().prefix(5).coeffs);

        // fが空の場合 (0^k)
        let empty = Fps::new(vec![]);
        let pow_empty = empty.pow(2, 5);
        assert_eq!(pow_empty.coeffs, Fps::new(vec![Mint::new(0); 5]).coeffs);

        // 0^0
        let pow_zero_zero = empty.pow(0, 5);
        assert_eq!(pow_zero_zero.coeffs, Fps::one().prefix(5).coeffs);

        // fが0の冪乗 (k=0)
        let f_one = Fps::one();
        let pow_k0 = f_one.pow(0, 5);
        assert_eq!(pow_k0.coeffs, Fps::one().prefix(5).coeffs);
    }

    #[test]
    #[ignore]
    fn test_pow_random() {
        // f.pow(k, deg) の結果が、愚直な掛け算の結果（naive_pow）と一致するかをテストすることで、
        // べき乗の実装の正当性を確認しています。
        let mut rng = StdRng::from_os_rng();
        for _ in 0..100 {
            let deg = rng.random_range(1..=50);
            let k = rng.random_range(0..=10) as i64;

            let mut coeffs = vec![Mint::new(0); deg];
            // 1/3 の確率で f(0) = 0 にする
            let start = if rng.random_bool(0.33) && deg > 1 {
                rng.random_range(1..deg.min(5))
            } else {
                0
            };
            for i in start..deg {
                coeffs[i] = Mint::new(rng.random_range(-3..=3));
            }
            if start < deg && coeffs[start].val() == 0 {
                coeffs[start] = Mint::new(1);
            }
            // 不要な要素を消す(trimはしないが、末尾が0になる可能性はある)

            let f = Fps::new(coeffs);
            let f_k = f.pow(k, deg);

            // 値の正当性テスト: naive_pow との比較 (k >= 0 の場合)
            if k >= 0 {
                let f_naive = naive_pow(&f, k as usize, deg);
                assert_eq!(
                    f_k.coeffs, f_naive.coeffs,
                    "f^k should match naive multiplication"
                );
            }
        }
    }

    #[test]
    fn test_shl() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3)]); // 1 + 2x + 3x^2

        // 2回左シフト: (1 + 2x + 3x^2) * x^2 = x^2 + 2x^3 + 3x^4
        let shl_f = f.clone() << 2;
        assert_eq!(
            shl_f.coeffs,
            vec![
                Mint::new(0),
                Mint::new(0),
                Mint::new(1),
                Mint::new(2),
                Mint::new(3)
            ]
        );

        // 0回左シフト
        let shl_f0 = f.clone() << 0;
        assert_eq!(
            shl_f0.coeffs,
            vec![Mint::new(1), Mint::new(2), Mint::new(3)]
        );

        // 空のFPS
        let empty = Fps::new(vec![]);
        let shl_empty = empty << 5;
        assert_eq!(shl_empty.coeffs, vec![]);
    }

    #[test]
    fn test_scalar_assign_ops() {
        // AddAssign
        let mut f1 = Fps::new(vec![Mint::new(1), Mint::new(2)]); // 1 + 2x
        f1.add_assign(Mint::new(3)); // (1 + 2x) + 3 = 4 + 2x
        assert_eq!(f1.coeffs, vec![Mint::new(4), Mint::new(2)]);

        let mut f_empty_add = Fps::new(vec![]);
        f_empty_add.add_assign(Mint::new(5));
        assert_eq!(f_empty_add.coeffs, vec![Mint::new(5)]);

        // SubAssign
        let mut f2 = Fps::new(vec![Mint::new(4), Mint::new(2)]); // 4 + 2x
        f2.sub_assign(Mint::new(3)); // (4 + 2x) - 3 = 1 + 2x
        assert_eq!(f2.coeffs, vec![Mint::new(1), Mint::new(2)]);

        let mut f_empty_sub = Fps::new(vec![]);
        f_empty_sub.sub_assign(Mint::new(5));
        assert_eq!(f_empty_sub.coeffs, vec![Mint::new(-5)]);

        // MulAssign
        let mut f3 = Fps::new(vec![Mint::new(1), Mint::new(2)]); // 1 + 2x
        f3.mul_assign(Mint::new(3)); // (1 + 2x) * 3 = 3 + 6x
        assert_eq!(f3.coeffs, vec![Mint::new(3), Mint::new(6)]);

        let mut f_empty_mul = Fps::new(vec![]);
        f_empty_mul.mul_assign(Mint::new(3));
        assert_eq!(f_empty_mul.coeffs, vec![]);
    }

    #[test]
    fn test_scalar_ops() {
        // Add (FPS + Mint)
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]); // 1 + 2x
        let add_res = f + Mint::new(3); // (1 + 2x) + 3 = 4 + 2x
        assert_eq!(add_res.coeffs, vec![Mint::new(4), Mint::new(2)]);

        let empty = Fps::new(vec![]);
        let empty_add = empty.clone() + Mint::new(5);
        assert_eq!(empty_add.coeffs, vec![Mint::new(5)]);

        // Sub (FPS - Mint)
        let f2 = Fps::new(vec![Mint::new(4), Mint::new(2)]); // 4 + 2x
        let sub_res = f2 - Mint::new(3); // (4 + 2x) - 3 = 1 + 2x
        assert_eq!(sub_res.coeffs, vec![Mint::new(1), Mint::new(2)]);

        let empty_sub = empty.clone() - Mint::new(5);
        assert_eq!(empty_sub.coeffs, vec![Mint::new(-5)]);

        // Mul (FPS * Mint)
        let f3 = Fps::new(vec![Mint::new(1), Mint::new(2)]); // 1 + 2x
        let mul_res = f3 * Mint::new(3); // (1 + 2x) * 3 = 3 + 6x
        assert_eq!(mul_res.coeffs, vec![Mint::new(3), Mint::new(6)]);

        let empty_mul = empty * Mint::new(3);
        assert_eq!(empty_mul.coeffs, vec![]);
    }

    #[test]
    fn test_integer_scalar_ops() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        assert_eq!(
            (f.clone() + 3_usize).coeffs,
            vec![Mint::new(4), Mint::new(2)]
        );
        assert_eq!(
            (f.clone() - 3_usize).coeffs,
            vec![Mint::new(-2), Mint::new(2)]
        );
        assert_eq!(
            (f.clone() * 3_usize).coeffs,
            vec![Mint::new(3), Mint::new(6)]
        );
        assert_eq!(
            (f.clone() + -3_i64).coeffs,
            vec![Mint::new(-2), Mint::new(2)]
        );
        assert_eq!(
            (f.clone() - -3_i64).coeffs,
            vec![Mint::new(4), Mint::new(2)]
        );
        assert_eq!(
            (f.clone() * -3_i64).coeffs,
            vec![Mint::new(-3), Mint::new(-6)]
        );

        let mut usize_f = f.clone();
        usize_f += 3_usize;
        usize_f -= 1_usize;
        usize_f *= 2_usize;
        assert_eq!(usize_f.coeffs, vec![Mint::new(6), Mint::new(4)]);

        let mut i64_f = f;
        i64_f += -3_i64;
        i64_f -= -1_i64;
        i64_f *= -2_i64;
        assert_eq!(i64_f.coeffs, vec![Mint::new(2), Mint::new(-4)]);
    }

    #[test]
    fn test_shr() {
        let f = Fps::new(vec![
            Mint::new(1),
            Mint::new(2),
            Mint::new(3),
            Mint::new(4),
            Mint::new(5),
        ]); // 1 + 2x + 3x^2 + 4x^3 + 5x^4

        // 2回右シフト: (1 + 2x + 3x^2 + 4x^3 + 5x^4) / x^2 = 3 + 4x + 5x^2
        let shr_f = f.clone() >> 2;
        assert_eq!(shr_f.coeffs, vec![Mint::new(3), Mint::new(4), Mint::new(5)]);

        // 0回右シフト
        let shr_f0 = f.clone() >> 0;
        assert_eq!(
            shr_f0.coeffs,
            vec![
                Mint::new(1),
                Mint::new(2),
                Mint::new(3),
                Mint::new(4),
                Mint::new(5)
            ]
        );

        // シフト量が多すぎる
        let shr_f_too_much = f.clone() >> 10;
        assert_eq!(shr_f_too_much.coeffs, vec![]);

        // 空のFPS
        let empty = Fps::new(vec![]);
        let shr_empty = empty >> 5;
        assert_eq!(shr_empty.coeffs, vec![]);
    }

    #[test]
    fn test_eval() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3)]); // f(x) = 1 + 2x + 3x^2

        // f(0) = 1
        let eval0 = f.eval(Mint::new(0));
        assert_eq!(eval0, Mint::new(1));

        // f(1) = 1 + 2 + 3 = 6
        let eval1 = f.eval(Mint::new(1));
        assert_eq!(eval1, Mint::new(6));

        // f(2) = 1 + 2*2 + 3*2^2 = 1 + 4 + 12 = 17
        let eval2 = f.eval(Mint::new(2));
        assert_eq!(eval2, Mint::new(17));

        // 空のFPS
        let empty = Fps::new(vec![]);
        let eval_empty = empty.eval(Mint::new(100));
        assert_eq!(eval_empty, Mint::new(0));

        // 定数FPS
        let constant = Fps::new(vec![Mint::new(5)]);
        let eval_constant = constant.eval(Mint::new(100));
        assert_eq!(eval_constant, Mint::new(5));
    }

    #[test]
    fn test_get() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(3)]); // f(x) = 1 + 2x + 3x^2

        // 存在する係数
        assert_eq!(f.get(0), Mint::new(1));
        assert_eq!(f.get(1), Mint::new(2));
        assert_eq!(f.get(2), Mint::new(3));

        // 配列外の係数 (小さいインデックス)
        assert_eq!(f.get(3), Mint::new(0));
        assert_eq!(f.get(4), Mint::new(0));

        // 空のFPS
        let empty = Fps::new(vec![]);
        assert_eq!(empty.get(0), Mint::new(0));
        assert_eq!(empty.get(5), Mint::new(0));
    }

    #[test]
    fn test_div_simple() {
        // (1 + 5x + 6x^2) / (1 + 2x) = 1 + 3x
        let f = Fps::new(vec![Mint::new(1), Mint::new(5), Mint::new(6)]);
        let g = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let q = f / &g;
        assert_eq!(q.coeffs, vec![Mint::new(1), Mint::new(3)]);
    }

    #[test]
    fn test_div_trailing_zeros() {
        // 除式に末尾のゼロが含まれる場合の処理（b_coeffs.pop()）をテストします。
        // (1 + 2x + x^2) / (1 + x + 0x^2) = 1 + x
        let f = Fps::new(vec![Mint::new(1), Mint::new(2), Mint::new(1)]);
        let g = Fps::new(vec![Mint::new(1), Mint::new(1), Mint::new(0)]);
        let q = f / &g;
        assert_eq!(q.coeffs, vec![Mint::new(1), Mint::new(1)]);
    }

    #[test]
    #[ignore]
    fn test_div_random() {
        // 除式の長さが 64 以下（長除算）と 64 超（NTTベース）の両方のケースをテストします。
        // A = Q * B + R と B から、商 Q が正しく得られるかを確認します。
        let mut rng = StdRng::from_os_rng();
        for _ in 0..500 {
            let m_orig = rng.random_range(1..=100);
            let q_len_orig = rng.random_range(1..=100);

            let q_coeffs: Vec<Mint> = (0..q_len_orig)
                .map(|_| Mint::new(rng.random_range(-3..=3)))
                .collect();
            let b_coeffs: Vec<Mint> = (0..m_orig)
                .map(|_| Mint::new(rng.random_range(-3..=3)))
                .collect();

            let q = Fps::new(q_coeffs);
            let b = Fps::new(b_coeffs);

            if b.coeffs.is_empty() {
                // 除式が実質的にゼロの場合はスキップ
                continue;
            }
            let m_eff = b.coeff_len();

            // 余り R (deg(R) < deg(B_eff)) をランダムに生成
            let r_len = rng.random_range(0..m_eff);
            let r_coeffs: Vec<Mint> = (0..r_len)
                .map(|_| Mint::new(rng.random_range(-3..=3)))
                .collect();
            let r = Fps::new(r_coeffs);

            let a = (q.clone() * &b) + &r; // A = Q * B + R

            let res_q = a.div_polynomial(&b);

            assert_eq!(
                res_q.coeffs, q.coeffs,
                "Division failed for m_orig={}, m_eff={}\nA: {:?}\nB: {:?}",
                m_orig, m_eff, a.coeffs, b.coeffs
            );
        }
    }

    #[test]
    fn test_div_with_remainder() {
        // (x^2 + x + 1) / (x + 1) = x (remainder 1)
        // f(x) = 1 + x + x^2
        // g(x) = 1 + x
        let f = Fps::new(vec![Mint::new(1), Mint::new(1), Mint::new(1)]);
        let g = Fps::new(vec![Mint::new(1), Mint::new(1)]);
        let q = f.clone() / &g;
        assert_eq!(q.coeffs, vec![Mint::new(0), Mint::new(1)]); // x
        let r = f % &g;
        assert_eq!(r.coeffs, vec![Mint::new(1)]); // 1
    }

    #[test]
    fn test_div_by_constant() {
        // (2 + 4x + 6x^2) / 2 = 1 + 2x + 3x^2
        let f = Fps::new(vec![Mint::new(2), Mint::new(4), Mint::new(6)]);
        let g = Fps::new(vec![Mint::new(2)]);
        let q = f / &g;
        assert_eq!(q.coeffs, vec![Mint::new(1), Mint::new(2), Mint::new(3)]);
    }

    #[test]
    fn test_div_dividend_smaller_than_divisor() {
        // (1 + x) / (1 + x + x^2) = 0
        let f = Fps::new(vec![Mint::new(1), Mint::new(1)]);
        let g = Fps::new(vec![Mint::new(1), Mint::new(1), Mint::new(1)]);
        let q = f / &g;
        assert!(q.coeffs.is_empty());
    }

    #[test]
    fn test_div_zero_dividend() {
        // 0 / (1 + x) = 0
        let f = Fps::zero();
        let g = Fps::new(vec![Mint::new(1), Mint::new(1)]);
        let q = f / &g;
        assert!(q.coeffs.is_empty());
    }

    #[test]
    #[should_panic(expected = "Division by zero polynomial")]
    fn test_div_panic_zero_divisor() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(1)]);
        let g = Fps::zero();
        let _ = f / &g;
    }

    #[test]
    fn test_rem_zero_remainder() {
        // (1 + 5x + 6x^2) % (1 + 2x) = 0
        let f = Fps::new(vec![Mint::new(1), Mint::new(5), Mint::new(6)]);
        let g = Fps::new(vec![Mint::new(1), Mint::new(2)]);
        let r = f % &g;
        assert!(r.coeffs.is_empty());
    }

    #[test]
    fn test_rem_dividend_smaller_than_divisor() {
        // (1 + x) % (1 + x + x^2) = 1 + x
        let f = Fps::new(vec![Mint::new(1), Mint::new(1)]);
        let g = Fps::new(vec![Mint::new(1), Mint::new(1), Mint::new(1)]);
        let r = f % &g;
        assert_eq!(r.coeffs, vec![Mint::new(1), Mint::new(1)]);
    }

    #[test]
    fn test_rem_zero_dividend() {
        // 0 % (1 + x) = 0
        let f = Fps::zero();
        let g = Fps::new(vec![Mint::new(1), Mint::new(1)]);
        let r = f % &g;
        assert!(r.coeffs.is_empty());
    }

    #[test]
    #[should_panic(expected = "Division by zero polynomial")]
    fn test_rem_panic_zero_divisor() {
        let f = Fps::new(vec![Mint::new(1), Mint::new(1)]);
        let g = Fps::zero();
        let _ = f % &g;
    }
}
