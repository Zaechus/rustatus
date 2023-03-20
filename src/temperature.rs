use std::{fs, io};

// TODO: all thermal_zones
pub fn temperature() -> io::Result<u32> {
    Ok(fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?
        .trim()
        .parse::<u32>()
        .unwrap()
        / 1000)
}
