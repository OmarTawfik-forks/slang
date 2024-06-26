#![allow(clippy::exit)]

use std::hint::black_box;

use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use solidity_testing_perf::{TestContract, TestContractKind};

// WARNING:
// The reported `iai` benchmark name is constructed from:
// `{file_name}::{group_name}::{function_name}::{bench_name}_{test_case_index}:{input_expression}`
// Example: `iai::iai::parse contract_0:& load_erc721()`
// Changing any of the above would change the resulting report, and de-link it from previous results.

fn load_erc721() -> TestContract {
    TestContractKind::ERC721_v5_0_0.load().unwrap()
}

#[library_benchmark]
#[benches::contract(&load_erc721())]
fn parse(contract: &TestContract) {
    black_box(contract.parse()).unwrap();
}

library_benchmark_group!(
    name = iai;
    benchmarks = parse
);

main!(library_benchmark_groups = iai);
