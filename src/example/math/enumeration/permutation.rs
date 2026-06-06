#![allow(dead_code)]

// replace: 元に戻す

fn permutations_with_replacement(n: usize, r: usize) -> Vec<Vec<usize>> {
    // n個のものから重複を許してr個取る順列 n^r

    // これは非再帰（Stack）だと書きにくい？
    // seq が現在の状態、seq_list が結果の蓄積物
    fn rec(n: usize, r: usize, seq: &mut Vec<usize>, seq_list: &mut Vec<Vec<usize>>) {
        if seq.len() == r {
            // ここがforループの中のようなもの
            seq_list.push(seq.clone());
            return;
        }

        for i in 0..n {
            seq.push(i); // caller で状態(seq)を管理する
            rec(n, r, seq, seq_list);
            seq.pop();
        }
        // for x in 0..n{
        //     for y in 0..n{
        //         for z in 0..n{
        //             // ここの処理 が `if seq.len() == r` の中の処理に対応している
        //         }
        //     }
        // }
    }

    let mut seq_list = vec![];
    rec(n, r, &mut vec![], &mut seq_list);
    seq_list
}

fn permutations(n: usize, r: usize) -> Vec<Vec<usize>> {
    // n個のものからr個取る順列 nPr

    // seq, visited が現在の状態、seq_list が結果の蓄積物
    fn rec(
        n: usize,
        r: usize,
        seq: &mut Vec<usize>,
        visited: &mut Vec<bool>,
        seq_list: &mut Vec<Vec<usize>>,
    ) {
        if seq.len() == r {
            // ここがforループの中のようなもの
            seq_list.push(seq.clone());
            return;
        }

        for i in 0..n {
            if visited[i] {
                continue;
            }
            seq.push(i);
            visited[i] = true;
            rec(n, r, seq, visited, seq_list);
            visited[i] = false;
            seq.pop();
        }
    }

    let mut seq_list = vec![];
    rec(n, r, &mut vec![], &mut vec![false; n], &mut seq_list);
    seq_list
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
                std::iter::repeat_n(0..n, r)
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
}
