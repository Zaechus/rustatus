use std::fs;

use crate::{block, colored_block, RED};

const TEMP_ICON: char = '\u{f2c7}';

pub fn temperature_block() -> String {
    let temperature = temperature();

    if temperature > 80 {
        colored_block(&format!("{TEMP_ICON} {}°C", temperature), RED)
    } else {
        block(&format!("{TEMP_ICON} {}°C", temperature))
    }
}

fn temperature() -> u32 {
    fs::read_dir("/sys/class/thermal")
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .filter(|x| x.starts_with("thermal_zone"))
        .map(|x| {
            fs::read_to_string(format!("/sys/class/thermal/{}/temp", x))
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap()
                / 1000
        })
        .max()
        .unwrap()
}
