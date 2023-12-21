mod utils;
mod structs;

use std::collections::HashMap;
use crate::{sysinfo::{get_cpu_info, get_components_temp, systembenchmark::benchmark_cpu}, cli::handlers::structs::Displayable};

fn get_initial_min_max_temps() -> HashMap<String, structs::DisplayableComponentTemps> {
  let mut min_max_temps:  HashMap<String, structs::DisplayableComponentTemps> = HashMap::new();

  let result = get_components_temp();

  for temp in result {
    let label = temp.label.clone();

    min_max_temps.insert(temp.label, structs::DisplayableComponentTemps {
      label: label,
      current_temp: temp.celsius_temp.clone(),
      max_temp: temp.celsius_temp.clone(),
      min_temp: temp.celsius_temp.clone(),
    });
  }

  min_max_temps
}

pub fn display_components_temperatures_live() {
  let mut min_max_temps: HashMap<String, structs::DisplayableComponentTemps> = get_initial_min_max_temps();

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

      println!("{}", temps.get_display_string());
    };
  }
}

pub fn display_cpu_info_live() {
  loop {
    utils::clear_screen();
    utils::wait(500);

    let result = get_cpu_info();

    for cpu_info in result {
      let displayable_cpu_info = structs::DisplayableCpuInfo {
        name: cpu_info.name,
        number: cpu_info.number,
        frequency: cpu_info.frequency,
        usage: cpu_info.usage,
      };

      println!("{}", displayable_cpu_info.get_display_string());
    }
  }
}

pub fn cpu_benchmark() {
  println!("Benchmark started...");

  const BENCHMARKS_NUMBER: u128 = 100;
  let mut current_benchmark: u128 = 1;
  let mut final_score: u128 = 0;

  while current_benchmark <= BENCHMARKS_NUMBER {
    final_score += benchmark_cpu() / BENCHMARKS_NUMBER;
    current_benchmark += 1;
  }

  println!("Benchmark ended. Your average score is: {}", final_score);
}