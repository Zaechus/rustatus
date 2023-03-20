use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn memory() -> io::Result<String> {
    let mut used = 0;

    for line in BufReader::new(File::open("/proc/meminfo")?).lines() {
        let line = line?;
        let line: Vec<_> = line.split_whitespace().collect();
        match line[0] {
            "MemTotal:" => {
                used = line[1].parse::<i32>().unwrap_or(0);
            }
            "MemFree:" | "Buffers:" | "Cached:" => {
                used -= line[1].parse::<i32>().unwrap_or(0);
            }
            "Shmem:" => {
                used += line[1].parse::<i32>().unwrap_or(0);
            }
            "SReclaimable:" => {
                used -= line[1].parse::<i32>().unwrap_or(0);
                break;
            }
            _ => (),
        }
    }

    used /= 1024;

    Ok(if used >= 1000 {
        format!("{:.1}G", (used as f64 / 1024.0))
    } else {
        format!("{}M", used)
    })
}
