

#[derive(Debug, Copy, Clone)]
pub struct TimeSpan {
    pub start: f64,
    pub samples: usize,
    pub sample_rate: f64,
}

impl TimeSpan {
    pub fn total_time(&self) -> f64 {
        self.samples as f64 * self.time_per_sample()
    }

    pub fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    pub fn time_iter(&self) -> TimeIterator {
        TimeIterator {
            start: self.start,
            samples: self.samples,
            per_sample: self.time_per_sample(),
            at: 0
        }
    }
}

pub struct TimeIterator {
    start: f64,
    samples: usize,
    per_sample: f64,
    at: usize,
}

impl Iterator for TimeIterator {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.at < self.samples {
            let new_at = self.at + 1;
            let at_time = self.start + self.per_sample * (self.at as f64);
            self.at = new_at;
            Some(at_time)
        } else {
            None
        }
    }
}