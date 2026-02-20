use chrono::{DateTime, Utc};

pub struct Transaction {
    pub cost: f64,
    pub cost_abs: i64,
    pub date: DateTime<Utc>,
    pub description: String,
}