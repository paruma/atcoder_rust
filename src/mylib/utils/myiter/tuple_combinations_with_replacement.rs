use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use tuple_combinations_with_replacement::*;")]
pub mod tuple_combinations_with_replacement {
    use std::iter::{Fuse, FusedIterator};
    use std::marker::PhantomData;

    /// 重複を許して固定長タプルを列挙するイテレータである。
    #[derive(Clone, Debug)]
    #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
    pub struct TupleCombinationsWithReplacement<I, T>
    where
        I: Iterator,
        T: HasCombinationWithReplacement<I>,
    {
        iter: T::Combination,
        _marker: PhantomData<I>,
    }

    pub trait HasCombinationWithReplacement<I>: Sized {
        type Combination: From<I> + Iterator<Item = Self>;
    }

    pub trait IteratorTupleCombinationsWithReplacement: Iterator + Sized {
        /// 重複を許す組合せを、タプルとして列挙する。
        ///
        /// 入力中での位置が同じ要素を複数回選べる。出力順は入力順に対する辞書順である。
        fn tuple_combinations_with_replacement<T>(self) -> TupleCombinationsWithReplacement<Self, T>
        where
            Self: Clone,
            Self::Item: Clone,
            T: HasCombinationWithReplacement<Self>,
        {
            TupleCombinationsWithReplacement {
                iter: T::Combination::from(self),
                _marker: PhantomData,
            }
        }
    }

    impl<T: Iterator> IteratorTupleCombinationsWithReplacement for T {}

    impl<I, T> Iterator for TupleCombinationsWithReplacement<I, T>
    where
        I: Iterator,
        T: HasCombinationWithReplacement<I>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }

        fn count(self) -> usize {
            self.iter.count()
        }

        fn fold<B, F>(self, init: B, f: F) -> B
        where
            F: FnMut(B, Self::Item) -> B,
        {
            self.iter.fold(init, f)
        }
    }

    impl<I, T> FusedIterator for TupleCombinationsWithReplacement<I, T>
    where
        I: FusedIterator,
        T: HasCombinationWithReplacement<I>,
    {
    }

    #[derive(Clone, Debug)]
    pub struct Tuple1CombinationWithReplacement<I> {
        iter: I,
    }

    impl<I> From<I> for Tuple1CombinationWithReplacement<I> {
        fn from(iter: I) -> Self {
            Self { iter }
        }
    }

    impl<I: Iterator> Iterator for Tuple1CombinationWithReplacement<I> {
        type Item = (I::Item,);

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|x| (x,))
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }

        fn count(self) -> usize {
            self.iter.count()
        }
    }

    impl<I: Iterator> HasCombinationWithReplacement<I> for (I::Item,) {
        type Combination = Tuple1CombinationWithReplacement<I>;
    }

    macro_rules! impl_tuple_combination_with_replacement {
        ($combination:ident $previous:ident; $($item:ident)*) => {
            #[derive(Clone, Debug)]
            pub struct $combination<I: Iterator> {
                item: Option<I::Item>,
                iter: I,
                combination: $previous<I>,
            }

            impl<I: Iterator + Clone> From<I> for $combination<I> {
                fn from(mut iter: I) -> Self {
                    let combination = iter.clone().into();
                    let item = iter.next();
                    Self {
                        item,
                        iter,
                        combination,
                    }
                }
            }

            impl<I: Iterator + Clone> From<I> for $combination<Fuse<I>> {
                fn from(iter: I) -> Self {
                    Self::from(iter.fuse())
                }
            }

            impl<I, A> Iterator for $combination<I>
            where
                I: Iterator<Item = A> + Clone,
                A: Clone,
            {
                type Item = (A, $(replace_ident!($item, A)),*);

                fn next(&mut self) -> Option<Self::Item> {
                    if let Some(($($item,)*)) = self.combination.next() {
                        let head = self.item.clone().unwrap();
                        Some((head, $($item),*))
                    } else {
                        let next_combination = self.iter.clone().into();
                        self.item = self.iter.next();
                        self.item.clone().and_then(|head| {
                            self.combination = next_combination;
                            self.combination
                                .next()
                                .map(|($($item,)*)| (head, $($item),*))
                        })
                    }
                }
            }

            impl<I, A> HasCombinationWithReplacement<I> for (A, $(replace_ident!($item, A)),*)
            where
                I: Iterator<Item = A> + Clone,
                I::Item: Clone,
            {
                type Combination = $combination<Fuse<I>>;
            }
        };
    }

    macro_rules! replace_ident {
        ($_ident:ident, $replacement:ty) => {
            $replacement
        };
    }

    impl_tuple_combination_with_replacement!(Tuple2CombinationWithReplacement Tuple1CombinationWithReplacement; a);
    impl_tuple_combination_with_replacement!(Tuple3CombinationWithReplacement Tuple2CombinationWithReplacement; a b);
    impl_tuple_combination_with_replacement!(Tuple4CombinationWithReplacement Tuple3CombinationWithReplacement; a b c);
    impl_tuple_combination_with_replacement!(Tuple5CombinationWithReplacement Tuple4CombinationWithReplacement; a b c d);
    impl_tuple_combination_with_replacement!(Tuple6CombinationWithReplacement Tuple5CombinationWithReplacement; a b c d e);
    impl_tuple_combination_with_replacement!(Tuple7CombinationWithReplacement Tuple6CombinationWithReplacement; a b c d e f);
    impl_tuple_combination_with_replacement!(Tuple8CombinationWithReplacement Tuple7CombinationWithReplacement; a b c d e f g);
    impl_tuple_combination_with_replacement!(Tuple9CombinationWithReplacement Tuple8CombinationWithReplacement; a b c d e f g h);
    impl_tuple_combination_with_replacement!(Tuple10CombinationWithReplacement Tuple9CombinationWithReplacement; a b c d e f g h i);
    impl_tuple_combination_with_replacement!(Tuple11CombinationWithReplacement Tuple10CombinationWithReplacement; a b c d e f g h i j);
    impl_tuple_combination_with_replacement!(Tuple12CombinationWithReplacement Tuple11CombinationWithReplacement; a b c d e f g h i j k);
}

