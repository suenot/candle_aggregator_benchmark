use crate::engines::Trade;
use rand::Rng;

pub fn generate_trades(count: usize) -> Vec<Trade> {
    let mut rng = rand::thread_rng();
    (0..count).map(|i| {
        Trade {
            id: format!("t{}", i),
            price: 100.0 + rng.gen_range(-5.0..5.0),
            amount: rng.gen_range(0.01..1.0),
            side: if rng.gen_bool(0.5) { "buy".to_string() } else { "sell".to_string() },
            timestamp: 1_700_000_000_000 + (i as i64) * 1000,
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_trades() {
        let trades = super::generate_trades(5);
        assert_eq!(trades.len(), 5);
    }
} 