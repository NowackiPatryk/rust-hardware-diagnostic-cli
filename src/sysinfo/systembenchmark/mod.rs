use std::time::SystemTime;

pub fn benchmark_cpu() -> u128 {
  const FULL_SCORE: u128 = 1000;
  const EPOCHS: i32 = 10000000;

  let mut number_to_square: f32 = 123456.789;
  let mut current_epoch = 1;

  let benchmark_start = SystemTime::now();
  while current_epoch <= EPOCHS {
    number_to_square = number_to_square.sqrt();
    current_epoch += 1;
  }
  let benchmark_end = SystemTime::now();

  let duration = benchmark_end.duration_since(benchmark_start).unwrap().as_millis();

  FULL_SCORE - duration
}

