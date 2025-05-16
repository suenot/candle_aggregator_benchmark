use candle_generator::{CandleGenerator, Trade as CGTrade, Candle as CGCandle, Instrument, Pair, MarketType, Side, Timeframe};

pub trait AggregatorEngine {
    fn name(&self) -> &'static str;
    fn aggregate(&self, trades: &[Trade]) -> Vec<Candle>;
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub price: f64,
    pub amount: f64,
    pub side: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub trade_count: u64,
}

// Stub для candle_generator (реализация будет позже)
pub struct CandleGeneratorEngine;
impl AggregatorEngine for CandleGeneratorEngine {
    fn name(&self) -> &'static str { "candle_generator" }
    fn aggregate(&self, trades: &[Trade]) -> Vec<Candle> {
        let cg_trades: Vec<CGTrade> = trades.iter().map(|t| {
            CGTrade {
                instrument: Instrument {
                    pair: Pair { base_id: "BTC".to_string(), quote_id: "USDT".to_string() },
                    exchange: "bench".to_string(),
                    market_type: MarketType::Spot,
                },
                id: t.id.clone(),
                price: t.price,
                amount: t.amount,
                side: match t.side.as_str() {
                    "buy" => Side::Buy,
                    "sell" => Side::Sell,
                    _ => Side::Unknown,
                },
                timestamp: chrono::Utc.timestamp_millis_opt(t.timestamp).unwrap(),
            }
        }).collect();
        let generator = CandleGenerator::default();
        let cg_candles = generator.aggregate(cg_trades.iter(), Timeframe::m1);
        cg_candles.iter().map(|c| Candle {
            timestamp: c.timestamp.timestamp_millis(),
            open: c.open,
            high: c.high,
            low: c.low,
            close: c.close,
            volume: c.volume,
            trade_count: c.trade_count,
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aggregate_empty() {
        let engine = CandleGeneratorEngine;
        let trades = vec![];
        let candles = engine.aggregate(&trades);
        assert!(candles.is_empty());
    }
    #[test]
    fn test_aggregate_one_trade() {
        let engine = CandleGeneratorEngine;
        let trades = vec![Trade {
            id: "t1".to_string(),
            price: 100.0,
            amount: 1.0,
            side: "buy".to_string(),
            timestamp: 1714000000000,
        }];
        let candles = engine.aggregate(&trades);
        assert_eq!(candles.len(), 1);
        assert_eq!(candles[0].open, 100.0);
        assert_eq!(candles[0].volume, 1.0);
    }
} 