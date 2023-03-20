use std::fs;

pub fn battery_capacity() -> String {
    fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .unwrap()
        .trim()
        .to_owned()
}

pub fn battery_status() -> String {
    match fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .unwrap()
        .trim()
    {
        "Charging" => "+".to_owned(),
        "Discharging" => "-".to_owned(),
        _ => String::new(),
    }
}
