use sysinfo::{System, SystemExt, ComponentExt, Component, CpuExt, Cpu};
pub mod structs;
pub mod systembenchmark;

pub fn get_components_temp() -> Vec<structs::ComponentTemperature> {
  let mut sys = System::new_all();
  sys.refresh_components();

  extract_components_temp(sys.components())
}

pub fn get_cpu_info() -> Vec<structs::CpuInfo> {
  let mut sys = System::new_all();
  sys.refresh_cpu();

  extract_cpu_info(sys.cpus())
}

fn extract_components_temp(components: &[Component]) -> Vec<structs::ComponentTemperature> {
  let mut temps: Vec<structs::ComponentTemperature> = Vec::new();

  for component in components {
    temps.push(structs::ComponentTemperature{
      label: component.label().to_string(),
      celsius_temp: component.temperature(),
    })
  }

  temps
}

fn extract_cpu_info(cpus: &[Cpu]) -> Vec<structs::CpuInfo> {
  let mut cpu_info: Vec<structs::CpuInfo> = Vec::new();

  for (index, cpu) in cpus.iter().enumerate() {
    cpu_info.push(structs::CpuInfo { number: index + 1, frequency: cpu.frequency(), usage: cpu.cpu_usage() })
  };

  cpu_info
}