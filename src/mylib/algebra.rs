trait Monoid {
    fn mempty() -> Self;
    fn mappend(&self, rhs: &Self) -> Self;
    fn mconcat(vec: &[Self]) -> Self
    where
        Self: std::marker::Sized,
    {
        vec.iter().fold(Self::mempty(), |acc, x| acc.mappend(x))
    }
}

trait Group: Monoid {
    fn invert(&self) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Sum {
    get_sum: i32,
}

impl Monoid for Sum {
    fn mempty() -> Self {
        Sum { get_sum: 0 }
    }

    fn mappend(&self, rhs: &Self) -> Self {
        Sum {
            get_sum: self.get_sum + rhs.get_sum,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rf() {
        let xs = vec![1, 2, 3]
            .iter()
            .map(|x| Sum { get_sum: *x })
            .collect::<Vec<_>>();

        assert_eq!(Monoid::mconcat(&xs), Sum { get_sum: 6 });
    }
}
