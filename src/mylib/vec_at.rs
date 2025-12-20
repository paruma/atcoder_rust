use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use vec_at::*;")]
pub mod vec_at {
    use easy_ext::ext;

    #[ext(VecAt)]
    impl<T> Vec<T> {
        pub fn at(&self, index: i64) -> &T {
            &self[index as usize]
        }

        pub fn at_mut(&mut self, index: i64) -> &mut T {
            &mut self[index as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use super::vec_at::*;
    #[test]
    fn test_at() {
        let xs = vec![1, 2, 3, 4, 5];
        assert_eq!(xs.at(0), &1);
        assert_eq!(xs.at(1), &2);
        assert_eq!(xs.at(2), &3);
        assert_eq!(xs.at(3), &4);
        assert_eq!(xs.at(4), &5);
    }

    #[test]
    fn test_at_2d() {
        let xss = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]];

        assert_eq!(xss.at(1).at(2), &8);
    }

    #[test]
    fn test_at_mut() {
        let mut xs = vec![1, 2, 3, 4, 5];
        *xs.at_mut(2) = 100;
        assert_eq!(xs, vec![1, 2, 100, 4, 5]);
    }
}
