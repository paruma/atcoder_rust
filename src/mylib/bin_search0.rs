use cargo_snippet::snippet;

// ng ng ng ok ok ok
//          ↑ここのindexを求めたい

// 念のために、pはi64全体で定義しておいて(配列外も適当にfalseとかtrueとか定義して)
// ok,ngは±2くらい大きめに範囲を取っておいたほうが安全。
// たぶん(ng, ok)の範囲、または(ok, ng)の範囲では定義されている必要がありそう。
// (ng, ok)でpがすべてfalseの場合はokが帰ってくるはず。
// 戻り値をretとしたとき、pの値は(ng, ret)でfalse, [ret, ok)でtrueなことが保証されているはず

#[snippet]
pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    //assert!(p(ok));
    //assert!(!p(ng));

    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        // mid != ok, mid != ngが保証されている
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

// xsは昇順ソートされている（されてなくても、 |i| key <= xs[i] が単調ならOK)
// key <= xs[i] となる最小のiを求める。
// そのようなiが存在しない場合はInfの代わりにNoneを返す？または、xs.len()を返すか。
// xs.len()を返す場合は戻り値をiとしたとき、[0, i)がxs[i] < keyを満たすといえる。
// 長さ0だったらどうする？
#[snippet(include = "bin_search")]
pub fn lower_bound<T>(xs: &[T], key: T) -> i64
where
    T: Ord,
{
    let pred = |i: i64| key <= xs[i as usize];
    bin_search(xs.len() as i64, -1 as i64, pred)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

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

    #[test]
    fn test_lower_bound() {
        // 1,3,5,7,...という数列に対してテストを行う。

        for len in 0..10 {
            // 1,3,...,2 * len -1
            let xs = (0..len).map(|i| 2 * i + 1).collect_vec();
            for key in -1..(2 * len) {
                // -1 => 0
                // 0 => 0
                // 1 => 0
                // 2 => 1
                // 3 => 1
                // ...
                // 2 * len - 1 => len - 1
                // 2 * len => len
                let excepted = if key <= -1 {
                    0
                } else if 2 * len <= key {
                    len
                } else {
                    key / 2
                };
                let ret = lower_bound(&xs, key);
                assert_eq!(ret, excepted);
            }
        }
    }
}
