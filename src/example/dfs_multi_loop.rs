#![allow(dead_code)]

// replace: 元に戻す

fn permutations_with_replacement(n: usize, r: usize) -> Vec<Vec<usize>> {
    struct DfsPermutationsWithReplacement {
        // n個のものから重複を許してr個取る順列 n^r
        n: usize,
        r: usize,
    }

    // これは非再帰（Stack）だと書きにくい？
    impl DfsPermutationsWithReplacement {
        fn new(n: usize, r: usize) -> Self {
            Self { n, r }
        }

        /// 計算量: O(n^r)
        fn exec(&self) -> Vec<Vec<usize>> {
            let mut seq_list = vec![];
            self.exec_rec(&mut vec![], &mut seq_list);
            seq_list
        }

        // seq が現在の状態、seq_list が結果の蓄積物
        fn exec_rec(&self, seq: &mut Vec<usize>, seq_list: &mut Vec<Vec<usize>>) {
            if seq.len() == self.r {
                // ここがforループの中のようなもの
                seq_list.push(seq.clone());
                return;
            }

            for i in 0..self.n {
                seq.push(i); // caller で状態(seq)を管理する
                self.exec_rec(seq, seq_list);
                seq.pop();
            }
        }
        // for x in 0..n{
        //     for y in 0..n{
        //         for z in 0..n{
        //             // ここの処理 が `if seq.len() == self.r` の中の処理に対応している
        //         }
        //     }
        // }
    }
    let dfs = DfsPermutationsWithReplacement::new(n, r);
    dfs.exec()
}

fn permutations(n: usize, r: usize) -> Vec<Vec<usize>> {
    struct DfsPermutations {
        // n個のものからr個取る順列 nPr
        n: usize,
        r: usize,
    }

    impl DfsPermutations {
        fn new(n: usize, r: usize) -> Self {
            Self { n, r }
        }

        fn exec(&self) -> Vec<Vec<usize>> {
            let mut seq_list = vec![];
            self.exec_rec(&mut vec![], &mut vec![false; self.n], &mut seq_list);
            seq_list
        }

        // seq, visited が現在の状態、seq_list が結果の蓄積物
        fn exec_rec(
            &self,
            seq: &mut Vec<usize>,
            visited: &mut Vec<bool>,
            seq_list: &mut Vec<Vec<usize>>,
        ) {
            if seq.len() == self.r {
                // ここがforループの中のようなもの
                seq_list.push(seq.clone());
                return;
            }

            for i in 0..self.n {
                if visited[i] {
                    continue;
                }
                seq.push(i);
                visited[i] = true;
                self.exec_rec(seq, visited, seq_list);
                visited[i] = false;
                seq.pop();
            }
        }
    }
    let dfs = DfsPermutations::new(n, r);
    dfs.exec()
}

fn combinations_with_replacement(n: usize, r: usize) -> Vec<Vec<usize>> {
    struct DfsCombinationsWithReplacement {
        // n個のものからr個取る重複組合せ n+r-1 C r-1
        n: usize,
        r: usize,
    }

    impl DfsCombinationsWithReplacement {
        fn new(n: usize, r: usize) -> Self {
            Self { n, r }
        }

        fn exec(&self) -> Vec<Vec<usize>> {
            let mut seq_list = vec![];
            self.exec_rec(&mut vec![], &mut seq_list);
            seq_list
        }

        // seq が現在の状態、seq_list が結果の蓄積物
        fn exec_rec(&self, seq: &mut Vec<usize>, seq_list: &mut Vec<Vec<usize>>) {
            if seq.len() == self.r {
                // ここがforループの中のようなもの
                seq_list.push(seq.clone());
                return;
            }

            let max = seq.last().copied().unwrap_or(0);

            for i in max..self.n {
                seq.push(i);
                self.exec_rec(seq, seq_list);
                seq.pop();
            }
        }
    }
    let dfs = DfsCombinationsWithReplacement::new(n, r);
    dfs.exec()
}

fn combinations(n: usize, r: usize) -> Vec<Vec<usize>> {
    struct DfsCombinations {
        // n個のものからr個取る組合せ nCr
        n: usize,
        r: usize,
    }

    impl DfsCombinations {
        fn new(n: usize, r: usize) -> Self {
            Self { n, r }
        }

        fn exec(&self) -> Vec<Vec<usize>> {
            let mut seq_list = vec![];
            self.exec_rec(&mut vec![], &mut seq_list);
            seq_list
        }

        // seq が現在の状態、seq_list が結果の蓄積物
        fn exec_rec(&self, seq: &mut Vec<usize>, seq_list: &mut Vec<Vec<usize>>) {
            if seq.len() == self.r {
                // ここがforループの中のようなもの
                seq_list.push(seq.clone());
                return;
            }

            let begin = seq.last().copied().map(|x| x + 1).unwrap_or(0);

            for i in begin..self.n - self.r + 1 + seq.len() {
                seq.push(i);
                self.exec_rec(seq, seq_list);
                seq.pop();
            }
        }
    }
    let dfs = DfsCombinations::new(n, r);
    dfs.exec()
}
#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_permutations_with_replacement() {
        // n個から重複を許してr個取って並べる順列
        fn expected(n: usize, r: usize) -> Vec<Vec<usize>> {
            if r == 0 {
                // itertools の multi_cartesian_product では
                // 0個の直積は空集合という扱いになってしまっているバグがある。
                // （なお、itertools 0.13.0 で解決された。）
                // https://github.com/rust-itertools/itertools/issues/337
                vec![vec![]]
            } else {
                std::iter::repeat(0..n)
                    .take(r)
                    .multi_cartesian_product()
                    .collect_vec()
            }
        }
        fn test(n: usize, r: usize) {
            assert_eq!(permutations_with_replacement(n, r), expected(n, r));
        }

        test(2, 3); // 2^3
        test(0, 2); // 0^2
        test(2, 0); // 0^0
        test(0, 0); // 0^0
    }

    #[test]
    fn test_permutations() {
        // n 個から r 個取って並べる順列
        fn test(n: usize, r: usize) {
            let actual = permutations(n, r);
            let expected = (0..n).permutations(r).collect_vec();
            dbg!(&actual);
            assert_eq!(actual, expected);
        }

        test(2, 3);
        test(2, 0);
        test(0, 0);
    }

    #[test]
    fn test_combinations_with_replacement() {
        // n 個の中から重複を許して r 個選ぶ選び方
        fn test(n: usize, r: usize) {
            let actual = combinations_with_replacement(n, r);
            let expected = (0..n).combinations_with_replacement(r).collect_vec();
            assert_eq!(actual, expected);
        }
        test(5, 3);
        test(5, 6);
        test(5, 0);
        test(0, 0);
    }

    #[test]
    fn test_combinations() {
        // n 個の中から r 個を選ぶ選び方
        fn test(n: usize, r: usize) {
            let actual = combinations(n, r);
            let expected = (0..n).combinations(r).collect_vec();
            assert_eq!(actual, expected);
        }
        test(5, 3);
        test(5, 5);
        test(5, 0);
        test(0, 0);
    }
}
