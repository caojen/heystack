use super::options;

pub fn deal_with_options(option: options::Options) {
  if option.unknown.len() != 0 {
    crate::log!["a"];
  }
}