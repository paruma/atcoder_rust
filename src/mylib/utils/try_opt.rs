use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[macro_use]
#[snippet]
pub mod try_opt {
    /// `?` 演算子を式・ブロックの中で使うためのマクロ。
    ///
    /// IIFE `(|| Some(...))()` の代替として用いる。
    /// 複文ブロックは `try` ブロックと同様に書け、最後の式が自動的に `Some` で包まれる。
    ///
    /// # Example
    ///
    /// ```
    /// # use mylib::try_opt;
    /// let dp = [[10_i64, 20, 30]];
    /// let neg_inf = i64::MIN / 2;
    ///
    /// // 単一式
    /// let item_p = 1_usize;
    /// let p = 2_usize;
    /// let val = try_opt!(dp[0][p.checked_sub(item_p)?]).unwrap_or(neg_inf);
    /// assert_eq!(val, 20);
    ///
    /// let p = 0_usize;
    /// let val = try_opt!(dp[0][p.checked_sub(item_p)?]).unwrap_or(neg_inf);
    /// assert_eq!(val, neg_inf);
    ///
    /// // 複文ブロック（try ブロックと同様に書ける）
    /// let f = |n: i32| if n > 0 { Some(n * 10) } else { None };
    /// let result = try_opt! {
    ///     let a = f(1)?;
    ///     let b = f(2)?;
    ///     a + b
    /// };
    /// assert_eq!(result, Some(30));
    ///
    /// let result = try_opt! {
    ///     let a = f(1)?;
    ///     let b = f(-1)?;
    ///     a + b
    /// };
    /// assert_eq!(result, None);
    /// ```
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! try_opt {
        ($e:expr) => {
            (|| Some($e))()
        };
        ($($t:tt)*) => {
            (|| Some({ $($t)* }))()
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_try_opt_some() {
        let x: Option<i32> = Some(3);
        let result = try_opt!(x? * 2);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_try_opt_none() {
        let x: Option<i32> = None;
        let result = try_opt!(x? * 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_opt_block_some() {
        let f = |n: i32| if n > 0 { Some(n * 10) } else { None };
        let result = try_opt! {
            let a = f(1)?;
            let b = f(2)?;
            a + b
        };
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_try_opt_block_none() {
        let f = |n: i32| if n > 0 { Some(n * 10) } else { None };
        let result = try_opt! {
            let a = f(1)?;
            let b = f(-1)?;
            a + b
        };
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_opt_checked_sub() {
        let dp = [[10_i64, 20, 30]];
        let neg_inf = i64::MIN / 2;
        let item_p = 1_usize;

        let p = 2_usize;
        assert_eq!(
            try_opt!(dp[0][p.checked_sub(item_p)?]).unwrap_or(neg_inf),
            20
        );

        let p = 0_usize;
        assert_eq!(
            try_opt!(dp[0][p.checked_sub(item_p)?]).unwrap_or(neg_inf),
            neg_inf
        );
    }
}
