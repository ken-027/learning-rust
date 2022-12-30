pub fn convert_temp_f_c(fahrenheit: u64) -> f64 {
  let celcuis: f64 = fahrenheit as f64 - 32.0;
  celcuis * (5.0/9.0)
}