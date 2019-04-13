#[cfg(target_os = "macos")]
pub fn main_symbol() -> &'static str {
  "_main"
}

#[cfg(target_os = "linux")]
pub fn main_symbol() -> &'static str {
  "main"
}

