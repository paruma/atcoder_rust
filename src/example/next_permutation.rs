#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use permutohedron::LexicalPermutation;

    #[test]
    fn test_next_permutation() {
        // 同じものを含む順列を全列挙
        let mut xs = vec![0, 0, 1, 1, 2];
        let mut buf = vec![];
        while {
            buf.push(xs.clone());
            xs.next_permutation()
        } {}
        assert_eq!(buf.len(), 30); // 5!/(2! 2! 1!)
        let expected = vec![0, 0, 1, 1, 2]
            .iter()
            .copied()
            .permutations(5)
            .unique()
            .sorted()
            .collect_vec();
        assert_eq!(buf, expected);
    }
}
