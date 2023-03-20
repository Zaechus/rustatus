mod battery;
mod cpu;
mod mem;
mod temperature;

pub use battery::*;
pub use cpu::*;
pub use mem::*;
pub use temperature::*;

pub fn block(text: &str) -> String {
    format!("{{\"full_text\":\" {} \"}},", text)
}
