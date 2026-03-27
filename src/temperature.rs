use std::fs;

use crate::{block, colored_block, RED};

pub fn temperature_block() -> String {
    if let Some(temperature) = temperature() {
        if temperature > 80 {
            colored_block(&format!("{}°C", temperature), RED)
        } else {
            block(&format!("{}°C", temperature))
        }
    } else {
        String::new()
    }
}

fn temperature() -> Option<u32> {
    if let Ok(thermal_dir) = fs::read_dir("/sys/class/thermal") {
        thermal_dir
            .map(|x| x.unwrap().file_name().into_string().unwrap())
            .filter(|x| x.starts_with("thermal_zone"))
            .map(|x| {
                if let Ok(s) = fs::read_to_string(format!("/sys/class/thermal/{}/temp", x)) {
                    s.trim().parse::<u32>().unwrap_or_default() / 1000
                } else {
                    0
                }
            })
            .max()
    } else {
        None
    }
}
