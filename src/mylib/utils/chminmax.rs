use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[macro_use]
#[snippet]
pub mod chminmax {
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! chmin {
        ($a: expr, $b: expr) => {
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        };
    }

    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! chmax {
        ($a: expr, $b: expr) => {
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        };
    }
}

// test
#[cfg(test)]
mod tests {
    #[test]
    fn test_chmin() {
        {
            let mut a = 3;
            let b = 2;
            assert!(chmin!(a, b));
            assert_eq!(a, 2);
        }
        {
            let mut a = 3;
            let b = 3;
            assert!(!chmin!(a, b));
            assert_eq!(a, 3);
        }
        {
            let mut a = 3;
            let b = 4;
            assert!(!chmin!(a, b));
            assert_eq!(a, 3);
        }
    }

    #[test]
    fn test_chmax() {
        {
            let mut a = 3;
            let b = 2;
            assert!(!chmax!(a, b));
            assert_eq!(a, 3);
        }
        {
            let mut a = 3;
            let b = 3;
            assert!(!chmax!(a, b));
            assert_eq!(a, 3);
        }
        {
            let mut a = 3;
            let b = 4;
            assert!(chmax!(a, b));
            assert_eq!(a, 4);
        }
    }
}
