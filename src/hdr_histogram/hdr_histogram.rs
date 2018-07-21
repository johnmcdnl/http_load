use std::time::{Duration, Instant};

extern crate time;

#[derive(Debug)]
pub struct HDRHistogram {
    total_count: isize,
    counts: Vec<u64>,
    min_value: Duration,
    max_value: Duration,
    start_time: Instant,
    end_time: Instant,
}

impl HDRHistogram {
    pub fn new() -> HDRHistogram {
        let mut h: HDRHistogram = HDRHistogram {
            total_count: 0,
            counts: Vec::new(),
            min_value: Duration::new(0, 0),
            max_value: Duration::new(0, 0),
            start_time: Instant::now(),
            end_time: Instant::now(),
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
        self.max_value = Duration::new(0, max as u32);
        self.max_value
    }

    pub fn get_min_value(&mut self) -> Duration {
        let min = *match self.counts.iter().min() {
            Some(i) => i,
            None => &{ 0 as u64 },
        };
        self.min_value = Duration::new(0, min as u32);
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

    pub fn get_mean(&self) {}
    pub fn median_equivalent_value(&self) {}

    pub fn get_std_deviation(&self) -> Duration {
        Duration::new(0, 0)
    }

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