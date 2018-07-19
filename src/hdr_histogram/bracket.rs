#[derive(Debug)]
pub struct Bracket {
    quantile: f64,
    count: i64,
    value_at: i64,
}

impl Bracket {
    pub fn new() -> Bracket {
        Bracket {
            quantile: 0.0,
            count: 0,
            value_at: 0,
        }
    }
}