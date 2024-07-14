#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use ac_library::{lcp_array_arbitrary, suffix_array_arbitrary, z_algorithm_arbitrary};
    use itertools::Itertools;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_suffix_string() {
        let s = "missisippi".chars().collect_vec();
        let n = s.len();
        let sa = suffix_array_arbitrary(&s);
        for i in 0..n {
            let suffix = &s[sa[i]..];
            let suffix_string = suffix.iter().collect::<String>();
            println!("{} {}", i, suffix_string);
        }
    }

    #[test]
    fn test_lcp_array() {
        let s = "missisippi".chars().collect_vec();
        let n = s.len();
        let sa = suffix_array_arbitrary(&s);
        let lcp_array = lcp_array_arbitrary(&s, &sa);
        for i in 0..n {
            let suffix = &s[sa[i]..];
            let suffix_string = suffix.iter().collect::<String>();
            let lcp_msg = lcp_array
                .get(i)
                .map(|l| l.to_string())
                .unwrap_or("".to_string());
            println!("{:>2} {}", lcp_msg, suffix_string);
        }
    }

    #[test]
    fn test_z_algorithm() {
        let s = "missisippi".chars().collect_vec();
        let n = s.len();
        let lcp_len = z_algorithm_arbitrary(&s);
        for i in 0..n {
            let suffix = &s[i..];
            let suffix_string = suffix.iter().collect::<String>();
            println!("{:>2} {}", lcp_len[i], suffix_string);
        }
    }
}
