use druid::{widget::{Checkbox, Flex, Label, List}, Widget, WidgetExt, EventCtx, Env, Color, ArcStr};

use crate::data::*;
use druid::widget::{TextBox, Button, Container, LabelText, RawLabel};
use crate::controller::TodoItemController;
use druid::text::RichText;
use std::convert::TryFrom;

fn todo_item() -> impl Widget<TodoItem> {
    let open_button = Button::new("Open").padding(5.0);
    let edit_button = Button::new("Edit").padding(5.0);
    let save_button = Button::new("Save").padding(5.0);


    Flex::row()

        .with_child(open_button)
        .with_child(edit_button)
        .with_child(save_button)
        .align_left()
        .background(Color::WHITE)
        .border(Color::GRAY,0.5)
        .controller(TodoItemController)

}

pub fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(List::new(todo_item).lens(AppState::todos))
        .with_child(RawLabel::new().lens(AppState::rich_text))
}

//fn new_todo_textbox() -> impl Widget<AppState> {
    //let new_todo_textbox = TextBox::new()
        //.with_placeholder("Add a new todo")
        //.expand_width()
        //.lens(AppState::new_todo);
    //let add_todo_button = Button::new("Add")
        //.on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| data.add_todo());
    //Flex::row()
        //.with_flex_child(new_todo_textbox, 1.)
        //.with_child(add_todo_button)
//}


