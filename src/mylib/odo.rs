use cargo_snippet::snippet;

#[snippet("fn_guard")]
pub fn guard(p: bool) -> Option<()> {
    if p {
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::guard;

    #[test]
    fn test_guard() {
        let x_opt = Some(3);
        let y_opt = Some(4);
        let z_opt1 = || -> Option<_> {
            let x = x_opt?;
            let y = y_opt?;
            guard(x + y < 5)?;
            Some(x + y)
        }();

        let z_opt2 = || -> Option<_> {
            let x = x_opt?;
            let y = y_opt?;
            guard(x + y < 10)?;
            Some(x + y)
        }();

        assert_eq!(z_opt1, None);
        assert_eq!(z_opt2, Some(7));
    }
}

/*
参考
TeXitoi/rust-mdo: Monadic do notation for rust using macro and duck typing https://github.com/TeXitoi/rust-mdo
Rustでもモナドは実装できるのか？（再） - Don't Repeat Yourself https://blog-dry.com/entry/2020/12/25/130250
 */
