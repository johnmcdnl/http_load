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

        let unit_magnitude = if { min_value as f64 }.log2().floor() as i64 != 0 {
            { min_value as f64 }.log2().floor() as i64
        } else {
            0
        };

        let largest_value_with_single_unit_resolution = { sig_figs as f64 }.powi(10);
        let sub_bucket_count_magnitude = { largest_value_with_single_unit_resolution.log2().ceil() };

        let sub_bucket_half_count_magnitude = if { sub_bucket_count_magnitude as i64 } < 1 {
            1
        } else {
            sub_bucket_count_magnitude as i64 - 1
        };


        let sub_bucket_count = { { { sub_bucket_half_count_magnitude + 1 } }.pow(2) };
        let sub_bucket_half_count = sub_bucket_count / 2;
        let sub_bucket_mask = sub_bucket_count - 1 << unit_magnitude;

        let mut smallest_untrackable_value = sub_bucket_count << unit_magnitude;
        let mut bucket_count = 1;
        while smallest_untrackable_value < max_value {
            smallest_untrackable_value <<= 1;
            bucket_count += 1
        }
        let counts_len = { bucket_count + 1 } * { sub_bucket_count / 2 };


        HdrHistogram
            {
                lowest_trackable_value: min_value,
                highest_trackable_value: max_value,
                unit_magnitude,
                significant_figures: sig_figs,
                sub_bucket_half_count_magnitude,
                sub_bucket_count,
                sub_bucket_half_count,
                sub_bucket_mask,
                bucket_count,
                counts_len,
                total_count: 0,
                counts: Vec::new(),
            }
    }
}