use cargo_snippet::snippet;

#[snippet(prefix = "use acl_lcp_array::*;")]
#[allow(clippy::module_inception)]
pub mod acl_lcp_array {
    use ac_library::{Min, Segtree, lcp_array_arbitrary, suffix_array_arbitrary};

    /// 文字列内の部分文字列の LCP および辞書順比較を効率的に処理する。
    pub struct SubstringLcpQuery {
        rank: Vec<usize>,
        lcp_rmq: Segtree<Min<usize>>,
        n: usize,
    }

    impl SubstringLcpQuery {
        /// 部分文字列の LCP 計算や辞書順比較を行うためのデータ構造を構築する。
        /// 計算量: O(N log N)
        pub fn new<T: Ord + Clone>(s: &[T]) -> Self {
            let n = s.len();
            assert!(n >= 1, "文字列の長さは 1 以上である必要があります。");

            let sa = suffix_array_arbitrary(s);
            let lcp = lcp_array_arbitrary(s, &sa);
            let mut rank = vec![0; n];
            for (i, &x) in sa.iter().enumerate() {
                rank[x] = i;
            }
            let lcp_rmq = Segtree::<Min<usize>>::from(lcp);

            Self { rank, lcp_rmq, n }
        }

        /// s[i..] と s[j..] の LCP を返す
        pub fn lcp_suffix(&self, i: usize, j: usize) -> usize {
            assert!(i <= self.n && j <= self.n);
            if i == self.n || j == self.n {
                return 0;
            }
            if i == j {
                return self.n - i;
            }
            let r1 = self.rank[i];
            let r2 = self.rank[j];
            let (min_r, max_r) = if r1 < r2 { (r1, r2) } else { (r2, r1) };
            self.lcp_rmq.prod(min_r..max_r)
        }

        /// s[begin1..end1] と s[begin2..end2] の LCP を返す
        pub fn lcp_substring(
            &self,
            begin1: usize,
            end1: usize,
            begin2: usize,
            end2: usize,
        ) -> usize {
            let l = self.lcp_suffix(begin1, begin2);
            l.min(end1 - begin1).min(end2 - begin2)
        }

        /// s[begin1..end1] と s[begin2..end2] の辞書順比較を行う
        pub fn compare_substring(
            &self,
            begin1: usize,
            end1: usize,
            begin2: usize,
            end2: usize,
        ) -> std::cmp::Ordering {
            let len1 = end1 - begin1;
            let len2 = end2 - begin2;
            let l = self.lcp_substring(begin1, end1, begin2, end2);
            if l == len1 && l == len2 {
                return std::cmp::Ordering::Equal;
            }
            if l == len1 {
                return std::cmp::Ordering::Less;
            }
            if l == len2 {
                return std::cmp::Ordering::Greater;
            }
            self.rank[begin1 + l].cmp(&self.rank[begin2 + l])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::acl_lcp_array::SubstringLcpQuery;
    use itertools::Itertools;

    #[test]
    fn test_basic() {
        let s = "mississippi".chars().collect_vec();
        // 01234567890
        // m i s s i s s i p p i
        let lcp = SubstringLcpQuery::new(&s);

        // 接尾辞 LCP: "issippi" (1..) vs "ississippi" (4..) -> "issi" (4)
        assert_eq!(lcp.lcp_suffix(1, 4), 4);

        // 比較: "mississippi" (1..11) vs "issippi" (4..11)
        // LCP "issi" の次の文字 s[5]='s', s[8]='p' で 's' > 'p'
        assert_eq!(
            lcp.compare_substring(1, 11, 4, 11),
            std::cmp::Ordering::Greater
        );

        // 部分文字列 LCP: "miss" (0..4) vs "iss" (1..4) -> 0
        assert_eq!(lcp.lcp_substring(0, 4, 1, 4), 0);
        assert_eq!(
            lcp.compare_substring(0, 4, 1, 4),
            std::cmp::Ordering::Greater
        );

        // 部分文字列 LCP: "sipp" (6..10) vs "siss" (3..7)
        // s[6..10] = "sipp", s[3..7] = "siss" -> LCP "si" (2)
        // 次の文字 'p' < 's'
        assert_eq!(lcp.lcp_substring(6, 10, 3, 7), 2);
        assert_eq!(lcp.compare_substring(6, 10, 3, 7), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_edge_cases() {
        let s = "banana".chars().collect_vec();
        // 012345
        // b a n a n a
        let lcp = SubstringLcpQuery::new(&s);

        // 同じ接尾辞
        assert_eq!(lcp.lcp_suffix(1, 1), 5);

        // 全く同じ部分文字列
        assert_eq!(lcp.lcp_substring(1, 4, 3, 6), 3); // "ana" vs "ana"
        assert_eq!(lcp.compare_substring(1, 4, 3, 6), std::cmp::Ordering::Equal);

        // 接頭辞の関係
        assert_eq!(lcp.compare_substring(0, 3, 0, 6), std::cmp::Ordering::Less); // "ban" vs "banana"
        assert_eq!(
            lcp.compare_substring(0, 6, 0, 3),
            std::cmp::Ordering::Greater
        );

        // 空文字列との比較
        assert_eq!(lcp.lcp_substring(0, 0, 0, 3), 0);
        assert_eq!(lcp.compare_substring(0, 0, 0, 3), std::cmp::Ordering::Less);

        // 文字列末尾
        assert_eq!(lcp.lcp_substring(5, 6, 6, 6), 0); // "a" vs ""
        assert_eq!(
            lcp.compare_substring(5, 6, 6, 6),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    #[should_panic]
    fn test_empty_string_panic() {
        let s: Vec<char> = vec![];
        let _ = SubstringLcpQuery::new(&s);
    }

    #[test]
    #[ignore]
    fn test_random() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..1000 {
            let n = rng.random_range(1..=50);
            let s: Vec<u8> = (0..n).map(|_| rng.random_range(b'a'..b'd')).collect();
            let lcp_finder = SubstringLcpQuery::new(&s);

            for _ in 0..20 {
                let begin1 = rng.random_range(0..=n);
                let end1 = rng.random_range(begin1..=n);
                let begin2 = rng.random_range(0..=n);
                let end2 = rng.random_range(begin2..=n);

                let sub1 = &s[begin1..end1];
                let sub2 = &s[begin2..end2];
                let expected_lcp = sub1
                    .iter()
                    .zip(sub2.iter())
                    .take_while(|(a, b)| a == b)
                    .count();

                let expected_cmp = sub1.cmp(sub2);

                let actual_lcp = lcp_finder.lcp_substring(begin1, end1, begin2, end2);
                let actual_cmp = lcp_finder.compare_substring(begin1, end1, begin2, end2);

                assert_eq!(
                    actual_lcp,
                    expected_lcp,
                    "LCP mismatch: s={:?}, sub1={:?}, sub2={:?}",
                    String::from_utf8_lossy(&s),
                    String::from_utf8_lossy(sub1),
                    String::from_utf8_lossy(sub2)
                );
                assert_eq!(
                    actual_cmp,
                    expected_cmp,
                    "Compare mismatch: s={:?}, sub1={:?}, sub2={:?}",
                    String::from_utf8_lossy(&s),
                    String::from_utf8_lossy(sub1),
                    String::from_utf8_lossy(sub2)
                );
            }
        }
    }
}
