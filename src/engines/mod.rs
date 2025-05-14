use candle_generator::{CandleGenerator, Trade as CGTrade, Instrument, Pair, MarketType, Side, Timeframe};

pub trait AggregatorEngine {
    fn name(&self) -> &'static str;
    fn aggregate(&self, orders: &[Order]) -> Vec<Trade>;
}

// Пример структуры Order и Trade (можно расширить)
#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub price: f64,
    pub amount: f64,
    pub side: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub price: f64,
    pub amount: f64,
    pub timestamp: i64,
}

// Stub для candle_generator (реализация будет позже)
pub struct CandleGeneratorEngine;
impl AggregatorEngine for CandleGeneratorEngine {
    fn name(&self) -> &'static str { "candle_generator" }
    fn aggregate(&self, orders: &[Order]) -> Vec<Trade> {
        // Преобразуем Order в candle_generator::Trade
        let trades: Vec<CGTrade> = orders.iter().map(|o| {
            CGTrade {
                instrument: Instrument {
                    pair: Pair { base_id: "BTC".to_string(), quote_id: "USDT".to_string() },
                    exchange: "bench".to_string(),
                    market_type: MarketType::Spot,
                },
                id: o.id.clone(),
                price: o.price,
                amount: o.amount,
                side: match o.side.as_str() {
                    "buy" => Side::Buy,
                    "sell" => Side::Sell,
                    _ => Side::Unknown,
                },
                timestamp: chrono::Utc.timestamp_millis_opt(o.timestamp).unwrap(),
            }
        }).collect();
        let generator = CandleGenerator::default();
        let candles = generator.aggregate(trades.iter(), Timeframe::m1);
        // Преобразуем свечи обратно в Vec<Trade> (например, open как trade)
        candles.iter().map(|c| Trade {
            id: format!("candle_{}", c.timestamp.timestamp_millis()),
            price: c.open,
            amount: c.volume,
            timestamp: c.timestamp.timestamp_millis(),
        }).collect()
    }
} 