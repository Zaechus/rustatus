mod battery;
mod clock;
mod cpu;
mod mem;
mod temperature;

pub use battery::battery_block;
pub use clock::*;
pub use cpu::*;
pub use mem::*;
pub use temperature::temperature_block;

const RED: &str = "#cc241d";

pub fn init() {
    // println!("{{\"version\": 1, \"click_events\": true}}\n[");
    println!("{{\"version\": 1}}\n[");
}

pub fn block(text: &str) -> String {
    format!("{{\"full_text\":\" {text} \",\"separator\":false}},")
}

pub fn colored_block(text: &str, color: &str) -> String {
    format!("{{\"full_text\":\" {text} \",\"background\":\"{color}\",\"separator\":false}},")
}
