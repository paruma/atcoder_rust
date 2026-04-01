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
}
