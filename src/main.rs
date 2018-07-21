use std::time::Duration;

mod hdr_histogram;

fn main() {
    let mut h = hdr_histogram::hdr_histogram::HDRHistogram::new();
    h.record_value(Duration::new(1, 100_000_000));
    h.record_value(Duration::new(2, 200_000_000));
    h.record_value(Duration::new(3, 300_000_000));
    h.get_start_time_stamp();
    h.get_end_time_stamp();

    h.get_mean();
    h.get_std_deviation();
    h.median_equivalent_value();

    h.get_max_value();
    h.get_min_value();
    h.get_min_non_zero_value();


    println!("{:?}", h)
}
