use std::ops::{Index, IndexMut};
#[derive(Clone, Debug, PartialEq, Eq)]
struct BoolDp {
    dp: Vec<bool>,
}
impl BoolDp {
    fn new(w: i64) -> BoolDp {
        BoolDp {
            dp: vec![false; (w + 1) as usize],
        }
    }
}

impl Index<i64> for BoolDp {
    type Output = bool;

    fn index(&self, index: i64) -> &Self::Output {
        if index >= 0 {
            &self.dp[index as usize]
        } else {
            &false
        }
    }
}

impl IndexMut<i64> for BoolDp {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        &mut self.dp[index as usize]
    }
}

#[allow(dead_code)]
/// xs の各要素を0個以上使って和を w にできるか
fn unlimited_subbag_sum(xs: &[i64], w: i64) -> bool {
    let mut dp = BoolDp::new(w);
    dp[0] = true;

    for j in 1..=w {
        dp[j] = xs.iter().copied().any(|x| dp[j - x]);
    }
    dp[w]
}

#[allow(dead_code)]
/// xs の各要素を0個以上使って和を w にできるか
fn unlimited_subbag_sum2(xs: &[i64], w: i64) -> bool {
    let n = xs.len();
    let mut dp = vec![BoolDp::new(w); n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=w {
            let choose = dp[i + 1][j - xs[i]];
            let no_choose = dp[i][j];
            dp[i + 1][j] = choose || no_choose;
        }
    }
    dp[n][w]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn test1() {
        assert_eq!(unlimited_subbag_sum(&[3, 5], 0), true);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 1), false);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 2), false);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 3), true);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 4), false);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 5), true);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 6), true);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 7), false);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 8), true);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 9), true);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 10), true);
        assert_eq!(unlimited_subbag_sum(&[3, 5], 11), true);

        assert_eq!(unlimited_subbag_sum2(&[3, 5], 0), true);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 1), false);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 2), false);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 3), true);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 4), false);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 5), true);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 6), true);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 7), false);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 8), true);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 9), true);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 10), true);
        assert_eq!(unlimited_subbag_sum2(&[3, 5], 11), true);
    }
}

//---------snippet---------
