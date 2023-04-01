use std::{thread, time::Duration};

use rustbar::*;

fn main() {
    rustbar::init();

    loop {
        println!(
            "[{}{}{}],",
            temperature_block(),
            battery_block(),
            block(&clock())
        );
        thread::sleep(Duration::from_millis(4000));
    }
}
