mod battery;
mod cpu;
mod mem;
mod temperature;

pub use battery::battery_block;
pub use cpu::*;
pub use mem::*;
pub use temperature::*;

pub const BG: &str = "#282828";
pub const RED: &str = "#cc241d";
pub const GREEN: &str = "#98971a";
pub const YELLOW: &str = "#d79921";

pub fn block(text: &str, color: &str) -> String {
    format!("{{\"full_text\":\" {text} \",\"background\":\"{color}\",\"separator\":false}},",)
}
