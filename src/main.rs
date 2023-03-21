use std::{thread, time::Duration};

use time::{macros::format_description, OffsetDateTime};

use rustbar::*;

fn main() {
    let date_format = format_description!(
        version = 2,
        "[weekday repr:short] [year]-[month]-[day] [hour]:[minute]"
    );

    println!("{}\n[", r#"{"version": 1, "click_events": true}"#);
    loop {
        println!(
            "[{}{}{}{}{}],",
            block(&format!("{}{:.1}GHz", boost(), freq().unwrap())),
            block(&format!("{}°C", temperature())),
            block(&memory().unwrap()),
            block(&format!("{}{}%", battery_status(), battery_capacity())),
            block(
                &OffsetDateTime::now_local()
                    .unwrap()
                    .format(date_format)
                    .unwrap()
            )
        );
        thread::sleep(Duration::from_millis(4000));
    }
}
