use cargo_snippet::snippet;

// ng ng ng ok ok ok
//          ↑ここのindexを求めたい

// 念のために、pはi64全体で定義しておいて(配列外も適当にfalseとかtrueとか定義して)
// ok,ngは±2くらい大きめに範囲を取っておいたほうが安全。

#[snippet]
pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    //assert!(p(ok));
    //assert!(!p(ng));

    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::int_plus_one)]
    #[test]
    fn test_bin_search() {
        for ok in -20..20 {
            for ng in (ok - 20)..ok {
                assert!(ng + 1 <= ok); //テストの実装のためのassert
                for bd in (ng + 1)..=ok {
                    // false false false true true true
                    let p = |x| bd <= x;
                    let ret = bin_search(ok, ng, p);
                    assert_eq!(ret, bd);
                }
            }
        }
    }

    #[allow(clippy::int_plus_one)]
    #[test]
    fn test_bin_search2() {
        for ok in -20..20 {
            for ng in (ok + 1)..=(ok + 20) {
                assert!(ok <= ng - 1); //テストの実装のためのassert
                for bd in ok..ng {
                    // true true true false false false
                    let p = |x| x <= bd;
                    let ret = bin_search(ok, ng, p);
                    assert_eq!(ret, bd);
                }
            }
        }
    }
}

// (ok, ng, bd) = (-20, -18, -19)
// -19
// mid: -19
// ok=-19
// mid=-19
// -20, -19
