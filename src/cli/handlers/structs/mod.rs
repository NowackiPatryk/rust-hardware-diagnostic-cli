pub trait Displayable {
  fn get_display_string(&self) -> String;
}

#[derive(Clone)]
pub struct DisplayableComponentTemps {
  pub label: String,
  pub current_temp: f32,
  pub max_temp: f32,
  pub min_temp: f32,
}

impl Displayable for DisplayableComponentTemps {
  fn get_display_string(&self) -> String {
    format!("{}: \n Current: {:.2}°C \n Min: {:.2}°C \n Max: {:.2}°C \n\n", self.label, self.current_temp, self.min_temp, self.max_temp)
  }
}

pub struct DisplayableCpuInfo {
  pub number: usize,
  pub frequency: u64,
  pub usage: f32,
}

impl Displayable for DisplayableCpuInfo {
  fn get_display_string(&self) -> String {
    format!("#{} - Frequency: {}HZ, Usage: {:.1}%", self.number, self.frequency, self.usage)
  }
}