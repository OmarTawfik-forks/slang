#![allow(clippy::exit)]

use std::hint::black_box;

use iai_callgrind::{library_benchmark, library_benchmark_group, main, FlamegraphConfig, LibraryBenchmarkConfig};
use solidity_testing_perf::{TestContract, TestContractKind};

// WARNING:
// The reported `iai` benchmark ID is constructed from:
// `{file_name}::{group_name}::{function_name}::{bench_name}_{test_case_index}:{input_expression}`
// Example: `iai::iai::parse contract_0:& erc721()`
// Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.

// Setup function: not included in `iai` calculations:
fn erc721() -> TestContract {
    TestContractKind::ERC721_v5_0_0.load().unwrap()
}

// Runner function: included in `iai` calculations:
#[library_benchmark]
#[benches::contract(&erc721())]
fn parse(contract: &TestContract) {
    black_box(contract.parse()).unwrap();
}

library_benchmark_group!(
    name = iai;
    benchmarks = parse
);

main!(
    config = LibraryBenchmarkConfig::default().flamegraph(FlamegraphConfig::default());
    library_benchmark_groups = iai
);
