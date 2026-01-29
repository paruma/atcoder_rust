use std::time::{Duration, Instant};

/// ベンチマークの結果を保持する構造体
pub struct BenchResult {
    pub name: String,
    pub total_time: Duration,
    pub query_count: usize,
}

impl BenchResult {
    pub fn print_markdown_row(&self) {
        println!(
            "| {:<20} | {:>12?} | {:>12.3} ns |",
            self.name,
            self.total_time,
            self.total_time.as_nanos() as f64 / self.query_count as f64
        );
    }
}

/// 複数のベンチマーク結果をまとめて表示・比較する
pub fn print_results(results: &[BenchResult]) {
    println!(
        "\n| {:<20} | {:>12} | {:>15} |",
        "Implementation", "Total Time", "Time per Query"
    );
    println!("| :--- | :---: | :---: |");
    for res in results {
        res.print_markdown_row();
    }

    if results.len() >= 2 {
        let base = &results[0];
        for target in results[1..].iter() {
            let ratio = target.total_time.as_nanos() as f64 / base.total_time.as_nanos() as f64;
            println!(
                "\n{} is {:.2}x slower than {}.",
                target.name, ratio, base.name
            );
        }
    }
}

/// 特定の実装に対してクエリ列を実行し、時間を計測する
///
/// `ds`: データ構造のインスタンス
/// `queries`: 実行するクエリのリスト
/// `f`: クエリをデータ構造のメソッド呼び出しにマップするクロージャ。
///      最適化によるコード消去を防ぐため、戻り値として i64 (チェックサム用) を返すことを推奨。
pub fn run_bench<T, Q, F>(name: &str, mut ds: T, queries: &[Q], mut f: F) -> BenchResult
where
    F: FnMut(&mut T, &Q) -> i64,
{
    let start = Instant::now();
    let mut checksum = 0;
    for q in queries {
        checksum ^= f(&mut ds, q);
    }
    let total_time = start.elapsed();
    println!("{} Checksum: {}", name, checksum);

    BenchResult {
        name: name.to_string(),
        total_time,
        query_count: queries.len(),
    }
}
