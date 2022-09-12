use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;

mod view;
mod controller;
mod delegate;
use delegate::Delegate;

use view::build_ui;
use crate::data::TodoItem;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Todo Tutorial")
        .window_size((1930.0, 1000.0));

    let todos = vec![TodoItem::new("thing one")];
    let initial_state = AppState::new(todos);

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}