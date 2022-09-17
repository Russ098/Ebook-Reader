extern crate core;

use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;
use data::Delegate;

mod view;

use view::build_ui;
use crate::data::Ebook;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Ebook Reader")
        .window_size((1100., 600.));

    let ebook = Ebook::new();
    let initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}