use crate::engines::{AggregatorEngine, Trade, Candle};
use std::time::Instant;

#[derive(serde::Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    struct MockEngine;
    impl crate::engines::AggregatorEngine for MockEngine {
        fn name(&self) -> &'static str { "mock" }
        fn aggregate(&self, trades: &[crate::engines::Trade]) -> Vec<crate::engines::Candle> {
            trades.iter().map(|t| crate::engines::Candle {
                timestamp: t.timestamp,
                open: t.price,
                high: t.price,
                low: t.price,
                close: t.price,
                volume: t.amount,
                trade_count: 1,
            }).collect()
        }
    }
    #[test]
    fn test_run_benchmark() {
        let engine = MockEngine;
        let trades = vec![crate::engines::Trade {
            id: "t1".to_string(), price: 1.0, amount: 2.0, side: "buy".to_string(), timestamp: 1
        }];
        let engines: Vec<Box<dyn crate::engines::AggregatorEngine>> = vec![Box::new(engine)];
        let results = super::run_benchmark(&engines, &trades, 2);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].candles, 1);
        assert_eq!(results[0].times.len(), 2);
    }
    #[test]
    fn test_print_report() {
        let results = vec![
            super::BenchmarkResult { engine: "mock".to_string(), times: vec![0.1, 0.2], candles: 2 }
        ];
        super::print_report(&results, "csv");
        super::print_report(&results, "json");
        super::print_report(&results, "pretty");
    }
} 