#[cfg(test)]
mod tests {
    use super::tuple_combinations_with_replacement::*;

    #[test]
    fn test_tuple_combinations_with_replacement_pair() {
        let actual = (1..4)
            .tuple_combinations_with_replacement::<(_, _)>()
            .collect::<Vec<_>>();

        assert_eq!(actual, vec![(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (3, 3)]);
    }

    #[test]
    fn test_tuple_combinations_with_replacement_triple() {
        let actual = (1..3)
            .tuple_combinations_with_replacement::<(_, _, _)>()
            .collect::<Vec<_>>();

        assert_eq!(actual, vec![(1, 1, 1), (1, 1, 2), (1, 2, 2), (2, 2, 2)]);
    }

    #[test]
    fn test_tuple_combinations_with_replacement_single() {
        let actual = (1..4)
            .tuple_combinations_with_replacement::<(_,)>()
            .collect::<Vec<_>>();

        assert_eq!(actual, vec![(1,), (2,), (3,)]);

        let count = (1..4).tuple_combinations_with_replacement::<(_,)>().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_tuple_combinations_with_replacement_count_and_fold() {
        let count = (1..4)
            .tuple_combinations_with_replacement::<(_, _)>()
            .count();
        assert_eq!(count, 6);

        let sum = (1..4)
            .tuple_combinations_with_replacement::<(_, _)>()
            .fold(0, |acc, (x, y)| acc + x + y);
        assert_eq!(sum, 24);
    }

    #[test]
    fn test_tuple_combinations_with_replacement_empty() {
        let actual = std::iter::empty::<i32>()
            .tuple_combinations_with_replacement::<(_, _)>()
            .collect::<Vec<_>>();

        assert!(actual.is_empty());
    }

    #[test]
    fn test_tuple_combinations_with_replacement_preserves_duplicate_positions() {
        let actual = [1, 1]
            .into_iter()
            .tuple_combinations_with_replacement::<(_, _)>()
            .collect::<Vec<_>>();

        assert_eq!(actual, vec![(1, 1), (1, 1), (1, 1)]);
    }
}
