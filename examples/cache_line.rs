//! # Prerequisites
//!
//! - `cargo install --git https://github.com/Banyc/dfplot.git`
//!
//! # What to observe
//!
//! - Decide the cache line size by observing the scatter plot.

use std::num::NonZeroUsize;

use bench::{Bencher, BencherConfig};
use serde::Serialize;
use xshell::{cmd, Shell};

fn main() {
    let config = BencherConfig {
        warm_ups: 2,
        batch_size: NonZeroUsize::new(2).unwrap(),
        samples: NonZeroUsize::new(2).unwrap(),
    };
    let bencher = Bencher::new(config);

    const CSV_OUTPUT_PATH: &str = "target/cache_line.csv";
    let csv_output_file = std::fs::File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(CSV_OUTPUT_PATH)
        .unwrap();
    let mut csv_writer = csv::WriterBuilder::new().from_writer(csv_output_file);

    const ARRAY_SIZE: usize = 2 << 26;
    let mut array = vec![0; ARRAY_SIZE];

    let mut step_size = 1;
    while step_size <= 2048 {
        let stats = bencher.iter(|| iter_incr(&mut array, step_size));
        let operations = array.len() / step_size;
        let amortized_mean = stats.mean_sec / operations as f64;
        let stats = Stats {
            step_size,
            amortized_mean_dur_sec: amortized_mean,
        };
        csv_writer.serialize(&stats).unwrap();
        step_size *= 2;
    }

    drop(csv_writer);
    let sh = Shell::new().unwrap();
    cmd!(
        sh,
        "dfplot scatter -x step_size -y amortized_mean_dur_sec {CSV_OUTPUT_PATH}"
    )
    .run()
    .unwrap();
}

fn iter_incr(array: &mut [u8], step: usize) {
    for i in (0..array.len()).step_by(step) {
        array[i] = array[i].wrapping_add(1);
    }
}

#[derive(Debug, Clone, Serialize)]
struct Stats {
    pub step_size: usize,
    pub amortized_mean_dur_sec: f64,
}
