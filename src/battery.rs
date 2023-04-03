use std::fs;

use crate::{block, colored_block, RED};

pub fn battery_block() -> String {
    if let Ok(capacity) = fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
        let capacity = capacity.trim().parse::<u8>().unwrap();

        let icon = match (capacity, status()) {
            (_, "Charging") => '\u{f1e6}',
            (94.., "Full" | "Not charging") => return block("\u{f240} "),
            (90.., "Discharging") => '\u{f240}',
            (66.., "Discharging") => '\u{f241}',
            (33.., "Discharging") => '\u{f242}',
            (10.., "Discharging") => '\u{f243}',
            (0.., "Discharging") => return colored_block(&format!("\u{f244}  {capacity}%"), RED),
            _ => '?',
        };

        block(&format!("{icon}  {capacity}%"))
    } else {
        String::new()
    }
}

fn status<'a>() -> &'a str {
    match fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .unwrap()
        .trim()
    {
        "Charging" => "Charging",
        "Discharging" => "Discharging",
        "Full" => "Full",
        "Not charging" => "Not charging",
        _ => "",
    }
}
