use std::{fs, thread, time::Duration};

use time::{macros::format_description, OffsetDateTime};

fn block(text: &str) -> String {
    format!("{{\"full_text\":\" {} \"}},", text)
}

fn battery_status() -> String {
    match fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .unwrap()
        .trim()
    {
        "Charging" => "+".to_owned(),
        "Discharging" => "-".to_owned(),
        _ => String::new(),
    }
}

fn main() {
    let f = format_description!(
        version = 2,
        "[weekday repr:short] [year]-[month]-[day] [hour]:[minute]"
    );

    println!("{}\n[", r#"{"version": 1, "click_events": true}"#);
    loop {
        println!(
            "[{}{}],",
            block(&format!(
                "{} {}%",
                battery_status(),
                fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
                    .unwrap()
                    .trim()
            )),
            block(&OffsetDateTime::now_local().unwrap().format(f).unwrap())
        );
        thread::sleep(Duration::from_millis(4000));
    }
}
