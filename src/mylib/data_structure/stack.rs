use cargo_snippet::snippet;

#[snippet(prefix = "use mod_stack::*;")]
pub mod mod_stack {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Stack<T> {
        raw: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack { raw: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push(value)
        }

        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop()
        }

        pub fn peek(&self) -> Option<&T> {
            self.raw.last()
        }

        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }

        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }

    impl<T> Default for Stack<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mod_stack::*;

    #[allow(clippy::eq_op)]
    #[test]
    fn test_stack() {
        let mut s = Stack::<i64>::default();
        assert_eq!(s.peek(), None);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);

        s.push(1);
        assert_eq!(s.peek(), Some(&1));
        assert!(!s.is_empty());
        assert_eq!(s.len(), 1);

        s.push(2);
        assert_eq!(s.peek(), Some(&2));
        assert!(!s.is_empty());
        assert_eq!(s.len(), 2);

        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.peek(), Some(&1));
        assert!(!s.is_empty());
        assert_eq!(s.len(), 1);

        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.peek(), None);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);

        assert_eq!(s.pop(), None);
    }
}
