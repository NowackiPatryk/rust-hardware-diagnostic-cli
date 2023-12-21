pub struct ComponentTemperature {
    pub label: String,
    pub celsius_temp: f32,
}

pub struct CpuInfo {
    pub number: usize,
    pub name: String,
    pub frequency: u64,
    pub usage: f32,
}
