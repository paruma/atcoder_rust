#![allow(dead_code)]

struct DfsMultiLoop {
    // n個のものから重複を許してr個取る順列
    n: usize,
    r: usize,
    seq_list: Vec<Vec<usize>>,
}

impl DfsMultiLoop {
    fn new(n: usize, r: usize) -> Self {
        Self { n, r, seq_list: vec![] }
    }
    fn exec(&mut self) {
        self.exec_rec(&mut vec![]);
    }

    fn exec_rec(&mut self, seq: &mut Vec<usize>) {
        if seq.len() == self.r {
            // ここがforループの中のようなもの
            self.seq_list.push(seq.clone());
            return;
        }

        for i in 0..self.n {
            seq.push(i);
            self.exec_rec(seq);
            seq.pop();
        }
    }
}

// for x in 0..n{
//     for y in 0..n{
//         for z in 0..n{
//             // ここの処理 が `if seq.len() == self.r` の中の処理に対応している
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::DfsMultiLoop;

    #[test]
    fn test_dfs_multi_loop() {
        // 2つのものから重複を許して3つ取る
        let n = 2;
        let r = 3;

        let mut dfs = DfsMultiLoop::new(n, r);
        dfs.exec();
        let seq_list = dfs.seq_list;
        let expected = std::iter::repeat(0..n).take(r).multi_cartesian_product().collect_vec();
        assert_eq!(seq_list, expected);
    }
}
