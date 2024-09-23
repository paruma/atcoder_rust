use cargo_snippet::snippet;

#[snippet(prefix = "use mod_queue::*;")]
pub mod mod_queue {
    use std::collections::VecDeque;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_front(value)
        }

        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }

        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }

        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }

        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }

    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mod_queue::*;

    #[allow(clippy::eq_op)]
    #[test]
    fn test_queue() {
        let mut s = Queue::<i64>::default();
        assert_eq!(s.peek(), None);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);

        s.push(1);
        assert_eq!(s.peek(), Some(&1));
        assert!(!s.is_empty());
        assert_eq!(s.len(), 1);

        s.push(2);
        assert_eq!(s.peek(), Some(&1));
        assert!(!s.is_empty());
        assert_eq!(s.len(), 2);

        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.peek(), Some(&2));
        assert!(!s.is_empty());
        assert_eq!(s.len(), 1);

        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.peek(), None);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);

        assert_eq!(s.pop(), None);
    }
}
