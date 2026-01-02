use cargo_snippet::snippet;

#[snippet(prefix = "use time_keeper::*;")]
pub mod time_keeper {
    // 参考: Rust 競プロ AHC参加の準備してみた（チートシート集）
    // https://zenn.dev/tipstar0125/articles/245bceec86e40a

    #[derive(Debug, Clone)]
    pub struct TimeKeeper {
        start_time: std::time::Instant,
        time_threshold_sec: f64,
    }

    impl TimeKeeper {
        /// time_threshold_sec: 制限時間(秒数)
        pub fn new(time_threshold_sec: f64) -> Self {
            TimeKeeper {
                start_time: std::time::Instant::now(),
                time_threshold_sec,
            }
        }
        #[inline]
        pub fn is_time_over(&self) -> bool {
            let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
            elapsed_time >= self.time_threshold_sec
        }
    }
}

#[cfg(test)]
mod tests {
    use super::time_keeper::*;
    #[test]
    fn test_time_keeper() {
        let time_keeper = TimeKeeper::new(0.001); // 1 ミリ秒
        assert!(!time_keeper.is_time_over());
        std::thread::sleep(std::time::Duration::from_millis(2));
        assert!(time_keeper.is_time_over());
    }
}
