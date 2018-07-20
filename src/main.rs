use std::time::Duration;

mod histogram;


fn main() {
    println!("Hello, world!");


    let mut h = histogram::histogram::Histogram::new();
    h.add(Duration::new(3, 3));
    println!("{:?}", h);
}
