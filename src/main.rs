use std::{thread, time::Duration};

mod feed_tui;

macro_rules! timeout {
    ($dur:expr) => {
        thread::sleep(Duration::from_millis($dur))
    };
}   

fn main() {
    feed_tui::tui::render().expect("failed to render feed to the terminal");
    timeout!(5000);
}