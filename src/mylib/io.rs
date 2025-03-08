use cargo_snippet::snippet;

#[snippet]
pub mod impl_readable_for_enum {
    /// 利用例
    /// ```
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// enum Query {
    ///     Move { p: usize, h: usize },
    ///     Swap { h1: usize, h2: usize },
    ///     Output { p: usize },
    /// }
    ///
    /// impl_readable_for_enum! {
    ///     Query {
    ///         1 => Move { p: Usize1, h: Usize1 },
    ///         2 => Swap { h1: Usize1, h2: Usize1 },
    ///         3 => Output { p: Usize1 },
    ///     }
    /// }
    /// ```
    #[macro_export]
    macro_rules! impl_readable_for_enum {
        ($enum_name:ident {
            $($idx:literal => $variant:ident $( { $($field:ident : $ty:ty),* } )? ),* $(,)?
        }) => {
            impl proconio::source::Readable for $enum_name {
                type Output = $enum_name;
                fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> $enum_name {
                    input! { from &mut *source, t: usize }
                    match t {
                        $(
                            $idx => {
                                impl_readable_for_enum!(@read_variant source, $enum_name, $variant $( { $($field: $ty),* } )? )
                            }
                        ),*,
                        _ => unreachable!(),
                    }
                }
            }
        };

        (@read_variant $source:ident, $enum_name:ident, $variant:ident { $($field:ident : $ty:ty),* } ) => {{
            input! { from &mut *$source, $($field: $ty),* };
            $enum_name::$variant { $($field),* }
        }};
        (@read_variant $source:ident, $enum_name:ident, $variant:ident) => {{
            $enum_name::$variant
        }};
    }
}
