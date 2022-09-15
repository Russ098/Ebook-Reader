use druid::{widget::{Flex}, Widget, WidgetExt, Color, UnitPoint, FileDialogOptions, FileSpec};

use crate::data::*;
use druid::widget::{TextBox, Button, RawLabel, Scroll, SizedBox};

fn option_row() -> impl Widget<AppState> {

    let rs = FileSpec::new("Rust source", &["rs"]);
    let txt = FileSpec::new("Text file", &["txt"]);
    let other = FileSpec::new("Bogus file", &["foo", "bar", "baz"]);
    let default_save_name = String::from("MyFile.txt");

    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![rs, txt, other])
        .default_type(txt)
        .default_name(default_save_name)
        .name_label("Target")
        .title("Choose a target for this lovely file")
        .button_text("Export");

    let open_dialog_options = save_dialog_options
        .clone()
        .default_name("MySavedFile.txt")
        .name_label("Source")
        .title("Where did you put that file?")
        .button_text("Import");

    let open_button = Button::new("Open").padding(5.0).on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });
    let edit_button = Button::new("Edit").padding(5.0).on_click(AppState::click_edit_button);
    let save_button = Button::new("Save").padding(5.0).on_click(AppState::click_save_button);
    Flex::row()
        .with_child(open_button)
        .with_child(edit_button)
        .with_child(save_button)
        .align_left()
        .background(Color::WHITE)
        .border(Color::GRAY,0.5)
}
fn settings_row() -> impl Widget<AppState> {
    let single_page_button = Button::new("Single Page").padding(5.0).on_click(AppState::click_single_page_button);
    let double_page_button = Button::new("Double Page").padding(5.0).on_click(AppState::click_double_page_button);
    let plus_button = Button::new("+").padding(5.0).on_click(AppState::click_plus_button);
    let min_button = Button::new("-").padding(5.0).on_click(AppState::click_min_button);
    let edit_size_text = TextBox::new()
        .with_placeholder("50")
        .lens(AppState::font_size);
    Flex::row()
        .with_child(single_page_button)
        .with_child(double_page_button)
        .with_child(min_button)
        .with_child(edit_size_text)
        .with_child(plus_button)
        .align_right()
        .align_vertical(UnitPoint::BOTTOM)
        .background(Color::WHITE)
        .border(Color::GRAY,0.5)
}

pub fn build_ui() -> impl Widget<AppState> {
    let mut c = Flex::column();
    c.add_child(option_row());
    c.add_flex_child(SizedBox::new(Scroll::new(RawLabel::new().lens(AppState::rich_text))).expand_height(),1.0);
    c.add_child(settings_row());
    return c;
}



