use std::cmp::Ordering;

use cargo_snippet::snippet;

#[snippet]
// 参考: 整数のまま行う偏角ソート（行列式のあれです。）の実装バリエーションの検討とご紹介です。 - ブログ名 https://ngtkana.hatenablog.com/entry/2021/11/13/202103
/// x軸正の向きを0度として、反時計回りを正とする偏角で順序を決める。
/// (0, 0) は未考慮。
pub fn argcmp((x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> Ordering {
    ((y0, x0) < (0, 0))
        .cmp(&((y1, x1) < (0, 0)))
        .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::argcmp;

    #[test]
    fn test_argcmp() {
        let ps = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (2, 2), (-10, -1)];
        let expected = ps
            .iter()
            .copied()
            .sorted_by(|&(x0, y0), &(x1, y1)| {
                let arg0 = f64::atan2(y0 as f64, x0 as f64);
                let arg1 = f64::atan2(y1 as f64, x1 as f64);
                let arg0 = if arg0 < 0.0 {
                    arg0 + std::f64::consts::TAU
                } else {
                    arg0
                };
                let arg1 = if arg1 < 0.0 {
                    arg1 + std::f64::consts::TAU
                } else {
                    arg1
                };
                f64::total_cmp(&arg0, &arg1)
            })
            .collect_vec();
        let actual = ps
            .iter()
            .copied()
            .sorted_by(|&p, &q| argcmp(p, q))
            .collect_vec();

        assert_eq!(actual, expected);
    }
}
