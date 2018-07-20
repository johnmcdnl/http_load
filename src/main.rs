use std::time::Duration;

mod histogram;


fn main() {
    let mut h = histogram::histogram::Histogram::new();
    h.add(Duration::new(10, 0));
    h.add(Duration::new(20, 0));
    h.add(Duration::new(30, 0));

    println!("{:?}", h);
    println!("{:?}", h.percentile(0.50));
}
