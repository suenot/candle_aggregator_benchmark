use crate::engines::{AggregatorEngine, Trade, Candle};
use std::time::Instant;

pub struct BenchmarkResult {
    pub engine: String,
    pub times: Vec<f64>,
    pub candles: usize,
}

pub fn run_benchmark(engines: &[Box<dyn AggregatorEngine>], trades: &[Trade], repeat: usize) -> Vec<BenchmarkResult> {
    let mut results = vec![];
    for engine in engines {
        let mut times = vec![];
        let mut last_candles = 0;
        for _ in 0..repeat {
            let start = Instant::now();
            let candles = engine.aggregate(trades);
            let elapsed = start.elapsed().as_secs_f64();
            last_candles = candles.len();
            times.push(elapsed);
        }
        results.push(BenchmarkResult {
            engine: engine.name().to_string(),
            times,
            candles: last_candles,
        });
    }
    results
}

pub fn print_report(results: &[BenchmarkResult], format: &str) {
    match format {
        "csv" => {
            println!("engine,run,time_secs,candles");
            for r in results {
                for (i, t) in r.times.iter().enumerate() {
                    println!("{},{},{},{}", r.engine, i + 1, t, r.candles);
                }
            }
        }
        "json" => {
            println!("{}", serde_json::to_string_pretty(&results).unwrap());
        }
        _ => {
            println!("=== Benchmark Results ===");
            for r in results {
                let avg: f64 = r.times.iter().sum::<f64>() / r.times.len() as f64;
                println!("{}: avg = {:.6} sec, runs = {:?}, candles = {}", r.engine, avg, r.times, r.candles);
            }
        }
    }
} 