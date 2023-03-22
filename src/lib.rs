mod battery;
mod config;
mod cpu;
mod mem;
mod temperature;

pub use battery::battery_block;
pub use config::Config;
pub use cpu::*;
pub use mem::*;
pub use temperature::*;

pub fn block(text: &str) -> String {
    colored_block(text, "")
}

pub fn colored_block(text: &str, color: &str) -> String {
    if color.is_empty() {
        format!("{{\"full_text\":\" {text} \",\"separator\":false}},")
    } else {
        format!("{{\"full_text\":\" {text} \",\"background\":\"{color}\",\"separator\":false}},")
    }
}
