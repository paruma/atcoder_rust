use cargo_snippet::snippet;

#[snippet]
/// 実数領域における三分探索を行い、単峰な関数（下に凸など）の最小値を与える引数 `x` を返す。
///
/// 200回の反復を行うことで、元の区間幅の (2/3)^200 倍（約 6.0 * 10^-36 程度）の精度で値を求める。
///
/// ## 単峰性の前提
/// 探索範囲 `[l, r]` において、関数 `f` が単峰（値が減少してから増加する）である必要がある。
///
/// ## Arguments
/// * `l`: 探索範囲の下限
/// * `r`: 探索範囲の上限
/// * `f`: 評価関数
///
/// ## Return
/// 最小値を与える `x` の近似値を返す。
pub fn ternary_search<T, F>(mut l: f64, mut r: f64, mut f: F) -> f64
where
    T: PartialOrd,
    F: FnMut(f64) -> T,
{
    assert!(l <= r);
    const NUM_ITERATION: i64 = 200;
    for _ in 0..NUM_ITERATION {
        let ml = (l * 2.0 + r) / 3.0;
        let mr = (l + r * 2.0) / 3.0;
        if f(ml) < f(mr) {
            r = mr;
        } else {
            l = ml;
        }
    }
    (l + r) / 2.0
}

#[snippet]
/// 整数領域における三分探索を行い、単峰な関数（下に凸など）の最小値を与える引数 `x` を返す。
///
/// ## 単峰性の前提
/// 探索範囲 `[l, r]` において、関数 `f` が単峰（値が減少してから増加する）である必要がある。
///
/// ## Arguments
/// * `l`: 探索範囲の下限
/// * `r`: 探索範囲の上限
/// * `f`: 評価関数
///
/// ## Return
/// 最小値を与える `x` を返す。
pub fn ternary_search_i64<T, F>(mut l: i64, mut r: i64, mut f: F) -> i64
where
    T: PartialOrd,
    F: FnMut(i64) -> T,
{
    assert!(l <= r);
    // r - l > 2 でもよいが、誤差を考慮し、広めに候補を残す
    while r - l > 5 {
        let m1 = l + (r - l) / 3;
        let m2 = r - (r - l) / 3;
        if f(m1) < f(m2) {
            r = m2;
        } else {
            l = m1;
        }
    }

    // 残った候補 [l, l+1, ..., r] の中から最小値を探す
    let mut min_x = l;
    let mut min_val = f(l);
    for x in l + 1..=r {
        let val = f(x);
        if val < min_val {
            min_val = val;
            min_x = x;
        }
    }
    min_x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_search() {
        // f(x) = (x - 2.0)^2 は x = 2.0 で最小値 0.0 をとる
        let res = ternary_search(-10.0, 10.0, |x| (x - 2.0).powi(2));
        assert!((res - 2.0).abs() < 1e-15);

        // f(x) = |x + 5.0| は x = -5.0 で最小値 0.0 をとる
        let res = ternary_search(-10.0, 10.0, |x| (x + 5.0).abs());
        assert!((res - (-5.0)).abs() < 1e-15);

        // f(x) = (x - 3.0)^2 + 0.5 を整数に丸めたもの (f64 -> i64)
        // (x - 3.0)^2 + 0.5 < 1.0 => (x - 3.0)^2 < 0.5 => |x - 3.0| < sqrt(0.5)
        // よって最小値 0 を与える x の範囲は (3 - sqrt(0.5), 3 + sqrt(0.5))
        let res = ternary_search(0.0, 10.0, |x| ((x - 3.0).powi(2) + 0.5) as i64);
        let lower = 3.0 - 0.5f64.sqrt();
        let upper = 3.0 + 0.5f64.sqrt();
        assert!(lower - 1e-15 <= res && res <= upper + 1e-15);
    }

    #[test]
    fn test_ternary_search_i64() {
        // f(x) = (x - 10)^2
        let res = ternary_search_i64(-100, 100, |x| (x - 10).pow(2));
        assert_eq!(res, 10);

        // f(x) = |x + 5|
        let res = ternary_search_i64(-100, 100, |x| (x + 5).abs());
        assert_eq!(res, -5);

        // 平坦な区間がある場合: f(x) = 0 for x in [2, 4]
        // 2, 3, 4 のいずれかが返れば正解
        let res = ternary_search_i64(0, 10, |x| {
            if (2..=4).contains(&x) {
                0
            } else if x < 2 {
                2 - x
            } else {
                x - 4
            }
        });
        assert!((2..=4).contains(&res));
    }
}
