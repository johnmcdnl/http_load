use std::ops::{Add, Div, Index, Mul, Sub};
use std::time::Duration;

#[derive(Debug)]
pub struct Histogram {
    durations: Vec<Duration>,
    durations_as_nano: Vec<i64>,
    cumulative: Duration,
    mean: Duration,
    harmonic_mean: Duration,
    standard_deviation: Duration,
}

impl Histogram {
    pub fn new() -> Histogram {
        let mut histogram = Histogram {
            durations: fake_duration_data(),
            durations_as_nano: Vec::new(),
            cumulative: Duration::new(0, 0),
            mean: Duration::new(0, 0),
            harmonic_mean: Duration::new(0, 0),
            standard_deviation: Duration::new(0, 0),
        };
        histogram.calculate();
        histogram
    }

    pub fn add(&mut self, duration: Duration) {
        self.durations.push(duration);
        self.durations_as_nano.push(nanoseconds(duration));
        self.length();
        self.calculate();
    }

    fn calculate(&mut self) {
        if self.length() > 0 {
            self.cumulative();
            self.mean();
            self.harmonic_mean();
            self.standard_deviation();
        }
    }

    fn cumulative(&mut self) {
        self.cumulative = self.durations.iter().sum();
    }

    fn length(&mut self) -> i64 {
        self.durations.len() as i64
    }

    fn mean(&mut self) {
        self.mean = self.cumulative.div(self.length() as u32)
    }

    fn harmonic_mean(&mut self) {
        let mut sum_reciprocals = 0 as f64;
        for duration in self.durations.iter() {
            sum_reciprocals = sum_reciprocals.add({ 1 as f64 }.div(nanoseconds(*duration) as f64));
        }
        let nanos = { self.length() as f64 }.div(sum_reciprocals) as u64;
        self.harmonic_mean = Duration::from_nanos(nanos);
    }

    fn standard_deviation(&mut self) {
        if self.length() < 2 {
            return;
        }
        let mut sum_diff: i128 = 0;
        let mean = nanoseconds(self.mean);
        for duration in self.durations_as_nano.iter() {
            let diff = duration.sub(mean);
            let product_diff = { diff as i128 }.mul(diff as i128);
            sum_diff = sum_diff.add(product_diff);
        }
        println!("{:?}", sum_diff);
        let std_dev = { sum_diff.div((self.length() - 1) as i128) as f64 }.sqrt();
        self.standard_deviation = Duration::from_nanos(std_dev as u64);
    }

    pub fn percentile(&mut self, p: f64) -> Duration {
        let index = { { self.length() as f64 }.mul(p.add(0.5)) } as usize - 1;
        *self.durations.index(index)
    }
}

fn fake_duration_data() -> Vec<Duration> {
    let data = Vec::new();

    data
}

fn nanoseconds(d: Duration) -> i64 {
    let secs = d.as_secs().mul(1_000_000_000);
    let nanos = d.subsec_nanos();
    let total = secs + nanos as u64;
    total as i64
}
