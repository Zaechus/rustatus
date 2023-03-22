use std::{io, thread, time::Duration};

use time::{macros::format_description, OffsetDateTime};

use rustbar::*;

fn main() -> io::Result<()> {
    let date_format = format_description!(
        version = 2,
        "[weekday repr:short] [year]-[month]-[day] [hour]:[minute]"
    );

    println!("{{\"version\": 1, \"click_events\": true}}\n[");
    loop {
        println!(
            "[{}{}{}{}{}],",
            block(&format!("{}{:.1}GHz", boost(), freq()?), BG),
            block(&format!("{}°C", temperature()?), BG),
            block(&memory()?, BG),
            battery_block(),
            block(
                &OffsetDateTime::now_local()
                    .unwrap()
                    .format(date_format)
                    .unwrap(),
                BG
            )
        );
        thread::sleep(Duration::from_millis(4000));
    }
}
