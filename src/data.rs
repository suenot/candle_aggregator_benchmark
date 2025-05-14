use crate::engines::Order;
use rand::Rng;

pub fn generate_orders(count: usize) -> Vec<Order> {
    let mut rng = rand::thread_rng();
    (0..count).map(|i| {
        Order {
            id: format!("o{}", i),
            price: 100.0 + rng.gen_range(-5.0..5.0),
            amount: rng.gen_range(0.01..1.0),
            side: if rng.gen_bool(0.5) { "buy".to_string() } else { "sell".to_string() },
            timestamp: 1_700_000_000_000 + (i as i64) * 1000,
        }
    }).collect()
} 