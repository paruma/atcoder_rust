#![allow(dead_code)]

fn combinations_with_replacement(n: usize, r: usize) -> Vec<Vec<usize>> {
    // n個のものからr個取る重複組合せ n+r-1 C r-1

    // seq が現在の状態、seq_list が結果の蓄積物
    fn rec(n: usize, r: usize, seq: &mut Vec<usize>, seq_list: &mut Vec<Vec<usize>>) {
        if seq.len() == r {
            // ここがforループの中のようなもの
            seq_list.push(seq.clone());
            return;
        }

        let max = seq.last().copied().unwrap_or(0);

        for i in max..n {
            seq.push(i);
            rec(n, r, seq, seq_list);
            seq.pop();
        }
    }

    let mut seq_list = vec![];
    rec(n, r, &mut vec![], &mut seq_list);
    seq_list
}

fn combinations(n: usize, r: usize) -> Vec<Vec<usize>> {
    // n個のものからr個取る組合せ nCr

    // seq が現在の状態、seq_list が結果の蓄積物
    fn rec(n: usize, r: usize, seq: &mut Vec<usize>, seq_list: &mut Vec<Vec<usize>>) {
        if seq.len() == r {
            // ここがforループの中のようなもの
            seq_list.push(seq.clone());
            return;
        }

        let begin = seq.last().copied().map(|x| x + 1).unwrap_or(0);

        // ループ範囲は具体例 (r=2 くらい) を考えるとわかる
        for i in begin..n - r + 1 + seq.len() {
            seq.push(i);
            rec(n, r, seq, seq_list);
            seq.pop();
        }
    }

    let mut seq_list = vec![];
    rec(n, r, &mut vec![], &mut seq_list);
    seq_list
}

// 組合せ全列挙の別実装
fn combinations2(n: usize, r: usize) -> Vec<Vec<usize>> {
    // n個のものからr個取る組合せ nCr

    // seq が現在の状態、seq_list が結果の蓄積物
    fn rec(n: usize, r: usize, i: usize, seq: &mut Vec<usize>, seq_list: &mut Vec<Vec<usize>>) {
        if i == n {
            // ここがforループの中のようなもの
            seq_list.push(seq.clone());
            return;
        }

        // 選ぶ
        if seq.len() != r {
            seq.push(i);
            rec(n, r, i + 1, seq, seq_list);
            seq.pop();
        }

        // 選ばない
        if seq.len() + (n - i) != r {
            rec(n, r, i + 1, seq, seq_list);
        }
    }

    let mut seq_list = vec![];
    rec(n, r, 0, &mut vec![], &mut seq_list);
    seq_list
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

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
            let actual = combinations2(n, r);
            let expected = (0..n).combinations(r).collect_vec();
            assert_eq!(actual, expected);
        }
        test(5, 3);
        test(5, 5);
        test(5, 0);
        test(0, 0);
    }
}
