use std::fs;

use crate::{colored_block, Config};

pub fn battery_block(config: &Config) -> String {
    let capacity = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .unwrap()
        .trim()
        .parse::<u8>()
        .unwrap();

    let color = match capacity {
        60..=99 => config.green(),
        30..=59 => config.yellow(),
        0..=29 => config.red(),
        _ => "",
    };

    colored_block(&format!("{}{}%", status(), capacity), color)
}

fn status() -> String {
    match fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .unwrap()
        .trim()
    {
        "Charging" => "+ ".to_owned(),
        "Discharging" => "- ".to_owned(),
        _ => String::new(),
    }
}
