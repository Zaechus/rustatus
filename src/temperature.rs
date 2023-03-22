use std::{fs, io};

pub fn temperature() -> io::Result<u32> {
    Ok(fs::read_dir("/sys/class/thermal")?
        .map(|x| {
            if let Ok(x) = x.unwrap().file_name().into_string() {
                if x.starts_with("thermal_zone") {
                    fs::read_to_string(format!("/sys/class/thermal/{}/temp", x))
                        .unwrap()
                        .trim()
                        .parse::<u32>()
                        .unwrap()
                        / 1000
                } else {
                    0
                }
            } else {
                0
            }
        })
        .max()
        .unwrap())
}
