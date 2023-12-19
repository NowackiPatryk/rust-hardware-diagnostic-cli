mod utils;
use std::collections::HashMap;

use crate::sysinfo::{get_cpu_info, get_components_temp, CpuInfo};

#[derive(Clone)]
struct MinMaxComponentTemps {
  label: String,
  current_temp: f32,
  max_temp: f32,
  min_temp: f32,
}

fn format_temperature_message(temp: MinMaxComponentTemps) -> String {
  format!("{}: \n Current: {:.2}°C \n Min: {:.2}°C \n Max: {:.2}°C \n\n", temp.label, temp.current_temp, temp.min_temp, temp.max_temp)
}

fn format_cpu_info_message(cpu_info: CpuInfo) -> String {
  format!("#{} - Frequency: {}HZ, Usage: {:.1}%", cpu_info.number, cpu_info.frequency, cpu_info.usage)
}

fn get_initial_min_max_temps() -> HashMap<String, MinMaxComponentTemps> {
  let mut min_max_temps:  HashMap<String, MinMaxComponentTemps> = HashMap::new();

  let result = get_components_temp();

  for temp in result {
    let label = temp.label.clone();

    min_max_temps.insert(temp.label, MinMaxComponentTemps {
      label: label,
      current_temp: temp.celsius_temp.clone(),
      max_temp: temp.celsius_temp.clone(),
      min_temp: temp.celsius_temp.clone(),
    });
  }

  min_max_temps
}

pub fn display_components_temperatures_live() {
  let mut min_max_temps = get_initial_min_max_temps();

  loop {
    utils::clear_screen();
    utils::wait(500);

    let result = get_components_temp();

    for temp in result {
      let temps = min_max_temps.get_mut(temp.label.as_str()).unwrap();

      temps.current_temp = temp.celsius_temp;

      if temp.celsius_temp > temps.max_temp {
        temps.max_temp = temp.celsius_temp;
      }

      if temp.celsius_temp < temps.min_temp {
        temps.min_temp = temp.celsius_temp;
      }

      println!("{}", format_temperature_message(temps.clone()));
    };
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