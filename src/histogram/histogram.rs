use std::time::Duration;
use std::ops::Div;
use std::ops::Add;

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
        self.cumulative();
        self.mean();
        self.harmonic_mean();
    }

    fn cumulative(&mut self) {
        self.cumulative = Duration::new(0, 0);
        for d in self.durations.iter() {
            self.cumulative.add(*d);
        }

    }

    fn mean(&mut self) {
        self.mean = self.cumulative.div(self.durations.len() as u32)
    }

    fn harmonic_mean(&mut self) {

    }
}

fn fake_duration_data() -> Vec<Duration> {
    let mut data = Vec::new();
    data.push(Duration::new(1, 1));
    data.push(Duration::new(2, 2));
    data.push(Duration::new(3, 3));
    data
}