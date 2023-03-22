use std::fs;

use crate::{block, BG, GREEN, RED, YELLOW};

pub fn battery_block() -> String {
    let capacity = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .unwrap()
        .trim()
        .parse::<u8>()
        .unwrap();

    let color = match capacity {
        60..=99 => GREEN,
        30..=59 => YELLOW,
        0..=29 => RED,
        _ => BG,
    };

    block(&format!("{}{}%", status(), capacity), color)
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
