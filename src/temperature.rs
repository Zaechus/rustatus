use std::fs;

use glob::glob;

pub fn temperature() -> u32 {
    glob("/sys/class/thermal/thermal_zone*/temp")
        .unwrap()
        .map(|entry| {
            if let Ok(path) = entry {
                fs::read_to_string(path)
                    .unwrap()
                    .trim()
                    .parse::<u32>()
                    .unwrap()
                    / 1000
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0)
}
