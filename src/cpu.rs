use std::{fs, io};

pub fn boost() -> String {
    match fs::read_to_string("/sys/devices/system/cpu/cpufreq/boost")
        .unwrap()
        .trim()
        .parse::<u8>()
        .unwrap()
    {
        0 => String::new(),
        _ => "*".to_owned(),
    }
}

pub fn freq() -> io::Result<f64> {
    let cpus = fs::read_to_string("/sys/devices/system/cpu/online")?.trim()[2..]
        .parse::<u8>()
        .unwrap();

    let mut cur = 0;

    for cpu in 0..cpus {
        cur += fs::read_to_string(format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq",
            cpu
        ))?
        .trim()
        .parse::<u32>()
        .unwrap();
    }

    cur /= cpus as u32;

    Ok(cur as f64 / 1000000.0)
}
