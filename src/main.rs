use std::{io, thread, time::Duration};

use time::{macros::format_description, OffsetDateTime};

use rustbar::*;

fn main() -> io::Result<()> {
    let config = Config::new("#cc241d", "#98971a", "#d79921");
    // let config = Config::new(
    //     env::args().nth(1).unwrap(),
    //     env::args().nth(2).unwrap(),
    //     env::args().nth(3).unwrap(),
    // );

    let date_format = format_description!(
        version = 2,
        "[weekday repr:short] [year]-[month]-[day] [hour]:[minute]"
    );

    println!("{{\"version\": 1, \"click_events\": true}}\n[");
    loop {
        println!(
            "[{}{}{}{}{}],",
            block(&format!("{}{:.1}GHz", boost(), freq()?)),
            block(&format!("{}°C", temperature()?)),
            block(&memory()?),
            battery_block(&config),
            block(
                &OffsetDateTime::now_local()
                    .unwrap()
                    .format(date_format)
                    .unwrap(),
            )
        );
        thread::sleep(Duration::from_millis(4000));
    }
}
