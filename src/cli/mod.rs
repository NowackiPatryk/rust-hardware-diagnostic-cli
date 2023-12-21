mod handlers;
use handlers::{cpu_benchmark, display_components_temperatures_live, display_cpu_info_live};

const TEMP_COMMAND: &str = "temp";
const CPU_INFO_COMMAND: &str = "cpuinfo";
const CPU_BENCHMARK_COMMAND: &str = "cpubenchmark";

pub fn handle(command: &str) {
    match command {
        TEMP_COMMAND => display_components_temperatures_live(),
        CPU_INFO_COMMAND => display_cpu_info_live(),
        CPU_BENCHMARK_COMMAND => cpu_benchmark(),
        _ => eprintln!("Command not recognized"),
    }
}
