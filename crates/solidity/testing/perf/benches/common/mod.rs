use infra_utils::perf::{Benchmark, Measurement, MeasurementsDir, Report};
use semver::Version;
use sysinfo::{get_current_pid, System};

pub const SOLC_VERSION: Version = Version::new(0, 8, 20);

pub const SOURCES: [&str; 3] = [
    include_str!("../../dataset/ERC20.sol"),
    include_str!("../../dataset/ERC721.sol"),
    include_str!("../../dataset/Governor.sol"),
];

pub fn report_process_memory(benchmark_name: &str) -> Result<()> {
    let process_memory = System::new_all()
        .process(get_current_pid()?)
        .unwrap()
        .memory();

    let measurement = Measurement::new(process_memory);

    let benchmark = Benchmark::new();
    benchmark.insert("Process Memory".to_owned(), measurement);

    let report = Report::new();
    report.add_measurement(benchmark_name, benchmark);

    let dir = MeasurementsDir::load_existing()?;
    dir.add_report(&report)?;

    Ok(())
}
