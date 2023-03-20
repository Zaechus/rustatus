mod battery;
mod mem;

pub use battery::battery_status;
pub use mem::memory;

pub fn block(text: &str) -> String {
    format!("{{\"full_text\":\" {} \"}},", text)
}
