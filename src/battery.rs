use std::fs;

use crate::colored_block;

pub fn battery_block() -> String {
    if let Ok(capacity) = fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
        let capacity = capacity.trim().parse::<u8>().unwrap();

        let (icon, color) = match (capacity, status()) {
            (_, "Charging") => ('\u{f1e6}', ""),
            (90.., _) => ('\u{f240}', ""),
            (66.., _) => ('\u{f241}', ""),
            (33.., _) => ('\u{f242}', ""),
            (10.., _) => ('\u{f243}', ""),
            (0.., _) => ('\u{f244}', "#cc241d"),
        };

        colored_block(&format!(" {icon}  {capacity}%"), color)
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
        _ => "",
    }
}
