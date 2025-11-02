use cargo_snippet::snippet;

#[snippet]
/// 転倒数 `#{(i, j) | i < j and xs[i] > xs[j]}` を求める
/// 計算量: O(m log m + n)
/// (m = max(xs), n = xs.len())
pub fn inversion_number(xs: &[usize]) -> i64 {
    use ac_library::FenwickTree;
    if xs.is_empty() {
        return 0;
    }
    let max_val = xs.iter().copied().max().unwrap();

    // 各値が今までに現れた回数を記録する
    let mut fenwick = FenwickTree::new(max_val + 1, 0_i64);
    let mut cnt = 0;
    for &x in xs {
        cnt += fenwick.sum(x + 1..); // 今までに見たxより大きい値の数
        fenwick.add(x, 1);
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::inversion_number;
    #[test]
    fn test_inversion_nubmer() {
        assert_eq!(inversion_number(&[3, 2, 1]), 3);
        assert_eq!(inversion_number(&[1, 2, 3, 4, 5, 6]), 0);
        assert_eq!(inversion_number(&[1, 6, 3, 4, 5, 2]), 7);
        assert_eq!(inversion_number(&[1, 1, 0, 0, 0, 0]), 8);
    }
}
