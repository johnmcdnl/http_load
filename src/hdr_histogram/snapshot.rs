#[derive(Debug)]
pub struct Snapshot {
    lowest_trackable_value: i64,
    highest_trackable_value: i64,
    significant_figures: i64,
    counts: Vec<i64>,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        Snapshot {
            lowest_trackable_value: 0,
            highest_trackable_value: 0,
            significant_figures: 0,
            counts: Vec::new(),
        }
    }
}