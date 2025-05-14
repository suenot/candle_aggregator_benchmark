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
    fn aggregate(&self, _orders: &[Order]) -> Vec<Trade> {
        // TODO: вызвать candle_generator
        vec![]
    }
} 