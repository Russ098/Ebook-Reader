use druid::{AppLauncher, WindowDesc, Point};

mod data;
use data::AppState;

mod view;

use view::build_ui;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Ebook Reader")
        .window_size((1100., 600.));


    let initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}