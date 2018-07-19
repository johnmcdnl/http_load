mod hdr_histogram;


fn main() {
    println!("Hello, world!");

    let h = hdr_histogram::hdr_histogram::HdrHistogram::new(300, 1000, 3);
    let s = hdr_histogram::snapshot::Snapshot::new();
    let b = hdr_histogram::bracket::Bracket::new();

    println!("{:?}", h);
    println!("{:?}", s);
    println!("{:?}", b);
}
