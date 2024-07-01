use infra_utils::perf::{Benchmark, Measurement, MeasurementsDir, Report};
use semver::Version;
use sysinfo::{get_current_pid, System};

pub const SOLC_VERSION: Version = Version::new(0, 8, 20);

pub const SOURCES: [&str; 3] = [
    include_str!("../../dataset/ERC20.sol"),
    include_str!("../../dataset/ERC721.sol"),
    include_str!("../../dataset/Governor.sol"),
];

pub fn report_process_memory(benchmark_name: &str) {
    let pid = get_current_pid().unwrap();

    let process_memory = System::new_all().process(pid).unwrap().memory();

    #[allow(clippy::cast_precision_loss)]
    let measurement = Measurement::new(process_memory as f64);

    let mut benchmark = Benchmark::new();
    benchmark.insert("Process Memory".to_owned(), measurement);

    let mut report = Report::new();
    report.insert(benchmark_name.to_owned(), benchmark);

    let dir = MeasurementsDir::load_existing().unwrap();
    dir.add_report(&report).unwrap();
}
