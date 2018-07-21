use std::time::{Duration, Instant};
use std::ops::Div;

extern crate time;

#[derive(Debug)]
pub struct HDRHistogram {
    total_count: isize,
    counts: Vec<u64>,

    start_time: Instant,
    end_time: Instant,

    min_value: Duration,
    max_value: Duration,
    sum: Duration,
    mean: Duration,
    standard_deviation: Duration,
}

impl HDRHistogram {
    pub fn new() -> HDRHistogram {
        let mut h: HDRHistogram = HDRHistogram {
            total_count: 0,
            counts: Vec::new(),

            start_time: Instant::now(),
            end_time: Instant::now(),

            min_value: Duration::from_nanos(0),
            max_value: Duration::from_nanos(0),
            sum: Duration::from_nanos(0),
            mean: Duration::from_nanos(0),
            standard_deviation: Duration::from_nanos(0),
        };
        h.set_start_time_stamp();
        h.set_end_time_stamp();
        h
    }

    pub fn record_value(&mut self, d: Duration) {
        self.record_value_nanoseconds(duration_as_nanoseconds(d));
    }

    pub fn record_value_nanoseconds(&mut self, nanos: u64) {
        self.counts.push(nanos);
        self.total_count += 1;
    }


    pub fn get_max_value(&mut self) -> Duration {
        let max = *match self.counts.iter().max() {
            Some(i) => i,
            None => &{ 0 as u64 },
        };
        self.max_value = Duration::from_nanos(max);
        self.max_value
    }

    pub fn get_min_value(&mut self) -> Duration {
        let min = *match self.counts.iter().min() {
            Some(i) => i,
            None => &{ 0 as u64 },
        };
        self.min_value = Duration::from_nanos(min);
        self.min_value
    }

    pub fn get_min_non_zero_value(&self) -> Duration {
        Duration::new(0, 0)
    }

    pub fn get_start_time_stamp(&self) -> Instant {
        return self.start_time;
    }

    fn set_start_time_stamp(&mut self) {
        self.start_time = Instant::now()
    }

    pub fn get_end_time_stamp(&self) -> Instant {
        return self.end_time;
    }

    fn set_end_time_stamp(&mut self) {
        self.end_time = Instant::now()
    }

    pub fn get_sum(&mut self) -> Duration {
        self.sum = Duration::from_nanos(self.counts.iter().sum());
        self.sum
    }

    pub fn get_mean(&mut self) -> Duration {
        let sum = duration_as_nanoseconds(self.get_sum());
        self.mean = Duration::from_nanos(sum / self.counts.len() as u64);
        self.mean
    }

    pub fn get_std_deviation(&mut self) -> Duration {
        if self.counts.len() < 2 {
            return Duration::from_nanos(0);
        }

        let mean = duration_as_nanoseconds(self.get_mean());

        let mut sum_diffs = 0 as u128;

        for value in self.counts.iter() {
            let diff = if mean > *value { mean - value } else { value - mean } as u128;
            sum_diffs += diff * diff;
        }

        sum_diffs = sum_diffs.div(self.counts.len() as u128);

        self.standard_deviation = Duration::from_nanos({ sum_diffs as f64 }.sqrt() as u64);
        self.standard_deviation
    }

    pub fn median_equivalent_value(&self) {}
    // TODO

    //    fn add(&self, histogram: HDRHistogram) {}
    //    fn subtract(&self) {}
    //    fn addWhileCorrectingForCoordinatedOmission() {}
    //    fn allValues() {}
    //    fn copyInto() {}
    //    fn copyIntoCorrectedForCoordinatedOmission() {}
    //    fn encodeIntoByteBuffer() {}
    //    fn encodeIntoCompressedByteBuffer() {}
    //    fn equals() {}
    //    fn getCountAtValue() {}
    //    fn getCountBetweenValues() {}
    //    fn getEstimatedFootprintInBytes() {}
    //    fn getHighestTrackableValue() {}
    //    fn getLowestDiscernibleValue() {}
    //    fn getMaxValueAsDouble() {}
    //    fn getNeededByteBufferCapacity() {}
    //    fn getNumberOfSignificantValueDigits() {}
    //    fn getPercentileAtOrBelowValue() {}
    //    fn getTag() {}
    //    fn getValueAtPercentile() {}
    //    fn hashCode() {}
    //    fn highestEquivalentValue() {}
    //    fn isAutoResize() {}
    //    fn linearBucketValues() {}
    //    fn logarithmicBucketValues() {}
    //    fn lowestEquivalentValue() {}
    //    fn nextNonEquivalentValue() {}
    //    fn outputPercentileDistribution() {}
    //    fn percentiles() {}
    //    fn recordConvertedDoubleValueWithCount() {}
    //    fn recordedValues() {}
    //    fn record_value() {}
    //    fn recordValueWithCount() {}
    //    fn recordValueWithExpectedInterval() {}
    //    fn reset() {}
    //    fn setAutoResize() {}
    //    fn setTag() {}
    //    fn shiftValuesLeft() {}
    //    fn shiftValuesRight() {}
    //    fn sizeOfEquivalentValueRange() {}
    //    fn supportsAutoResize() {}
    //    fn valuesAreEquivalent() {}
}

fn duration_as_nanoseconds(d: Duration) -> u64 {
    match time::Duration::from_std(d) {
        Ok(d) => match d.num_nanoseconds() {
            Some(i) => i as u64,
            None => panic!("None "),
        },
        Err(err) => panic!("invalid {:?}", err)
    }
}