use std::collections::HashMap;

use anyhow::Result;
use get_size::GetSize;
use serde::Serialize;
use solidity_testing_perf::TestContractKind;

type Benchmarks = HashMap<String, Measures>;

type Measures = HashMap<MeasureName, Measure>;

#[derive(Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum MeasureName {
    HeapSize,
}

#[derive(Serialize)]
struct Measure {
    value: f64,
}

pub fn main() -> Result<()> {
    let mut benchmarks = Benchmarks::new();
    collect_erc721_cst_size(&mut benchmarks)?;

    let json = serde_json::to_string_pretty(&benchmarks)?;
    println!("{json}");

    Ok(())
}

fn collect_erc721_cst_size(benchmarks: &mut Benchmarks) -> Result<()> {
    let node = TestContractKind::ERC721_v5_0_0.load()?.parse()?;

    let mut measures = Measures::new();

    measures.insert(
        MeasureName::HeapSize,
        Measure {
            value: node.get_heap_size(),
        },
    );

    benchmarks.insert("erc721-cst-size".to_string(), measures);

    Ok(())
}
