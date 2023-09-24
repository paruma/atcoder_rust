use cargo_snippet::snippet;

/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
///
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
#[snippet]
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok); // |ok - ng| > 1 なので
        assert!(mid != ng); // 同じく
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

/// 指定された要素以上の値が現れる最初の位置を返す。
/// ## Arguments
/// * xs: 単調増加
///     * 単調増加でなくても、 `|i| key <= xs[i]` が単調ならOK
///
/// ## Return
/// `I = {i in 0..xs.len() | key <= xs[i]}` としたとき、`min I` を返す。
/// ただし、`I` が空の場合は `xs.len()` を返す
/// 戻り値は、区間 `0..=xs.len()` の間で返る。
#[snippet(include = "bin_search")]
pub fn lower_bound<T: PartialOrd>(xs: &[T], key: T) -> usize {
    let pred = |i: i64| key <= xs[i as usize];
    bin_search(xs.len() as i64, -1_i64, pred) as usize
}

/// 指定された要素より大きい値が現れる最初の位置を返す。
/// ## Arguments
/// * xs: 単調増加
///     * 単調増加でなくても、 `|i| key < xs[i]` が単調ならOK
///
/// ## Return
/// `I = {i in 0..xs.len() | key < xs[i]}` としたとき、`min I` を返す。
/// ただし、`I` が空の場合は `xs.len()` を返す
/// 戻り値は、区間 `0..=xs.len()` の間で返る。
#[snippet(include = "bin_search")]
pub fn upper_bound<T: PartialOrd>(xs: &[T], key: T) -> usize {
    let pred = |i: i64| key < xs[i as usize];
    bin_search(xs.len() as i64, -1_i64, pred) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 以下の関数 p: i64 -> bool を返す。
    ///
    /// * is_increasing == true の場合 p は単調増加、そうでない場合は p は単調減少である。
    /// * boundary で false と true が切り替わる。p(boundary) == true とする。
    ///
    /// ## example
    /// `create_predicate(0, 5, 3, true)` の場合、以下のようになる
    /// 
    /// | 0     | 1     | 2     | 3     | 4     |
    /// | ----- | ----- | ----- | ----- | ----- |
    /// | false | false | false | true  | true  |
    ///
    /// ## note
    /// FIXME: 論理結合している
    fn create_predicate(
        range_begin: i64,
        range_end: i64,
        boundary: i64,
        is_increasing: bool,
    ) -> Box<dyn Fn(i64) -> bool> {
        assert!(range_begin <= range_end);
        let p = move |i: i64| {
            if i < range_begin || range_end <= i {
                panic!()
            } else if is_increasing {
                boundary <= i
            } else {
                i <= boundary
            }
        };
        Box::new(p)
    }

    #[test]
    fn test_bin_search() {
        // 普通のケースのテスト
        struct TestCase {
            // 区間 [range_begin, range_end) を表す
            range_begin: i64,
            range_end: i64,
        }
        impl TestCase {
            fn new(range_begin: i64, range_end: i64) -> Self {
                assert!(range_begin <= range_end);
                TestCase {
                    range_begin,
                    range_end,
                }
            }
            fn test_inc(&self) {
                let ok = self.range_end;
                let ng = self.range_begin - 1;
                // [self.range_begin, self.range_end) -> bool のすべての単調増加関数に対してテストをする
                for boundary in self.range_begin..=self.range_end {
                    // boundary == self.range_begin の場合: すべて true
                    // boundary == self.range_end   の場合: すべて false
                    let p = create_predicate(self.range_begin, self.range_end, boundary, true);
                    assert_eq!(bin_search(ok, ng, p), boundary);
                }
            }
            fn test_dec(&self) {
                let ok = self.range_begin - 1;
                let ng = self.range_end;
                // [self.range_begin, self.range_end) -> bool のすべての単調減少関数に対してテストをする
                for boundary in self.range_begin - 1..self.range_end {
                    // boundary == self.range_begin - 1 の場合: すべて false
                    // boundary == self.range_end       の場合: すべて true
                    let p = create_predicate(self.range_begin, self.range_end, boundary, false);
                    assert_eq!(bin_search(ok, ng, p), boundary);
                }
            }
        }
        let test_cases = [
            // (range_begin, range_end)
            (0, 10),    // 定義域が非負
            (-20, -10), // 定義域が負
            (-20, 10),  // 定義域が正負をまたがる
            (-10, 20),  // (同上)
            (4, 5),     // 定義域が1点
            (5, 5),     // 定義域が空
        ]
        .map(|(range_begin, range_end)| TestCase::new(range_begin, range_end));

        for test_case in test_cases {
            test_case.test_inc();
            test_case.test_dec();
        }
    }

    #[test]
    fn test_lower_bound() {
        assert_eq!(lower_bound(&[1, 2, 2, 3], 2), 1); // key が配列にある場合
        assert_eq!(lower_bound(&[1, 3, 3, 4], 2), 1); // key が配列にない場合
        assert_eq!(lower_bound(&[1, 1, 2, 2], 1), 0); // 答えが左端
        assert_eq!(lower_bound(&[1, 1, 2, 2], 0), 0); // 答えが左端
        assert_eq!(lower_bound(&[1, 1, 2, 2], 10), 4); // 答えが右端
        assert_eq!(lower_bound(&[], 2), 0); // 空列
    }

    #[test]
    fn test_upper_bound() {
        assert_eq!(upper_bound(&[1, 2, 2, 3], 2), 3); // key が配列にある場合
        assert_eq!(upper_bound(&[1, 3, 3, 4], 2), 1); // key が配列にない場合
        assert_eq!(upper_bound(&[1, 1, 2, 2], 1), 2); // key が配列にある場合
        assert_eq!(lower_bound(&[1, 1, 2, 2], 0), 0); // 答えが左端 
        assert_eq!(upper_bound(&[1, 1, 2, 2], 10), 4); // 答えが右端
        assert_eq!(upper_bound(&[], 2), 0); // 空列
    }
}
