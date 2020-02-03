// use std::{default::Default, io};
use std::cell::RefCell;
use std::sync::Arc;

use cursive::{logger, Cursive};

mod app;
mod func;
mod ui;
mod utils;

use app::App;
use ui::images::render_images_table;

fn main() {
    let mut ui = Cursive::default();
    // let mut ui = Cursive::dummy();
    let app = Arc::new(RefCell::new(App::new(&mut ui)));

    // Setup logger.
    logger::init();

    ui.add_global_callback('~', cursive::Cursive::toggle_debug_console);
    ui.add_global_callback('q', |s| s.quit());

    // Show images.
    ui.set_user_data(app.clone());
    render_images_table(&mut ui);

    // Run.
    ui.run();
}
