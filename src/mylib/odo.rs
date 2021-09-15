use cargo_snippet::snippet;

#[allow(unused_macros)]
macro_rules! odo {
        (let $p: pat = $e: expr ; $($t:tt)+)=>{
            { let $p = $e ; odo! {$($t)+} }
        };
        (let $p: ident : $ty: ty = $e: expr ; $($t:tt)+)=>{
            { let $p: $ty = $e ; odo! {$($t)+} }
        };
        ($i:ident <- $e:expr; $($t:tt)+) => {
            $e.and_then(move|$i| odo!{$($t)+})
        };
        ($e:expr; $($t:tt)+) => {
            $e.and_then(move|_| odo!{$($t)+})
        };
        (guard $e:expr; $($t:tt)+)=>{
            ($e).then(move|| odo!{$($t)+}).flatten()
        };
        ($e:expr) => {
            $e
        };
}

#[snippet("fn_guard")]
pub fn guard(p: bool) -> Option<()> {
    p.then(|| ())
}

#[cfg(test)]
mod test {
    use super::guard;

    #[test]
    fn test_odo2() {
        // 普通に即時関数使ったほうが安全な気がする(letとかはちゃんともとのRustの文法として使えるし、整形効くし。)
        let ans1 = odo! {
            a <- Some(3);
            b <- Some(4);
            let c: i32 = a + b;
            guard c < 5;
            Some(c)
        };

        assert_eq!(ans1, None);

        let ans2 = odo! {
            a <- Some(3);
            b <- Some(4);
            let c = a + b;
            guard c < 10;
            Some(c)
        };
        assert_eq!(ans2, Some(7));
    }
    #[test]
    fn test_odo() {
        let ans1 = odo! {
            a <- Some(3);
            b <- Some(4);
            Some(a + b)
        };

        assert_eq!(ans1, Some(7));

        let ans2 = odo! {
            a <- Some(3);
            b <- None as Option<i32>;
            Some(a + b)
        };

        assert_eq!(ans2, None);

        let ans3 = odo! {
            a <- None as Option<i32>;
            b <- Some(4);
            Some(a + b)
        };

        assert_eq!(ans3, None);

        let ans4 = odo! {
            a <- None as Option<i32>;
            b <- None as Option<i32>;
            Some(a + b)
        };

        assert_eq!(ans4, None);
    }

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
