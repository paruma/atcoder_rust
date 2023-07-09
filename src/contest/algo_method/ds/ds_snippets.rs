fn calc_mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

fn calc_variant(xs: &[f64]) -> f64 {
    let mean = calc_mean(xs);
    xs.iter().map(|x| (*x - mean) * (*x - mean)).sum::<f64>() / xs.len() as f64
}

fn calc_sd(xs: &[f64]) -> f64 {
    let variant = calc_variant(xs);
    variant.sqrt()
}

fn calc_median(xs: &[f64]) -> f64 {
    let n = xs.len();
    if n % 2 == 0 {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    } else {
        xs[n / 2]
    }
}

fn calc_quartile(xs: &[i64]) -> (f64, f64, f64) {
    let mut xs = xs.to_vec();
    xs.sort();
    let xs = xs.iter().map(|x| *x as f64).collect::<Vec<_>>();
    // xs = [0,1,2,3]の場合
    // [0,1], [2,3] が前半と後半
    // xs = [0,1,2,3,4]の場合
    // [0,1], [3,4] が前半と後半
    let xs_first_half = &xs[0..xs.len() / 2];
    let xs_last_half = &xs[(xs.len() + 1) / 2..];
    let q1 = calc_median(xs_first_half);
    let q2 = calc_median(&xs);
    let q3 = calc_median(xs_last_half);
    (q1, q2, q3)
}

fn calc_quartile_range(xs: &[i64]) -> f64 {
    let (q1, _q2, q3) = calc_quartile(xs);
    q3 - q1
}

fn calc_covariance(xs: &[f64], ys: &[f64]) -> f64 {
    assert_eq!(xs.len(), ys.len());
    let n = xs.len();
    let xs_mean = calc_mean(&xs);
    let ys_mean = calc_mean(&ys);
    xs.iter()
        .zip(ys.iter())
        .map(|(&x, &y)| (x - xs_mean) * (y - ys_mean))
        .sum::<f64>()
        / n as f64
}

fn calc_correlation_coefficient(xs: &[f64], ys: &[f64]) -> f64 {
    let covariance = calc_covariance(xs, ys);
    let xs_sd = calc_sd(xs);
    let ys_sd = calc_sd(ys);
    covariance / (xs_sd * ys_sd)
}
