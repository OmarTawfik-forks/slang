//! WARNING:
//! The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
//! For the function below: `iai::benchmarks::list_contracts`
//! Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.

#![allow(clippy::exit)]
#![allow(clippy::unit_arg)]

mod dataset;
mod testcases;

use std::hint::black_box;

use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Direction, FlamegraphConfig,
    LibraryBenchmarkConfig,
};

#[library_benchmark]
fn list_contracts() {
    black_box(testcases::list_contracts::run());
}

library_benchmark_group!(
    name = benchmarks;

    benchmarks = list_contracts
);

main!(
    config = LibraryBenchmarkConfig::default()
        .env_clear(false)
        .flamegraph(FlamegraphConfig::default().direction(Direction::BottomToTop));

    library_benchmark_groups = benchmarks
);
