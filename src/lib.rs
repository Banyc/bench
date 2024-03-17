use std::{num::NonZeroUsize, time::Instant};

use math::statistics::{MeanExt, StandardDeviationExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct BencherConfig {
    /// Iterations of operations to warm up the CPU cache
    pub warm_ups: usize,
    /// Minimum iterations of operations that are measurable by [`std::time::Instant::elapsed()`]
    pub batch_size: NonZeroUsize,
    /// The number of sample durations to collect for the statistics
    pub samples: NonZeroUsize,
}

#[derive(Debug, Clone)]
pub struct Bencher {
    config: BencherConfig,
}
impl Bencher {
    pub fn new(config: BencherConfig) -> Self {
        Self { config }
    }

    pub fn iter<T>(&self, mut f: impl FnMut() -> T) -> BenchStats {
        for _ in 0..self.config.warm_ups {
            std::hint::black_box(f());
        }

        let mut durations = Vec::with_capacity(self.config.samples.get());
        for _ in 0..self.config.samples.get() {
            let start = Instant::now();
            for _ in 0..self.config.batch_size.get() {
                std::hint::black_box(f());
            }
            let duration = start.elapsed();
            durations.push(duration);
        }

        let sec = durations
            .iter()
            .map(|d| d.as_secs_f64())
            .map(|d| d / self.config.batch_size.get() as f64);
        let mean_sec = sec.clone().mean().expect("not an empty sequence");
        let std_dev_sec = sec
            .clone()
            .standard_deviation()
            .expect("not an empty sequence");

        BenchStats {
            mean_sec,
            std_dev_sec,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchStats {
    pub mean_sec: f64,
    pub std_dev_sec: f64,
}
