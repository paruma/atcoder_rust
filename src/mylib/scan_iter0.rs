use cargo_snippet::snippet;

#[snippet(prefix = "use scan_iter::*;")]
pub mod scan_iter {
    #[derive(Clone)]
    pub struct Scanl<I, B, F> {
        iter: I,
        state: Option<B>,
        f: F,
    }

    impl<I, B, F> Scanl<I, B, F> {
        fn new(iter: I, init: B, f: F) -> Scanl<I, B, F> {
            Scanl {
                iter,
                state: Some(init),
                f,
            }
        }
    }

    impl<I, B, F> Iterator for Scanl<I, B, F>
    where
        B: Clone + Copy,
        I: Iterator,
        F: FnMut(&mut B, I::Item) -> B,
    {
        type Item = B;

        #[inline]
        fn next(&mut self) -> Option<B> {
            let retval = self.state?;
            let a_opt = self.iter.next();

            self.state = self
                .state
                .and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));

            Some(retval)
        }
    }

    pub trait IteratorExtScanLeft: Iterator + Sized {
        fn scanl<B, F>(self, init: B, f: F) -> Scanl<Self, B, F>
        where
            Self: Sized,
            F: FnMut(&mut B, Self::Item) -> B,
        {
            Scanl::new(self, init, f)
        }
    }

    impl<T: Iterator> IteratorExtScanLeft for T {}
}

#[cfg(test)]
mod test {

    use super::scan_iter::*;

    #[test]
    fn test_scan() {
        let xs: Vec<i32> = vec![1, 2, 3];
        let cumsum = xs
            .into_iter()
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<i32>>();

        assert_eq!(cumsum, vec![1, 3, 6]);
        // vec![0,1,3,6]が帰ってきてほしい。
    }

    #[test]
    fn test_scanl() {
        let xs: Vec<i32> = vec![1, 2, 3];
        let cumsum = xs.iter().scanl(0, |acc, x| *acc + *x).collect::<Vec<i32>>();

        assert_eq!(cumsum, vec![0, 1, 3, 6]);
    }
}
