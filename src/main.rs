extern crate core;

use druid::{AppLauncher, WindowDesc};

mod data;

use data::AppState;
use data::Delegate;

mod view;

use view::build_ui;


pub fn main() {
    let initial_state = AppState::new();
    let main_window = WindowDesc::new(build_ui)
        .title("Ebook Reader")
        .window_size((1100., 600.));

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}
