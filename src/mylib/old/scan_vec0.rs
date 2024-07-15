pub mod scan_vec {

    pub fn scanl<A, B, F>(vec: &[A], init: B, mut f: F) -> Vec<B>
    where
        F: FnMut(&mut B, &A) -> B,
        B: Copy,
    {
        let mut ret: Vec<B> = Vec::new();
        let mut acc = init;
        ret.push(acc);

        for x in vec {
            acc = f(&mut acc, x);
            ret.push(acc);
        }

        ret
    }

    pub fn scanr<A, B, F>(vec: &[A], init: B, f: F) -> Vec<B>
    where
        F: FnMut(&mut B, &A) -> B,
        A: Clone,
        B: Copy,
    {
        let vec2 = vec.iter().cloned().rev().collect::<Vec<A>>();
        let vec3 = scanl(&vec2, init, f);
        vec3.iter().copied().rev().collect::<Vec<B>>()
    }

    pub fn cumsum<T>(vec: &[T]) -> Vec<T>
    where
        T: std::ops::Add + num::Zero + Copy,
    {
        scanl(vec, T::zero(), |acc, x| *acc + *x)
    }
}

#[cfg(test)]
mod test {

    use super::scan_vec::*;

    #[test]
    fn test_scanl() {
        let xs: Vec<i32> = vec![1, 2, 3];
        let cumsum1 = scanl(&xs, 0, |acc, x| *acc + *x);
        let cumsum2 = cumsum(&xs);

        let cumsum_right = scanr(&xs, 0, |acc, x| *acc + *x);

        assert_eq!(cumsum1, vec![0, 1, 3, 6]);
        assert_eq!(cumsum2, vec![0, 1, 3, 6]);
        assert_eq!(cumsum_right, vec![6, 5, 3, 0]);
    }
}
