use std::time::Duration;
use std::ops::{Div, Mul, Add};

#[derive(Debug)]
pub struct Histogram {
    durations: Vec<Duration>,
    cumulative: Duration,
    mean: Duration,
    harmonic_mean: Duration,
}

impl Histogram {
    pub fn new() -> Histogram {
        let mut histogram = Histogram {
            durations: fake_duration_data(),
            cumulative: Duration::new(0, 0),
            mean: Duration::new(0, 0),
            harmonic_mean: Duration::new(0, 0),
        };
        histogram.calculate();
        histogram
    }

    pub fn add(&mut self, duration: Duration) {
        self.durations.push(duration);
        self.calculate();
    }

    fn calculate(&mut self) {
        if self.length() > 0 {
            self.cumulative();
            self.mean();
            self.harmonic_mean();
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
}

fn fake_duration_data() -> Vec<Duration> {
    let data = Vec::new();

    data
}

fn nanoseconds(d: Duration) -> u64 {
    d.as_secs().mul(1_000_000_000) + d.subsec_nanos() as u64
}