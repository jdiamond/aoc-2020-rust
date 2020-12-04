#[cfg(test)]
pub fn load_input(name: &str) -> String {
  std::fs::read_to_string(format!("{}/inputs/{}", env!("CARGO_MANIFEST_DIR"), name)).unwrap()
}
