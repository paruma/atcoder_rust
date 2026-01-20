use cargo_snippet::snippet;

#[snippet("define_queries_macro")]
#[macro_use]
pub mod define_queries {
    /// クエリ形式の入力を proconio::input! で読み込める enum を定義するマクロ。
    ///
    /// 出典： https://zenn.dev/magurofly/articles/6ee845bd5e385e
    ///
    /// # 利用例
    ///
    /// ```
    /// use mylib::define_queries;
    /// use proconio::marker::Usize1;
    ///
    /// define_queries! {
    ///     #[derive(Debug, PartialEq)]
    ///     enum Query: usize {
    ///         1 => Add { a: i64, b: i64 },
    ///         2 => Show { k: Usize1 },
    ///     }
    /// }
    /// ```
    #[macro_export]
    macro_rules! define_queries {
    ($( $(#[$attr:meta])* enum $enum_name:ident : $sig:ty { $( $pattern:pat => $variant:ident $( { $($name:ident : $marker:ty $(,)?),* } )? $(,)?),* } )*) => {
        $(
            $(#[$attr])*
            enum $enum_name {
                $(
                    $variant $( {
                        $( $name : <$marker as proconio::source::Readable>::Output ),*
                    } )?
                ),*
            }

            impl proconio::source::Readable for $enum_name {
                type Output = Self;
                fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self {
                    #![allow(unreachable_patterns)]
                    match <$sig as proconio::source::Readable>::read(source) {
                        $(
                            $pattern => $enum_name::$variant $( {
                                $( $name: <$marker as proconio::source::Readable>::read(source) ),*
                            } )?
                        ),*
                        , _ => unreachable!()
                    }
                }
            }
        )*
    }
}
}

#[cfg(test)]
mod tests {
    use proconio::input;
    use proconio::marker::Usize1;
    use proconio::source::once::OnceSource;

    define_queries! {
        #[derive(Debug, PartialEq)]
        enum Query: usize {
            1 => Add { a: i64, b: i64 },
            2 => Show { k: Usize1 },
            3 => Quit,
            4 => Reset {},
        }
    }

    #[test]
    fn test_define_queries() {
        let input_str = "1 10 20\n2 5\n3\n4";
        let source = OnceSource::from(input_str);

        input! {
            from source,
            queries: [Query; 4],
        }

        assert_eq!(queries[0], Query::Add { a: 10, b: 20 });
        assert_eq!(queries[1], Query::Show { k: 4 });
        assert_eq!(queries[2], Query::Quit);
        assert_eq!(queries[3], Query::Reset {});
    }
}
