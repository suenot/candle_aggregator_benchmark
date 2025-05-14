mod bench;
mod data;
mod engines;

use clap::Parser;
use engines::CandleGeneratorEngine;
use engines::AggregatorEngine;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Список движков через запятую (например: candle_generator,python_lib,external_api)
    #[arg(short = 'e', long)]
    engine: String,

    /// Количество трейдов для теста
    #[arg(short = 'n', long, default_value = "100000")] 
    trades: usize,

    /// Количество повторов для каждого движка
    #[arg(short = 'r', long, default_value = "3")]
    repeat: usize,

    /// Формат отчёта (csv/json/pretty)
    #[arg(short = 'f', long, default_value = "pretty")]
    report: String,
}

fn main() {
    let args = Args::parse();
    let engines_list: Vec<&str> = args.engine.split(',').map(|s| s.trim()).collect();
    let mut engines: Vec<Box<dyn AggregatorEngine>> = vec![];
    for e in engines_list {
        match e {
            "candle_generator" => engines.push(Box::new(CandleGeneratorEngine)),
            // TODO: другие движки
            _ => println!("Неизвестный движок: {} (будет пропущен)", e),
        }
    }
    if engines.is_empty() {
        println!("Нет валидных движков для бенчмарка");
        return;
    }
    let orders = data::generate_orders(args.trades);
    let results = bench::run_benchmark(&engines, &orders, args.repeat);
    bench::print_report(&results, &args.report);
} 