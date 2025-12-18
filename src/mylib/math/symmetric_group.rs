use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet(prefix = "use symmetric_group::*;")]
pub mod symmetric_group {
    /// 置換を巡回置換の積で表す
    pub fn make_cycles(ps: &[usize]) -> Vec<Vec<usize>> {
        let n = ps.len();
        let mut visited = vec![false; n];

        let mut cycles = vec![];

        for init in 0..n {
            if visited[init] {
                continue;
            }
            let mut cycle = vec![];
            let mut cur = init;
            while !visited[cur] {
                cycle.push(cur);
                visited[cur] = true;
                cur = ps[cur];
            }
            cycles.push(cycle);
        }
        cycles
    }
}

#[cfg(test)]
mod test_symmetric_group {
    use crate::mylib::math::symmetric_group::symmetric_group::make_cycles;

    #[test]
    fn test_make_cycles() {
        // 恒等置換
        assert_eq!(make_cycles(&[] as &[usize]), vec![] as Vec<Vec<usize>>);
        assert_eq!(make_cycles(&[0]), vec![vec![0]]);
        assert_eq!(make_cycles(&[0, 1, 2]), vec![vec![0], vec![1], vec![2]]);

        // 単一の巡回置換
        assert_eq!(make_cycles(&[1, 2, 0]), vec![vec![0, 1, 2]]);
        assert_eq!(make_cycles(&[1, 0]), vec![vec![0, 1]]);

        // 複数の巡回置換
        assert_eq!(make_cycles(&[1, 0, 3, 2]), vec![vec![0, 1], vec![2, 3]]);
        assert_eq!(make_cycles(&[2, 3, 0, 1]), vec![vec![0, 2], vec![1, 3]]);
    }
}
