use std::fs;

use crate::{block, colored_block, RED};

pub fn battery_block() -> String {
    for bat in ["BAT0", "BAT1"] {
        if let Ok(capacity) =
            fs::read_to_string(format!("/sys/class/power_supply/{}/capacity", bat))
        {
            let capacity = capacity.trim().parse::<u8>().unwrap();

            let icon = match (capacity, status(bat)) {
                (_, "Charging") => '\u{f1e6}',
                (94.., "Full" | "Not charging") => return block("\u{f240} "),
                (90.., "Discharging") => '\u{f240}',
                (66.., "Discharging") => '\u{f241}',
                (33.., "Discharging") => '\u{f242}',
                (10.., "Discharging") => '\u{f243}',
                (0.., "Discharging") => {
                    return colored_block(&format!("\u{f244}  {capacity}%"), RED)
                }
                _ => '?',
            };

            return block(&format!("{icon}  {capacity}%"));
        }
    }
    String::new()
}

fn status<'a>(bat: &str) -> &'a str {
    match fs::read_to_string(format!("/sys/class/power_supply/{}/status", bat))
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
