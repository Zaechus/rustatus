use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn memory() -> io::Result<String> {
    let mut used = 0;

    for line in BufReader::new(File::open("/sys/devices/system/node/node0/meminfo")?).lines() {
        let line = line?;
        let line: Vec<_> = line.split_whitespace().collect();
        match line[2] {
            "MemTotal:" => {
                used = line[3].parse::<u32>().unwrap_or(0);
            }
            "MemFree:" | "FilePages:" => {
                used -= line[3].parse::<u32>().unwrap_or(0);
            }
            "Shmem:" => {
                used += line[3].parse::<u32>().unwrap_or(0);
            }
            "SReclaimable:" => {
                used -= line[3].parse::<u32>().unwrap_or(0);
                break;
            }
            _ => (),
        }
    }

    used /= 1024;

    Ok(if used >= 1000 {
        format!("{:.1}GiB", (used as f64 / 1024.0))
    } else {
        format!("{}MiB", used)
    })
}
