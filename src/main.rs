mod bench;
mod data;
mod engines;

use clap::Parser;

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
    println!("Benchmark engines: {:?}", args.engine);
    println!("Trades: {}", args.trades);
    println!("Repeat: {}", args.repeat);
    println!("Report format: {}", args.report);
    // TODO: регистрация движков, генерация данных, запуск бенчмарка
} 