mod handlers;
use handlers::{display_components_temperatures_live, display_cpu_info_live};

const TEMP_COMMAND: &str = "temp";
const CPU_INFO_COMMAND: &str = "cpuinfo";

pub fn handle(command: &str) {
  match command {
    TEMP_COMMAND => display_components_temperatures_live(),
    CPU_INFO_COMMAND => display_cpu_info_live(),
    _=> eprintln!("Command not recognized"),
}
}