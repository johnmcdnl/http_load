#[derive(Debug)]
pub struct HdrHistogram {
    lowest_trackable_value: i64,
    highest_trackable_value: i64,
    unit_magnitude: i64,
    significant_figures: i64,
    sub_bucket_half_count_magnitude: i64,
    sub_bucket_half_count: i64,
    sub_bucket_mask: i64,
    sub_bucket_count: i64,
    bucket_count: i64,
    counts_len: i64,
    total_count: i64,
    counts: Vec<i64>,
}

impl HdrHistogram {
    pub fn new(min_value: i64, max_value: i64, sig_figs: i64) -> HdrHistogram {
        if sig_figs < 1 || 5 < sig_figs {
            panic!("sig_figs should be between 1-5 but was given {:?}", sig_figs)
        }

        let unit_magnitude = ((min_value as f64).log2()).floor();
        if unit_magnitude <= {0 as f64} {
            let unit_magnitude = 0;
        }


        HdrHistogram {
            lowest_trackable_value: min_value,
            highest_trackable_value: max_value,
            unit_magnitude: unit_magnitude as i64,
            significant_figures: sig_figs,
            sub_bucket_half_count_magnitude: 0,
            sub_bucket_half_count: 0,
            sub_bucket_mask: 0,
            sub_bucket_count: 0,
            bucket_count: 0,
            counts_len: 0,
            total_count: 0,
            counts: Vec::new(),
        }
    }
}