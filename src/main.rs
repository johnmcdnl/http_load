use std::time::Duration;

mod histogram;


fn main() {
    println!("Hello, world!");


    let mut h = histogram::histogram::Histogram::new();
    h.add(Duration::new(10, 0));
    h.add(Duration::new(20, 0));
    h.add(Duration::new(30, 0));
    println!("{:?}", h);
}
