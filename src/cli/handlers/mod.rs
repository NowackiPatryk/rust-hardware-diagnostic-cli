mod utils;
use crate::sysinfo::{get_cpu_info, get_components_temp, ComponentTemperature, CpuInfo};

fn format_temperature_message(temp: ComponentTemperature) -> String {
  format!("{} : {:.2}°C", temp.label, temp.celsius_temp)
}

fn format_cpu_info_message(cpu_info: CpuInfo) -> String {
  format!("#{} - Frequency: {}HZ, Usage: {:.1}%", cpu_info.number, cpu_info.frequency, cpu_info.usage)
}

pub fn display_components_temperatures_live() {
  loop {
    utils::clear_screen();
    utils::wait(500);

    let result = get_components_temp();

    for temp in result {
      println!("{}", format_temperature_message(temp));
    }
  }
}

pub fn display_cpu_info_live() {
  loop {
    utils::clear_screen();
    utils::wait(500);

    let result = get_cpu_info();

    for cpu_info in result {
      println!("{}", format_cpu_info_message(cpu_info));
    }
  }
}