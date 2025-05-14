use crate::engines::{AggregatorEngine, Order};
use std::time::Instant;

pub struct BenchmarkResult {
    pub engine: String,
    pub times: Vec<f64>,
}

pub fn run_benchmark(engines: &[Box<dyn AggregatorEngine>], orders: &[Order], repeat: usize) -> Vec<BenchmarkResult> {
    let mut results = vec![];
    for engine in engines {
        let mut times = vec![];
        for _ in 0..repeat {
            let start = Instant::now();
            let _trades = engine.aggregate(orders);
            let elapsed = start.elapsed().as_secs_f64();
            times.push(elapsed);
        }
        results.push(BenchmarkResult {
            engine: engine.name().to_string(),
            times,
        });
    }
    results
}

pub fn print_report(results: &[BenchmarkResult], format: &str) {
    match format {
        "csv" => {
            println!("engine,run,time_secs");
            for r in results {
                for (i, t) in r.times.iter().enumerate() {
                    println!("{},{},{}", r.engine, i + 1, t);
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
                println!("{}: avg = {:.6} sec, runs = {:?}", r.engine, avg, r.times);
            }
        }
    }
} 