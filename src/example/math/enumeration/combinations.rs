#![allow(dead_code)]

/// n 個のものからr個取る組合せ nCr 個を全列挙する。
/// 各組合せは長さ `r` の `Vec<usize>` (ソート済) で表し、組合せの全列挙を `Vec<Vec<usize>>` で返す。
///
/// ### アルゴリズム
/// 最後に選択したのが a ならば次の選択肢は a+1, a+2, ...
///
/// ### 計算量
/// O(binom(n + 1, r))
fn combinations(n: usize, r: usize) -> Vec<Vec<usize>> {
    // seq: 現在の状態
    // seq_list: 結果の蓄積物

    fn rec(n: usize, r: usize, seq: &mut Vec<usize>, seq_list: &mut Vec<Vec<usize>>) {
        if seq.len() == r {
            // ここがforループの中のようなもの
            seq_list.push(seq.clone());
            return;
        }

        let begin = seq.last().copied().map(|x| x + 1).unwrap_or(0);
        let left_to_choose = r - seq.len();
        // i, i+1, ..., i + left_to_choose - 1 という選択はせめてできてほしい。
        // それすら出来ない場合はここで i を選ぶ意味がない
        for i in (begin..).take_while(|&i| i + left_to_choose - 1 < n) {
            seq.push(i);
            rec(n, r, seq, seq_list);
            seq.pop();
        }
    }

    // rec の呼び出し回数は binom(n + 1, r)
    //  → 深さ d の呼び出し回数が binom(n - (r - d), d) 回で、ホッケースティック恒等式を使うと得られる。)
    let mut seq_list = vec![];
    rec(n, r, &mut vec![], &mut seq_list);
    seq_list
}

/// 組合せ全列挙の別実装。(combinations より再帰呼び出し回数が多い)
///
/// n 個のものからr個取る組合せ nCr 個を全列挙する。
/// 各組合せは長さ `r` の `Vec<usize>`  (ソート済)  で表し、組合せの全列挙を `Vec<Vec<usize>>` で返す。
///
/// ### アルゴリズム
/// 各 i に対して、 i番目を選ぶかどうかで分岐する
fn combinations2(n: usize, r: usize) -> Vec<Vec<usize>> {
    // n個のものからr個取る組合せ nCr

    // i: 選ぶかどうかの対象(i番目を選ぶかどうか)
    // seq: 現在選んでいる要素の列
    // seq_list が結果の蓄積物
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

    #[test]
    fn test_combinations2() {
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
