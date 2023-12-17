use sysinfo::{System, SystemExt, ComponentExt, Component, CpuExt, Cpu};

pub struct ComponentTemperature {
  pub label: String,
  pub celsius_temp: f32,
}

pub struct CpuInfo {
  pub number: usize,
  pub frequency: u64,
  pub usage: f32,
}

pub fn get_components_temp() -> Vec<ComponentTemperature> {
  let mut sys = System::new_all();
  sys.refresh_components();

  extract_components_temp(sys.components())
}

pub fn get_cpu_info() -> Vec<CpuInfo> {
  let mut sys = System::new_all();
  sys.refresh_cpu();

  extract_cpu_info(sys.cpus())
}

fn extract_components_temp(components: &[Component]) -> Vec<ComponentTemperature> {
  let mut temps: Vec<ComponentTemperature> = Vec::new();

  for component in components {
    temps.push(ComponentTemperature{
      label: component.label().to_string(),
      celsius_temp: component.temperature(),
    })
  }

  temps
}

fn extract_cpu_info(cpus: &[Cpu]) -> Vec<CpuInfo> {
  let mut cpu_info: Vec<CpuInfo> = Vec::new();

  for (index, cpu) in cpus.iter().enumerate() {
    cpu_info.push(CpuInfo { number: index + 1, frequency: cpu.frequency(), usage: cpu.cpu_usage() })
  };

  cpu_info
}