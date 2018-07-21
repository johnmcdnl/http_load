use std::time::Duration;

mod hdr_histogram;

fn main() {
    let mut h = hdr_histogram::hdr_histogram::HDRHistogram::new();
    println!("{:?}", h);
    h.record_value(Duration::new(1_000, 0));
    h.record_value(Duration::new(2_000, 0));
    h.record_value(Duration::new(3_000, 0));
    h.record_value(Duration::new(4_000, 0));
    h.record_value(Duration::new(5_000, 0));
    h.calc();
    println!("{:?}", h);
}
